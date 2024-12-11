mod model;
mod msg;
mod view;
mod registration_page;
mod route;

use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use model::Model;
use registration_page::RegistrationPage;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Model /> },
        Route::Register => html! { <RegistrationPage /> },
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}
