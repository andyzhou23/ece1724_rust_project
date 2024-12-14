mod view;
//use reqwasm::http::Request;
use std::collections::HashMap;
use yew::prelude::*;
// websocket
use futures_util::sink::SinkExt;
use futures_util::stream::SplitSink;
use futures_util::stream::StreamExt;
use gloo::net::websocket::{futures::WebSocket, Message};
use gloo::timers::callback::Interval;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
struct Group {
    id: String,
    name: String,
    join_code: Option<String>,
    members: Vec<Member>,
    chat_history: Vec<String>,
    is_owner: bool,
}

#[derive(Clone, PartialEq)]
struct Member {
    name: String,
    status: String, // e.g., "online", "offline"
}

struct ChatApp {
    groups: Vec<Group>,
    join_codes: HashMap<String, String>,
    selected_group: Option<usize>,
    current_page: Page,
    error_message: Option<String>,
    logged_in: bool, // New field to track login state
    token: Option<String>,
    //registered_users: HashMap<String, String>,

    // websocket
    ws_write: Option<Rc<RefCell<SplitSink<WebSocket, Message>>>>,
    ws_ping_interval: Option<Interval>,
}

#[derive(Debug, PartialEq)]
enum Page {
    LoginPage,
    MainPage,
    NewGroupPage,
    RegistrationPage,
}

enum ChatAppMsg {
    NavigateTo(Page),
    CreateGroup(String),
    DeleteGroup(String),
    SelectGroup(usize),
    SendMessage(String),
    Login(String, String),
    LoginResponse(Result<(String, String, String), String>),
    Logout,
    Register(String, String), // Registration with username and password
    RegisterResponse(Result<String, String>), // Handle API response
    JoinGroup(String),
    CreateGroupResponse(Result<(String, String, String), String>),
    JoinGroupResponse(Result<(String, String, String), String>),
    DeleteGroupResponse(Result<String, String>),
    // websocket
    ConnectWebSocket,
    DisconnectWebSocket,
    WebSocketMessageReceived(String),
    SendWebSocketMessage(String),
}

impl Component for ChatApp {
    type Message = ChatAppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            groups: vec![],
            join_codes: HashMap::new(),
            //registered_users: HashMap::new(),
            selected_group: None,
            current_page: Page::LoginPage, // Start with LoginPage
            error_message: None,
            logged_in: false,
            token: None,

