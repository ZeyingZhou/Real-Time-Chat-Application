mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
