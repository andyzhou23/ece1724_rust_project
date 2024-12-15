use crate::{ChatApp, ChatAppMsg, Page};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub fn render_registration_page(app: &ChatApp, ctx: &Context<ChatApp>) -> Html {
    let link = ctx.link();

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h3>{ "Register" }</h3>
            <input
                type="text"
                placeholder="Username"
                id="reg_username"
                style="margin-bottom: 10px; padding: 5px;"
            />
            <input
                type="password"
                placeholder="Password"
                id="reg_password"
                style="margin-bottom: 10px; padding: 5px;"
            />
            <button
                onclick={link.callback(|_| {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let username = document.get_element_by_id("reg_username")
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    let password = document.get_element_by_id("reg_password")
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    ChatAppMsg::Register(username, password)
                })}
                style="padding: 10px 20px; background-color: #007bff; color: white; border: none; cursor: pointer;"
            >
                { "Register" }
            </button>
            <button
                onclick={link.callback(|_| ChatAppMsg::NavigateTo(Page::LoginPage))}
                style="margin-top: 10px; padding: 5px 15px; background-color: #6c757d; color: white; border: none; cursor: pointer;"
            >
                { "Back to Login" }
            </button>
            {
                if let Some(error) = &app.error_message {
                    html! {
                        <div style="color: red; margin-top: 20px;">{ error }</div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

pub fn render_login_page(app: &ChatApp, ctx: &Context<ChatApp>) -> Html {
    let link = ctx.link();

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h3>{ "Login" }</h3>
            <input
                type="text"
                placeholder="Username"
                id="login_username"
                style="margin-bottom: 10px; padding: 5px;"
            />
            <input
                type="password"
                placeholder="Password"
                id="login_password"
                style="margin-bottom: 10px; padding: 5px;"
            />
            <button
                onclick={link.callback(|_| {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let username = document.get_element_by_id("login_username")
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    let password = document.get_element_by_id("login_password")
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    ChatAppMsg::Login(username, password)
                })}
                style="padding: 10px 20px; background-color: #007bff; color: white; border: none; cursor: pointer;"
            >
                { "Login" }
            </button>
            <button
                onclick={link.callback(|_| ChatAppMsg::NavigateTo(Page::RegistrationPage))}
                style="margin-top: 10px; padding: 5px 15px; background-color: #6c757d; color: white; border: none; cursor: pointer;"
            >
                { "Register" }
            </button>
            {
                if let Some(error) = &app.error_message {
                    html! {
                        <div style="color: red; margin-top: 20px;">{ error }</div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}



pub fn render_main_page(app: &ChatApp, ctx: &Context<ChatApp>) -> Html {
    let link = ctx.link();

    html! {
        <div style="display: flex; flex-direction: column; height: 99vh; width: 99vw;">
            // Header
            <div style="padding: 10px; background-color: #007bff; color: white; display: flex; justify-content: space-between; align-items: center;">
                <h3>{ "Main Page" }</h3>
                <div style="display: flex; gap: 10px;">
                    <button
                        onclick={link.callback(|_| ChatAppMsg::NavigateTo(Page::NewGroupPage))}
                        style="padding: 10px 20px; background: #28a745; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    >
                        { "New Group" }
                    </button>
                    <button
                        onclick={link.callback(|_| ChatAppMsg::Logout)}
                        style="padding: 10px 20px; background: #ff4d4d; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    >
                        { "Logout" }
                    </button>
                 </div>
            </div>

            // Main content
            <div style="display: flex; flex: 1; overflow: hidden;">
                // Sidebar for group list
                <div style="width: 250px; background-color: #f8f9fa; border-right: 1px solid #dee2e6; overflow-y: auto;">
                    <div style="padding: 15px;">
                        <h4>{ "Your Groups" }</h4>
                        <ul style="list-style: none; padding: 0;">
                            { for app.groups.iter().enumerate().map(|(index, group)| {
                                let group_name = group.name.clone();
                                let group_id = group.id.clone();
                                let is_selected = app.selected_group == Some(index);
                                html! {
                                    <li style="margin-bottom: 8px;">
                                        <div style="display: flex; gap: 8px;">
                                            <button
                                                onclick={link.callback(move |_| ChatAppMsg::SelectGroup(index))}
                                                style={format!("flex: 1; padding: 8px; text-align: left; 
                                                    background: {}; color: #333; border: 1px solid #dee2e6; 
                                                    border-radius: 4px; cursor: pointer; 
                                                    transition: background-color 0.2s;",
                                                    if is_selected { "#e9ecef" } else { "#ffffff" }
                                                )}
                                            >
                                                { format!("{} # {}", group_name, group.code) }
                                            </button>
                                            <button
                                                onclick={link.callback(move |_| {
                                                    log::info!("Deleting group: {}", group_id);
                                                    ChatAppMsg::DeleteGroup(group_id.clone())
                                                })}
                                                style="padding: 8px; background: #ff4d4d; color: white; 
                                                       border: none; border-radius: 4px; cursor: pointer;"
                                            >
                                                { "×" }
                                            </button>
                                        </div>
                                    </li>
                                }
                            })}
                        </ul>
                    </div>
                </div>

                // Chat window for selected group
                <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden;">
                    {
                        if let Some(selected_index) = app.selected_group {
                            let selected_group = &app.groups[selected_index];
                            html! {
                                <>
                                    /* 
                                    <div style="padding: 15px; background-color: #f8f9fa; border-bottom: 1px solid #dee2e6;">
                                        <h4 style="margin: 0;">{ &selected_group.name }</h4>
                                        <small style="color: #666;">
                                            { format!("{} members", selected_group.members.len()) }
                                        </small>
                                    </div>*/
                                    <div style="padding: 15px; background-color: #f8f9fa; border-bottom: 1px solid #dee2e6;">
                                        <h4 style="margin: 0;">{ &selected_group.name }</h4>
                                        <small style="color: #666;">
                                            { format!("{} online member(s) | Online: ", selected_group.members.len()) }
                                            <span style="color: green;">
                                                { selected_group.members.iter()
                                                    .map(|m| m.name.clone())
                                                    .collect::<Vec<_>>()
                                                    .join(", ") 
                                                }
                                            </span>
                                        </small>
                                    </div>
                                    
                                    // Chat messages
                                    <div style="flex: 1; padding: 20px; overflow-y: auto;">
                                        { for selected_group.chat_history.iter().map(|message| {
                                            html! {
                                                <div style="margin-bottom: 10px; padding: 8px; 
                                                            background-color: #007bff; color: white; 
                                                            border-radius: 4px;">
                                                    { message }
                                                </div>
                                            }
                                        })}
                                    </div>

                                    // Chat input
                                    <div style="padding: 15px; background-color: #f8f9fa; border-top: 1px solid #dee2e6;">
                                        <div style="display: flex; gap: 10px;">
                                            <input
                                                type="text"
                                                id="chat-input"
                                                placeholder="Type a message..."
                                                style="flex: 1; padding: 8px; border: 1px solid #dee2e6; border-radius: 4px;"
                                            />
                                            <button
                                                onclick={link.callback(|_| {
                                                    let window = web_sys::window().unwrap();
                                                    let document = window.document().unwrap();
                                                    let input = document
                                                        .get_element_by_id("chat-input")
                                                        .unwrap()
                                                        .dyn_into::<web_sys::HtmlInputElement>()
                                                        .unwrap();
                                                    let message = input.value();
                                                    input.set_value(""); // Clear the input after sending
                                                    ChatAppMsg::SendMessage(message)
                                                })}
                                                style="padding: 8px 16px; background: #007bff; color: white; 
                                                       border: none; border-radius: 4px; cursor: pointer;"
                                            >
                                                { "Send" }
                                            </button>
                                        </div>
                                    </div>
                                </>
                            }
                        } else {
                            html! {
                                <div style="flex: 1; display: flex; justify-content: center; 
                                           align-items: center; color: #666;">
                                    { "Select a group to start chatting" }
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}


pub fn render_new_group_page(app: &ChatApp, ctx: &Context<ChatApp>) -> Html {
    let link = ctx.link();

    let onclick_create = link.callback(|_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let group_name = document
            .get_element_by_id("group_name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        ChatAppMsg::CreateGroup(group_name)
    });

    let onclick_join = link.callback(|_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let join_code = document
            .get_element_by_id("join_code")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        ChatAppMsg::JoinGroup(join_code)
    });

    html! {
        <div style="display: flex; flex-direction: column; height: 99vh; width: 99vw;">
            <div style="padding: 10px; background-color: #007bff; color: white; display: flex; justify-content: space-between; align-items: center;">
                <h3>{ "New Group Page" }</h3>
                <button
                    onclick={link.callback(|_| ChatAppMsg::NavigateTo(Page::MainPage))}
                    style="padding: 10px 20px; background: #ff4d4d; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    { "Back to Main" }
                </button>
            </div>

            <div style="padding: 20px;">
                <div>
                    <h4>{ "Create Group" }</h4>
                    <input 
                        type="text" 
                        placeholder="Group Name" 
                        id="group_name" 
                        style="margin-bottom: 10px; padding: 5px; width: 200px;" 
                    />
                    <br/>
                    <button
                        onclick={onclick_create}
                        style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    >
                        { "Create Group" }
                    </button>
                </div>
                <div style="margin-top: 20px;">
                    <h4>{ "Join Group" }</h4>
                    <input 
                        type="text" 
                        placeholder="Enter Join Code" 
                        id="join_code" 
                        style="margin-bottom: 10px; padding: 5px; width: 200px;" 
                    />
                    <br/>
                    <button
                        onclick={onclick_join}
                        style="padding: 10px 20px; background: #28a745; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    >
                        { "Join Group" }
                    </button>
                </div>
            </div>

            { if let Some(error) = &app.error_message {
                html! {
                    <div style="color: red; margin-top: 20px; padding: 0 20px;">{ error }</div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}



