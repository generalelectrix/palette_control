#![recursion_limit = "512"]

mod app;
mod event_bus;
mod websocket;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;

use app::App;

#[function_component(Main)]
fn main() -> Html {
    html! {
        <div class="flex w-screen h-screen">
            <App/>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
