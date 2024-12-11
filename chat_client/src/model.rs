use crate::msg::Msg;
use crate::view::ModelView;
use yew::prelude::*;
use gloo_timers::callback::Timeout;

pub struct Model {
    pub username: String,
    pub password: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    login_username: Option<String>,
    login_password: Option<String>,
    flag:bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: "".into(),
            password: "".into(),
            error_message: None,
            success_message: None,
            login_username: None,
            login_password: None,
            flag:true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(username) => {
                self.username = username;
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
            }
            Msg::SubmitForm => {
                if self.username.is_empty() || self.password.is_empty() {
                    self.error_message = Some("Both fields are required!".into());
                    self.success_message = None;
                } else {
                    self.error_message = None;
                    self.login_username = Some(self.username.clone());
                    self.login_password = Some(self.password.clone());
                    self.success_message = Some(format!("Successfully logged in as: {}", self.username));
                    log::info!("Login attempt - Username: {}, Password: {}", 
                        self.login_username.as_ref().unwrap(),
                        self.login_password.as_ref().unwrap());
                    
                    if self.flag {
                        let link = ctx.link().clone();
                        Timeout::new(1000, move || {
                            link.send_message(Msg::ShowMainPage);
                        }).forget();
                    }
                }
            }
            Msg::Register => {
                self.success_message = Some("Registration clicked!".into());
            }
            Msg::Logout => {
                self.username = "".into();
                self.password = "".into();
                self.success_message = Some("Successfully logged out".into());
            }
            Msg::ShowMainPage => {
                self.success_message = Some("Login success, jump to main page.....".into());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_container(ctx)
    }
}
