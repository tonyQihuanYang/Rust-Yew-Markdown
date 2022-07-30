mod app;
mod components;
mod features;
mod graphql_requests;
mod models;
mod pages;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
