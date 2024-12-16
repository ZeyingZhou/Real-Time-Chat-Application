mod api;
mod app;
mod components;
mod pages;
mod router;
mod services;
mod store;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
