mod app;
mod components;
mod env;
mod features;
mod graphql_requests;
mod models;

use crate::env::get_environment_variables;
use crate::env::APIEnvironmentVariables;
use crate::env::EnvironmentVariables;
use app::App;

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     #[derive(Debug)]
//     static ref ENV: EnvironmentVariables = get_environment_variables().expect("Could not load env");
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
