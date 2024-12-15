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
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq, serde::Deserialize)]
struct Group {
    id: i64,
    name: String,
    code: String,
    created_at: i64,
    members: Vec<Member>,
    chat_history: Vec<String>,
    latest_msg_id: i64,

}

#[derive(Clone, PartialEq, serde::Deserialize)]
struct Member {
    id: i64,
    name: String,
}

struct ChatApp {
    groups: Vec<Group>,
    join_codes: HashMap<String, String>,
    selected_group: Option<usize>,
    current_page: Page,
    error_message: Option<String>,
    logged_in: bool,
    token: Option<String>,
    current_user: Option<Member>,
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
    DeleteGroup(i64),
    SelectGroup(usize),
    SendMessage(String),
    Login(String, String), 
    LoginResponse(Result<(i64, String, String), String>),
    Logout,
    Register(String, String), // Registration with username and password
    RegisterResponse(Result<String, String>), // Handle API response
    JoinGroup(String),
    CreateGroupResponse(Result<(i64, String, String), String>),
    JoinGroupResponse(Result<(i64, String, String), String>),
    DeleteGroupResponse(Result<String, String>),
    UpdateGroups(Vec<Group>),
    // websocket
    ConnectWebSocket,
    DisconnectWebSocket,
    WebSocketMessageReceived(String),
    SendWebSocketMessage(String),
    UpdateChatHistory(i64, Vec<String>, i64),
    UpdateOnlineMembers(HashMap<i64, String>),
    FetchOnlineMembers(i64), // To fetch online members for a specific group

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
            current_user: None,
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
                let request = reqwasm::http::Request::post("http://localhost:8081/api/group/create")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &format!("Bearer {}", token.unwrap_or_default()))
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
                                        let group_id = json["group_id"].as_i64().unwrap_or_default();
                                        let group_name = json["group_name"].as_str().unwrap_or("").to_string();
                                        let group_code = json["group_code"].as_str().unwrap_or("").to_string();
                                        // Send a message to update the UI
                                        link.send_message(ChatAppMsg::CreateGroupResponse(Ok((group_id, group_name, group_code))));
                                    }
                                    Err(_) => {
                                        link.send_message(ChatAppMsg::CreateGroupResponse(Err("Failed to parse response".to_string())));
                                    }
                                }
                            } else {
                                link.send_message(ChatAppMsg::CreateGroupResponse(Err("Server error".to_string())));
                            }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::CreateGroupResponse(Err("Network error".to_string())));
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
                            code: group_code,
                            created_at: 0,
                            members: vec![Member {
                                id: 1,
                                name: "Owner".to_string(),
                            }],
                            chat_history: Vec::new(),
                            latest_msg_id: 0,
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
                    .header("Authorization", &format!("Bearer {}", token.unwrap_or_default()))
                    .body(body)
                    .send();

                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            let status = response.status();
                            if status == 200 {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let group_id = json["group_id"].as_i64().unwrap_or_default().to_string();
                                        let group_name = json["group_name"].as_str().unwrap_or("").to_string();
                                        let group_code = json["group_code"].as_str().unwrap_or("").to_string();
            
                                        // Send a message to update the UI
                                        link.send_message(ChatAppMsg::JoinGroupResponse(Ok((group_id.parse::<i64>().unwrap_or_default(), group_name, group_code))));
                                    }
                                    Err(_) => {
                                        link.send_message(ChatAppMsg::JoinGroupResponse(Err("Failed to parse response".to_string())));
                                    }
                                }
                            } else {
                                if status == 400 {
                                    let error_msg = "Invalid group code or user already in group".to_string();
                                    log::error!("{}", error_msg);
                                    link.send_message(ChatAppMsg::JoinGroupResponse(Err(error_msg)));
                                } else {
                                    match response.text().await {
                                        Ok(error_text) => {
                                            let error_msg = format!("Status {}: {}", status, error_text);
                                            log::error!("{}", error_msg);
                                            link.send_message(ChatAppMsg::JoinGroupResponse(Err(error_msg)));
                                        }
                                        Err(e) => {
                                            let error_msg = format!("Failed to read error response: {}", e);
                                            log::error!("{}", error_msg);
                                            link.send_message(ChatAppMsg::JoinGroupResponse(Err(error_msg)));
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::JoinGroupResponse(Err("Network error".to_string())));
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
                            code: group_code,
                            created_at: 0,
                            members: vec![Member {
                                id: 2,
                                name: "Member".to_string(),
                            }],
                            chat_history: Vec::new(),
                            latest_msg_id: 0,
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
                if let Some(index) = self.groups.iter().position(|g| g.id == group_id) {
                    let group = &self.groups[index];
                    // Remove the join code from the HashMap if it exists
                    self.join_codes.remove(&group.code);//changed
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
                let body = serde_json::json!({ "group_id": group_id }).to_string();

                // Send the HTTP request
                let request = reqwasm::http::Request::post("http://localhost:8081/api/group/leave")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &format!("Bearer {}", token.unwrap_or_default()))
                    .body(body)
                    .send();

                wasm_bindgen_futures::spawn_local(async move {
                    match request.await {
                        Ok(response) => {
                            //let status = response.status();
                            let json_response = response.json::<serde_json::Value>().await;
                            log::info!("Response JSON: {:?}", json_response);

                            // if status == 200 {
                            //     match json_response {
                            //         Ok(json) => {
                            //             if let Some(message) = json["message"].as_str() {
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Ok(message.to_string())));
                            //             } else {
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Err("Invalid response format".to_string())));
                            //             }
                            //         }
                            //         Err(_) => {
                            //             link.send_message(ChatAppMsg::DeleteGroupResponse(Err("Failed to parse response".to_string())));
                            //         }
                            //     }
                            // } else {
                            //     if status == 400 {
                            //         match json_response {
                            //             Ok(json) => {
                            //                 let error_msg = json["error"].as_str().unwrap_or_default().to_string();
                            //                 log::error!("Server error message: {}", error_msg);
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Err(error_msg)));
                            //             }
                            //             Err(e) => {
                            //                 let error_msg = format!("Failed to parse error response: {}", e);
                            //                 log::error!("{}", error_msg);
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Err(error_msg)));
                            //             }
                            //         }
                            //     } else {
                            //         match response.text().await {
                            //             Ok(error_text) => {
                            //                 let error_msg = format!("Status {}: {}", status, error_text);
                            //                 log::error!("{}", error_msg);
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Err(error_msg)));
                            //             }
                            //             Err(e) => {
                            //                 let error_msg = format!("Failed to read error response: {}", e);
                            //                 log::error!("{}", error_msg);
                            //                 link.send_message(ChatAppMsg::DeleteGroupResponse(Err(error_msg)));
                            //             }
                            //         }
                            //     }
                            // }
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::DeleteGroupResponse(Err("Network error".to_string())));
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
                if let Some(group) = self.groups.get(index) {
                    self.fetch_online_members(ctx.link(), group.id); // Fetch online members
                    self.fetch_chat_history(ctx.link(), group.id, group.latest_msg_id);
                    self.setup_online_user_refresh(ctx.link(), group.id); // Set up 3-second refresh
                }
                true
            }
            
            ChatAppMsg::SendMessage(message) => {
                if let Some(selected_index) = self.selected_group {
                    if !message.trim().is_empty() {
                        log::info!("Message sent: {}", message);
                        //self.groups[selected_index].chat_history.push(message.clone());
                        let group_id = self.groups[selected_index].id;
                        let message_json = serde_json::json!({
                            "group_id": group_id,
                            "content": message
                        }).to_string();
                        ctx.link().send_message(ChatAppMsg::SendWebSocketMessage(message_json));
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
                                        match (json["id"].as_i64(), json["username"].as_str(), json["access_token"].as_str()) {
                                            (Some(id), Some(username), Some(access_token)) => {
                                                link.send_message(ChatAppMsg::LoginResponse(Ok((id, username.to_string(), access_token.to_string()))));
                                            }
                                            _ => {
                                                let error_msg = format!("Invalid JSON structure. Received: {:?}", json);
                                                log::error!("{}", error_msg);
                                                link.send_message(ChatAppMsg::LoginResponse(Err(error_msg)));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = format!("JSON parsing error: {:?}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(error_msg)));
                                    }
                                }
                            } else {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let error_msg = format!("Status {}: {}", status, json["error"].as_str().unwrap_or("Unknown error"));
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(error_msg)));
                                    }
                                    Err(e) => {
                                        let error_msg = format!("Failed to parse error response: {}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::LoginResponse(Err(error_msg)));
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
                    Ok((id, username, access_token)) => {
                        self.logged_in = true;
                        self.error_message = None;
                        self.token = Some(access_token);
                        self.current_page = Page::MainPage;
                        // Store the current user's information
                        self.current_user = Some(Member {
                            id,
                            name: username,
                        });
                        log::info!("Logged in as: {}", self.current_user.as_ref().unwrap().name);
                        self.fetch_groups(ctx.link());
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
                let body = serde_json::json!({ "username": username, "password": password }).to_string();
            
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
                                        let username = json["username"].as_str().unwrap_or_default().to_string();
                                        link.send_message(ChatAppMsg::RegisterResponse(Ok(username)));
                                    }
                                    Err(e) => {
                                        let error_msg = format!("JSON parsing error: {:?}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(error_msg)));
                                    }
                                }
                            } else {
                                match response.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        let error_msg = format!("Status {}: {}", status, json["error"].as_str().unwrap_or("Unknown error"));
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(error_msg)));
                                    }
                                    Err(e) => {
                                        let error_msg = format!("Failed to parse error response: {}", e);
                                        log::error!("{}", error_msg);
                                        link.send_message(ChatAppMsg::RegisterResponse(Err(error_msg)));
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

            ChatAppMsg::UpdateGroups(groups) => {
                self.groups = groups;
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
                log::info!("Websocket disconnected");
                let ws_write = self.ws_write.clone();
                if let Some(ws_write) = ws_write {
                    wasm_bindgen_futures::spawn_local(async move {
                        ws_write
                            .borrow_mut()
                            .send(Message::Text("{disconnect}".to_string()))
                            .await
                            .unwrap();
                    });
                }
                self.ws_write = None;
                true
            }
            ChatAppMsg::WebSocketMessageReceived(message) => {
                log::info!("Received WebSocket message: {}", message);
                // message receive is done: browser console:
                // "Received WebSocket message: {"msg_id":1,"sender_id":2,"group_id":1,"content":"hello world in group 1","created_at":1734206817}"
                // TODO: handle the message, display on group page
                if let Ok(json_message) = serde_json::from_str::<serde_json::Value>(&message) {
                    log::info!("Parsed JSON message: {:?}", json_message);
                    if let Some(group_id) = json_message["group_id"].as_i64() {
                        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
                            if let Some(content) = json_message["content"].as_str() {
                                if let Some(sender_name) = json_message["sender_name"].as_str() {
                                    group.chat_history.push(format!("{}: {}", sender_name, content));
                                    group.latest_msg_id = json_message["msg_id"].as_i64().unwrap();
                                }
                            }
                        }
                    }
                    // TODO: Handle the parsed JSON message, e.g., update the chat history or UI
                } else {
                    log::info!("Received message: {}", message);
                }

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
            ChatAppMsg::UpdateChatHistory(group_id, history, latest_msg_id  ) => {
                if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
                    group.chat_history.extend(history);
                    group.latest_msg_id = latest_msg_id;
                }
                true
            }
            ChatAppMsg::UpdateOnlineMembers(members) => {
                if let Some(selected_index) = self.selected_group {
                    if let Some(group) = self.groups.get_mut(selected_index) {
                        group.members = members.into_iter()
                            .map(|(id, name)| Member { id, name })
                            .collect();
            
                        // Sort members by name alphabetically, and then by id numerically
                        group.members.sort_by(|a, b| {
                            let name_order = a.name.to_lowercase().cmp(&b.name.to_lowercase());
                            if name_order == std::cmp::Ordering::Equal {
                                a.id.cmp(&b.id) // Compare IDs if names are equal
                            } else {
                                name_order
                            }
                        });
                    }
                }
                true
            }
            ChatAppMsg::FetchOnlineMembers(group_id) => {
                self.fetch_online_members(ctx.link(), group_id);
                false // Don't re-render the component
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

impl ChatApp {
    fn fetch_chat_history(&self, link: &yew::html::Scope<Self>, group_id: i64, current_latest_msg_id: i64) {
        if let Some(token) = &self.token {
            let token = token.clone();
            let link = link.clone();

            // Create the request body
            let body = serde_json::json!({
                "user_id": self.current_user.as_ref().unwrap().id, // Use member id from struct
                "entries": [
                    {
                        "group_id": group_id,
                        "latest_msg_id": current_latest_msg_id
                    }
                ]
            })
            .to_string();

            let request = reqwasm::http::Request::post("http://localhost:8081/api/history")
                .header("Content-Type", "application/json")
                .header("Authorization", &format!("Bearer {}", token))
                .body(body)
                .send();

            wasm_bindgen_futures::spawn_local(async move {
                match request.await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    // log::info!("Received chat history response: {:?}", json);
                                    if let Some(messages) = json.get(&group_id.to_string()).and_then(|g| g.as_array()) {
                                        let chat_history: Vec<String> = messages
                                            .iter()
                                            .filter_map(|m| {
                                                let content = m.get("content").and_then(|c| c.as_str())?;
                                                let sender_name = m.get("sender_name").and_then(|n| n.as_str())?;
                                                Some(format!("{}: {}", sender_name, content))
                                            })
                                            .collect();
                                        // log::info!("Parsed chat history: {:?}", chat_history);
                                        let latest_msg_id = messages.iter()
                                            .filter_map(|m| m.get("msg_id").and_then(|id| id.as_i64()))
                                            .max()
                                            .unwrap_or(current_latest_msg_id);
                                        log::info!("Updating history: Latest message ID: {} -> {}", current_latest_msg_id, latest_msg_id);
                                        if latest_msg_id > current_latest_msg_id {
                                            link.send_message(ChatAppMsg::UpdateChatHistory(group_id, chat_history, latest_msg_id));
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to parse chat history response: {}", e);
                                }
                            }
                        } else {
                            log::error!("Failed to fetch chat history: {}", response.status());
                        }
                    }
                    Err(_) => {
                        log::error!("Network error while fetching chat history");
                    }
                }
            });
            
        }
    }
    fn fetch_groups(&self, link: &yew::html::Scope<Self>) {
        if let Some(token) = &self.token {
            let token = token.clone();
            let link = link.clone();
            let request = reqwasm::http::Request::get("http://localhost:8081/api/group/list")
                .header("Authorization", &format!("Bearer {}", token))
                .send();

            wasm_bindgen_futures::spawn_local(async move {
                match request.await {
                    Ok(response) => {
                        if response.status() == 200 {
                            // Create a custom deserializer for the API response that excludes chat_history
                            #[derive(serde::Deserialize)]
                            struct ApiGroup {
                                id: i64,
                                name: String,
                                code: String,
                                created_at: i64,
                                members: Vec<Member>,
                            }

                            match response.json::<Vec<ApiGroup>>().await {
                                Ok(api_groups) => {
                                    // Convert ApiGroup to Group by adding empty chat_history
                                    let groups = api_groups.into_iter()
                                        .map(|g| Group {
                                            id: g.id,
                                            name: g.name,
                                            code: g.code,
                                            created_at: g.created_at,
                                            members: g.members,
                                            chat_history: Vec::new(),
                                            latest_msg_id: 0,
                                        })
                                        .collect();
                                    link.send_message(ChatAppMsg::UpdateGroups(groups));
                                }
                                Err(e) => {
                                    log::error!("Failed to parse group response: {}", e);
                                }
                            }
                        } else if response.status() == 500 {
                            match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    let error_msg = format!("Database error: {}", json["error"].as_str().unwrap_or("Unknown error"));
                                    log::error!("{}", error_msg);
                                }
                                Err(e) => {
                                    log::error!("Failed to parse database error response: {}", e);
                                }
                            }
                        } else {
                            log::error!("Failed to fetch groups: {}", response.status());
                        }
                    }
                    Err(_) => {
                        log::error!("Network error while fetching groups");
                    }
                }
            });
        }
    }
    fn fetch_online_members(&self, link: &yew::html::Scope<Self>, group_id: i64) {
        if let Some(token) = &self.token {
            let token = token.clone();
            let link = link.clone();
    
            let body = serde_json::json!({ "group_id": group_id }).to_string();
    
            let request = reqwasm::http::Request::post("http://localhost:8081/api/group/status")
                .header("Content-Type", "application/json")
                .header("Authorization", &format!("Bearer {}", token))
                .body(body)
                .send();
    
            wasm_bindgen_futures::spawn_local(async move {
                match request.await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    log::info!("Received online members response: {:?}", json);
                                    let members = json["online_members"]
                                        .as_object()
                                        .unwrap_or(&serde_json::Map::new())
                                        .iter()
                                        .map(|(id, name)| (id.parse::<i64>().unwrap(), name.as_str().unwrap().to_string()))
                                        .collect::<HashMap<_, _>>();
                                    link.send_message(ChatAppMsg::UpdateOnlineMembers(members));
                                }
                                Err(_) => log::error!("Failed to parse online members response"),
                            }
                        }
                    }
                    Err(_) => log::error!("Network error while fetching online members"),
                }
            });
        }
    }

    fn setup_online_user_refresh(&self, ctx: &yew::html::Scope<Self>, group_id: i64) {
        let link = ctx.clone();
        gloo::timers::callback::Interval::new(3000, move || {
            link.send_message(ChatAppMsg::FetchOnlineMembers(group_id));
        })
        .forget();
    }
    
    
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ChatApp>::new().render();
}