            // websocket
            ws_write: None,
            ws_ping_interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatAppMsg::NavigateTo(page) => {
                self.current_page = page;
                self.error_message = None;
                true
            }
            ChatAppMsg::CreateGroup(name) => {
                if name.is_empty() {
                    self.error_message = Some("Group name cannot be empty.".to_string());
                    return true;
                }

                let token = self.token.clone(); // Use auth_token from LoginResponse
                let link = ctx.link().clone();

                // Create the JSON body
                let body = serde_json::json!({ "name": name }).to_string();

                // Send the HTTP request
                let request =
                    reqwasm::http::Request::post("http://localhost:8081/api/group/create")
                        .header("Content-Type", "application/json")
                        .header(
                            "Authorization",
                            &format!("Bearer {}", token.unwrap_or_default()),
                        )
                        .body(body)
                        .send();

                // Handle the HTTP response
                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            if status == 200 {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let group_id = json["group_id"]
                                            .as_i64()
                                            .unwrap_or_default()
                                            .to_string();
                                        let group_name =
                                            json["group_name"].as_str().unwrap_or("").to_string();
                                        let group_code =
                                            json["group_code"].as_str().unwrap_or("").to_string();

                                        // Send a message to update the UI
                                        link.send_message(ChatAppMsg::CreateGroupResponse(Ok((
                                            group_id, group_name, group_code,
                                        ))));
                                    }
                                    Err(_) => {
                                        link.send_message(ChatAppMsg::CreateGroupResponse(Err(
                                            "Failed to parse response".to_string(),
                                        )));
                                    }
                                }
                            } else {
                                link.send_message(ChatAppMsg::CreateGroupResponse(Err(
                                    "Server error".to_string(),
                                )));
                            }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::CreateGroupResponse(Err(
                                "Network error".to_string(),
                            )));
                        }
                    }
                });

                self.error_message = None;
                true
            }
            ChatAppMsg::CreateGroupResponse(result) => {
                match result {
                    Ok((group_id, group_name, group_code)) => {
                        let new_group = Group {
                            id: group_id,
                            name: group_name.clone(),
                            join_code: Some(group_code),
                            members: vec![Member {
                                name: "Owner".to_string(),
                                status: "online".to_string(),
                            }],
                            chat_history: vec!["Welcome to the group!".to_string()],
                            is_owner: true,
                        };
                        // Add the new group to the list
                        self.groups.push(new_group);

                        // Navigate to the Main Page
                        self.current_page = Page::MainPage;
                        self.error_message = None;

                        log::info!("Group created and added to the list: {}", group_name);
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                true
            }

            ChatAppMsg::JoinGroup(join_code) => {
                if join_code.is_empty() {
                    self.error_message = Some("Join code cannot be empty.".to_string());
                    return true;
                }
                let token = self.token.clone();
                let link = ctx.link().clone();

                // Create the JSON body with the group code
                let body = serde_json::json!({ "group_code": join_code }).to_string();

                // Send the HTTP request
                let request = reqwasm::http::Request::post("http://localhost:8081/api/group/join")
                    .header("Content-Type", "application/json")
                    .header(
                        "Authorization",
                        &format!("Bearer {}", token.unwrap_or_default()),
                    )
                    .body(body)
                    .send();

                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            if status == 200 {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let group_id = json["group_id"]
                                            .as_i64()
                                            .unwrap_or_default()
                                            .to_string();
                                        let group_name =
                                            json["group_name"].as_str().unwrap_or("").to_string();
                                        let group_code =
                                            json["group_code"].as_str().unwrap_or("").to_string();

                                        // Send a message to update the UI
                                        link.send_message(ChatAppMsg::JoinGroupResponse(Ok((
                                            group_id, group_name, group_code,
                                        ))));
                                    }
                                    Err(_) => {
                                        link.send_message(ChatAppMsg::JoinGroupResponse(Err(
                                            "Failed to parse response".to_string(),
                                        )));
                                    }
                                }
                            } else {
                                if status == 400 {
                                    let error_msg =
                                        "Invalid group code or user already in group".to_string();
                                    log::error!("{}", error_msg);
                                    link.send_message(ChatAppMsg::JoinGroupResponse(Err(
                                        error_msg,
                                    )));
                                } else {
                                    match response.text().await {
                                        Ok(error_text) => {
                                            let error_msg =
                                                format!("Status {}: {}", status, error_text);
                                            log::error!("{}", error_msg);
                                            link.send_message(ChatAppMsg::JoinGroupResponse(Err(
                                                error_msg,
                                            )));
                                        }
                                        Err(e) => {
                                            let error_msg =
                                                format!("Failed to read error response: {}", e);
                                            log::error!("{}", error_msg);
                                            link.send_message(ChatAppMsg::JoinGroupResponse(Err(
                                                error_msg,
                                            )));
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::JoinGroupResponse(Err(
                                "Network error".to_string()
                            )));
                        }
                    }
                });

                self.error_message = None;
                true
            }

            ChatAppMsg::JoinGroupResponse(result) => {
                match result {
                    Ok((group_id, group_name, group_code)) => {
                        let new_group = Group {
                            id: group_id,
                            name: group_name.clone(),
                            join_code: Some(group_code),
                            members: vec![Member {
                                name: "Member".to_string(),
                                status: "online".to_string(),
                            }],
                            chat_history: vec!["You joined the group!".to_string()],
                            is_owner: false,
                        };
                        self.groups.push(new_group);

                        // Navigate to the Main Page
                        self.current_page = Page::MainPage;
                        self.error_message = None;
                        log::info!("Successfully joined group: {}", group_name);
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                true
            }

            // ChatAppMsg::DeleteGroup(group_id) => {
            //     // Find and remove the group with matching ID
            //     if let Some(index) = self.groups.iter().position(|g| g.id == group_id) {
            //         let group = &self.groups[index];
            //         // Remove the join code from the HashMap if it exists
            //         if let Some(join_code) = &group.join_code {
            //             self.join_codes.remove(join_code);
            //         }
            //         // Remove the group from the vector
            //         self.groups.remove(index);
            //         // Reset selected_group if the deleted group was selected
            //         if Some(index) == self.selected_group {
            //             self.selected_group = None;
            //         } else if let Some(selected) = self.selected_group {
            //             if selected > index {
            //                 // Adjust selected_group index if it was after the deleted group
            //                 self.selected_group = Some(selected - 1);
            //             }
            //         }
            //         log::info!("Group deleted: {}", group_id);
            //     }
            //     true
            // }
            ChatAppMsg::DeleteGroup(group_id) => {
                // First remove the group locally
                if let Some(index) = self.groups.iter().position(|g| g.id == group_id.clone()) {
                    let group = &self.groups[index];
                    // Remove the join code from the HashMap if it exists
                    if let Some(join_code) = &group.join_code {
                        self.join_codes.remove(join_code);
                    }
                    // Remove the group from the vector
                    self.groups.remove(index);
                    // Reset selected_group if the deleted group was selected
                    if Some(index) == self.selected_group {
                        self.selected_group = None;
                    } else if let Some(selected) = self.selected_group {
                        if selected > index {
                            // Adjust selected_group index if it was after the deleted group
                            self.selected_group = Some(selected - 1);
                        }
                    }
                }

                let token = self.token.clone();
                let link = ctx.link().clone();

                // Create the JSON body
                let body =
                    serde_json::json!({ "group_id": group_id.parse::<i64>().unwrap() }).to_string();

                // Send the HTTP request
                let request = reqwasm::http::Request::post("http://localhost:8081/api/group/leave")
                    .header("Content-Type", "application/json")
                    .header(
                        "Authorization",
                        &format!("Bearer {}", token.unwrap_or_default()),
                    )
                    .body(body)
                    .send();

                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            let json_response = response.json::<serde_json::Value>().await;
                            match json_response {
                                Ok(json) => {
                                    if status == 200 {
                                        link.send_message(ChatAppMsg::DeleteGroupResponse(Ok(
                                            json["message"].as_str().unwrap_or("").to_string(),
                                        )));
                                    } else {
                                        link.send_message(ChatAppMsg::DeleteGroupResponse(Err(
                                            json["error"]
                                                .as_str()
                                                .unwrap_or("Unknown error")
                                                .to_string(),
                                        )));
                                    }
                                }
                                Err(_) => {
                                    log::error!("Failed to parse response: {:?}", json_response);
                                    link.send_message(ChatAppMsg::DeleteGroupResponse(Err(
                                        format!("Failed to parse response: {:?}", json_response),
                                    )));
                                }
                            }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::DeleteGroupResponse(Err(
                                "Network error".to_string(),
                            )));
                        }
                    }
                });

                true
            }

            ChatAppMsg::DeleteGroupResponse(result) => {
                match result {
                    Ok(message) => {
                        self.error_message = None;
                        log::info!("Group successfully deleted: {}", message);
                    }
                    Err(err) => {
                        self.error_message = Some(err.clone());
                        log::error!("Server response: {}", err);
                    }
                }
                true
            }

            ChatAppMsg::SelectGroup(index) => {
                self.selected_group = Some(index);
                true
            }
            ChatAppMsg::SendMessage(message) => {
                if let Some(selected_index) = self.selected_group {
                    if !message.trim().is_empty() {
                        log::info!("Message sent: {}", message);
                        self.groups[selected_index]
                            .chat_history
                            .push(message.clone());
                        ctx.link()
                            // Clone the message to avoid moving it
                            .send_message(ChatAppMsg::SendWebSocketMessage(message.clone()));
                    }
                }
                true
            }
            ChatAppMsg::Login(username, password) => {
                // First validate that username and password are not empty
                if username.trim().is_empty() || password.trim().is_empty() {
                    self.error_message = Some("All fields need to be filled out.".to_string());
                    return true;
                }

                let link = ctx.link().clone();

                // Create the JSON body for the request
                let body = serde_json::json!({
                    "username": username,
                    "password": password
                })
                .to_string();

                // Send the HTTP POST request
                let request = reqwasm::http::Request::post("http://localhost:8081/login")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send();

                // Handle the HTTP response
                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            if status == 200 {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        match (
                                            json["id"].as_i64(),
                                            json["username"].as_str(),
                                            json["access_token"].as_str(),
                                        ) {
                                            (Some(id), Some(username), Some(access_token)) => {
                                                link.send_message(ChatAppMsg::LoginResponse(Ok((
                                                    id.to_string(),
                                                    username.to_string(),
                                                    access_token.to_string(),
                                                ))));
                                            }
                                            _ => {
                                                let error_msg = format!(
                                                    "Invalid JSON structure. Received: {:?}",
                                                    json
                                                );
                                                log::error!("{}", error_msg);
                                                link.send_message(ChatAppMsg::LoginResponse(Err(
                                                    error_msg,
                                                )));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = format!("JSON parsing error: {:?}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                }
                            } else {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let error_msg = format!(
                                            "Status {}: {}",
                                            status,
                                            json["error"].as_str().unwrap_or("Unknown error")
                                        );
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                    Err(e) => {
                                        let error_msg =
                                            format!("Failed to parse error response: {}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("Network error: {}", e);
                            log::error!("{}", error_msg);
                            link.send_message(ChatAppMsg::LoginResponse(Err(error_msg)));
                        }
                    }
                });

                self.error_message = None;
                true
            }
            ChatAppMsg::LoginResponse(result) => {
                match result {
                    Ok((_id, username, access_token)) => {
                        self.logged_in = true;
                        self.error_message = None;
                        self.token = Some(access_token);
                        self.current_page = Page::MainPage;
                        log::info!("Logged in as: {}", username);
                        ctx.link().send_message(ChatAppMsg::ConnectWebSocket);
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                true
            }

            ChatAppMsg::Register(username, password) => {
                if username.trim().is_empty() || password.trim().is_empty() {
                    self.error_message = Some("All fields need to be filled out".to_string());
                    return true;
                }

                let link = ctx.link().clone();

                // Create the JSON body
                let body =
                    serde_json::json!({ "username": username, "password": password }).to_string();

                // Send the HTTP request
                let request = reqwasm::http::Request::post("http://localhost:8081/signup")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send();

                // Handle the HTTP response
                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            if status == 200 {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let username = json["username"]
                                            .as_str()
                                            .unwrap_or_default()
                                            .to_string();
                                        link.send_message(ChatAppMsg::RegisterResponse(Ok(
                                            username,
                                        )));
                                    }
                                    Err(e) => {
                                        let error_msg = format!("JSON parsing error: {:?}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                }
                            } else {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let error_msg = format!(
                                            "Status {}: {}",
                                            status,
                                            json["error"].as_str().unwrap_or("Unknown error")
                                        );
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                    Err(e) => {
                                        let error_msg =
                                            format!("Failed to parse error response: {}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(
                                            error_msg,
                                        )));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("Network error: {}", e);
                            log::error!("{}", error_msg);
                            link.send_message(ChatAppMsg::RegisterResponse(Err(error_msg)));
                        }
                    }
                });

                self.error_message = None;
                true
            }

            ChatAppMsg::RegisterResponse(result) => {
                match result {
                    Ok(username) => {
                        self.error_message = None;
                        log::info!("Successfully registered: {}", username);
                        self.current_page = Page::LoginPage;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                true
            }
            ChatAppMsg::Logout => {
                self.logged_in = false;
                self.current_page = Page::LoginPage;
                self.error_message = None;
                ctx.link().send_message(ChatAppMsg::DisconnectWebSocket);
                true
            }
            // websocket
            ChatAppMsg::ConnectWebSocket => {
                log::info!("Connecting to WebSocket");
                let token = self.token.clone().unwrap_or_default();
                let ws = WebSocket::open(&format!("ws://localhost:8081/api/ws/connect/{}", token))
                    .expect("Failed to connect WebSocket");
                let (write, mut read) = ws.split();
                self.ws_write = Some(Rc::new(RefCell::new(write)));
                log::info!("WebSocket connected");
                let link = ctx.link().clone();
                self.ws_ping_interval =
                    Some(gloo::timers::callback::Interval::new(5000, move || {
                        link.send_message(ChatAppMsg::SendWebSocketMessage("ping".to_string()));
                    }));
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                link.send_message(ChatAppMsg::WebSocketMessageReceived(text));
                            }
                            Ok(_) => (),
                            Err(e) => log::error!("WebSocket error: {:?}", e),
                        }
                    }
                });
                true
            }
            ChatAppMsg::DisconnectWebSocket => {
                self.ws_write = None;
                true
            }
            ChatAppMsg::WebSocketMessageReceived(message) => {
                log::info!("Received WebSocket message: {}", message);
                // message receive is done: browser console:
                // "Received WebSocket message: {"msg_id":1,"sender_id":2,"group_id":1,"content":"hello world in group 1","created_at":1734206817}"
                // TODO: handle the message, display on group page
                true
            }
            ChatAppMsg::SendWebSocketMessage(message) => {
                log::info!("Sending WebSocket message: {}", message);
                let ws_write = self.ws_write.clone();
                if let Some(ws_write) = ws_write {
                    wasm_bindgen_futures::spawn_local(async move {
                        ws_write
                            .borrow_mut()
                            .send(Message::Text(message))
                            .await
                            .unwrap();
                    });
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.current_page {
            Page::LoginPage => view::render_login_page(self, ctx),
            Page::MainPage => view::render_main_page(self, ctx),
            Page::NewGroupPage => view::render_new_group_page(self, ctx),
            Page::RegistrationPage => view::render_registration_page(self, ctx),
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ChatApp>::new().render();
}
