mod app;
mod pages;
mod router;
mod components;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
