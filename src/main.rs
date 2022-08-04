mod app;
mod components;
mod features;
mod graphql_requests;
mod models;
mod pages;

use app::App;

static API_URL: &'static str = "http://localhost:8081/graphql";
// let url = String::from("https://apps.gummui.com/markdown-api/graphql");
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
