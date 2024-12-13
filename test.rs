ChatAppMsg::Login(username, password) => {
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
                if response.status() == 200 {
                    // Parse the response JSON
                    match response.json::<serde_json::Value>().await {
                        Ok(json) => {
                            let id = json["id"].as_i64().unwrap_or_default().to_string();
                            let username = json["username"].as_str().unwrap_or_default().to_string();
                            link.send_message(ChatAppMsg::LoginResponse(Ok((id, username))));
                        }
                        Err(_) => {
                            link.send_message(ChatAppMsg::LoginResponse(Err("Invalid server response.".to_string())));
                        }
                    }
                } else if response.status() == 400 {
                    link.send_message(ChatAppMsg::LoginResponse(Err("Username or password incorrect.".to_string())));
                } else if response.status() == 500 {
                    link.send_message(ChatAppMsg::LoginResponse(Err("Internal server error.".to_string())));
                } else {
                    link.send_message(ChatAppMsg::LoginResponse(Err("Unknown error.".to_string())));
                }
            }
            Err(e) => {
                link.send_message(ChatAppMsg::LoginResponse(Err(format!("Network error: {}", e))));
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