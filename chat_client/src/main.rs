mod view;
//use reqwasm::http::Request;
use std::collections::HashMap;
use yew::prelude::*;

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
    //registered_users: HashMap<String, String>,
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
    CreateGroup(String, String),
    DeleteGroup(String),
    SelectGroup(usize),
    SendMessage(String),
    Login(String, String), 
    LoginResponse(Result<(String, String), String>),
    Logout,
    Register(String, String), // Registration with username and password
    RegisterResponse(Result<String, String>), // Handle API response
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
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatAppMsg::NavigateTo(page) => {
                self.current_page = page;
                self.error_message = None;
                true
            }
            ChatAppMsg::CreateGroup(name, join_id) => {
                // This code handles the creation of a new chat group when a user submits the create group form
                log::info!("Creating group: {} with join ID: {}", name, join_id);

                // First validate that the required fields are not empty
                if name.is_empty() || join_id.is_empty() {
                    self.error_message = Some("Group name and Join ID cannot be empty".to_string());
                    return true;
                }

                // Check if the join ID is already being used by another group
                if self.join_codes.contains_key(&join_id) {
                    self.error_message = Some("Join ID is already in use. Try another.".to_string());
                    return true;
                }

                // Generate a unique group ID
                let group_id = format!("group-{}", self.groups.len() + 1);
                
                // Create a new Group struct with initial values
                let new_group = Group {
                    id: group_id.clone(),
                    name: name.clone(),
                    join_code: Some(join_id.clone()),
                    members: vec![Member {
                        name: "Owner".to_string(), 
                        status: "online".to_string(),
                    }],
                    chat_history: vec!["Welcome to the group!".to_string()],
                    is_owner: true,
                };

                log::info!("New group created: {:?}", new_group.name);
                
                // Add the new group to the app state
                self.groups.push(new_group);
                self.join_codes.insert(join_id, group_id);
                self.selected_group = Some(self.groups.len() - 1); // Select the newly created group
                self.current_page = Page::MainPage; // Navigate back to main page
                self.error_message = None; // Clear any previous error messages
                true // Return true to trigger a re-render
            }
            ChatAppMsg::DeleteGroup(group_id) => {
                // Find and remove the group with matching ID
                if let Some(index) = self.groups.iter().position(|g| g.id == group_id) {
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
                    log::info!("Group deleted: {}", group_id);
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
                        self.groups[selected_index].chat_history.push(message);
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
                                        match (json["id"].as_i64(), json["username"].as_str()) {
                                            (Some(id), Some(username)) => {
                                                let jwt_token = json["token"].as_str().unwrap_or_default().to_string();//local variable
                                                link.send_message(ChatAppMsg::LoginResponse(Ok((id.to_string(), username.to_string()))));
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
                    Ok((_id, username)) => {
                        self.logged_in = true;
                        self.error_message = None;
                        self.current_page = Page::MainPage; // Navigate to MainPage
                        log::info!("Logged in as: {}", username);
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
