mod app;
mod event_bus;
mod websocket;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting.");
    yew::start_app::<App>();
}
