use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;
use crate::route::Route;

pub struct RegistrationPage {
    username_ref: NodeRef,
    password_ref: NodeRef,
    confirm_ref: NodeRef,
    error_message: Option<String>,
    success_message: Option<String>,
    registered_username: Option<String>,
    registered_password: Option<String>,
    flag: bool,
}

pub enum Msg {
    ClearForm,
    ReturnToLogin,
    Submit,
}

impl Component for RegistrationPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username_ref: NodeRef::default(),
            password_ref: NodeRef::default(),
            confirm_ref: NodeRef::default(),
            error_message: None,
            success_message: None,
            registered_username: None,
            registered_password: None,
            flag: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClearForm => {
                if let Some(input) = self.username_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                if let Some(input) = self.password_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                if let Some(input) = self.confirm_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                true
            }
            Msg::ReturnToLogin => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Home);
                true
            }
            Msg::Submit => {
                let username = self.username_ref.cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();
                let password = self.password_ref.cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();
                let confirm = self.confirm_ref.cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();

                if username.is_empty() || password.is_empty() || confirm.is_empty() {
                    self.error_message = Some("Need to fill in all information!".into());
                } else if password != confirm {
                    self.error_message = Some("The password is not the same!".into());
                } else {
                    self.error_message = None;
                    self.registered_username = Some(username);
                    self.registered_password = Some(password);
                    self.success_message = Some("Registration information stored successfully!".into());
                    log::info!("Registration successful - Username: {}, Password: {}", 
                        self.registered_username.as_ref().unwrap(),
                        self.registered_password.as_ref().unwrap());
                    
                    if self.flag {
                        // Navigate back to login page after a short delay
                        let link = ctx.link().clone();
                        gloo_timers::callback::Timeout::new(1000, move || {
                            link.navigator().unwrap().push(&Route::Home);
                        }).forget();
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="registration-container">
                <h2>{"Registration Page"}</h2>
                <div class="form">
                    <div>
                        <label for="create-username">{"Create Username"}</label>
                        <input
                            type="text"
                            id="create-username"
                            ref={self.username_ref.clone()}
                        />
                    </div>
                    <div>
                        <label for="create-password">{"Create Password"}</label>
                        <input
                            type="password"
                            id="create-password"
                            ref={self.password_ref.clone()}
                        />
                    </div>
                    <div>
                        <label for="confirm-password">{"Confirm Password"}</label>
                        <input
                            type="password"
                            id="confirm-password"
                            ref={self.confirm_ref.clone()}
                        />
                    </div>
                    <div class="button-group">
                        <button 
                            class="ok-btn"
                            onclick={ctx.link().callback(|_| Msg::Submit)}
                        >
                            { "OK" }
                        </button>
                        <button 
                            class="resume-btn"
                            onclick={ctx.link().callback(|_| Msg::ClearForm)}
                        >
                            { "Resume" }
                        </button>
                        <button 
                            class="return-btn"
                            onclick={ctx.link().callback(|_| Msg::ReturnToLogin)}
                        >
                            { "Return" }
                        </button>
                    </div>
                    if let Some(error) = &self.error_message {
                        <div class="error-message">
                            { error }
                        </div>
                    }
                    if let Some(success) = &self.success_message {
                        <div class="success-message">
                            { success }
                        </div>
                    }
                </div>
            </div>
        }
    }
} 