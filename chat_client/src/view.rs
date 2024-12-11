use crate::model::Model;
use crate::msg::Msg;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;

pub trait ModelView {
    fn view_error(&self) -> Html;
    fn view_success(&self) -> Html;
    fn view_form(&self, ctx: &Context<Model>) -> Html;
    fn view_container(&self, ctx: &Context<Model>) -> Html;
}

impl ModelView for Model {
    fn view_error(&self) -> Html {
        if let Some(ref error_message) = self.error_message {
            html! {
                <div class="error-message">
                    { error_message }
                </div>
            }
        } else {
            html! {}
        }
    }

    fn view_success(&self) -> Html {
        if let Some(ref message) = self.success_message {
            html! {
                <div class="success-message">
                    { message }
                </div>
            }
        } else {
            html! {}
        }
    }

    fn view_form(&self, ctx: &Context<Model>) -> Html {
        html! {
            <div class="form">
                <div>
                    <label for="username">{"Username"}</label>
                    <input
                        type="text"
                        id="username"
                        value={self.username.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateUsername(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                    />
                </div>
                <div>
                    <label for="password">{"Password"}</label>
                    <input
                        type="password"
                        id="password"
                        value={self.password.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| Msg::UpdatePassword(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                    />
                </div>
                <div class="button-group">
                    <button onclick={ctx.link().callback(|_| Msg::SubmitForm)}>{ "Login" }</button>
                    <button class="register-btn" onclick={ctx.link().callback(|_| Msg::Register)}>
                        <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                    </button>
                    <button class="logout-btn" onclick={ctx.link().callback(|_| Msg::Logout)}>{ "Logout" }</button>
                </div>
            </div>
        }
    }

    fn view_container(&self, ctx: &Context<Model>) -> Html {
        html! {
            <div class="login-container">
                <h2>{"Group Chat Account Login"}</h2>
                { self.view_form(ctx) }
                { self.view_error() }
                { self.view_success() }
            </div>
        }
    }
}
