use crate::pages::{
    error::error_page::ErrorPage, login::login_page::LoginPage,
    markdown::markdown_page::MarkdownPage,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    Error,
}

pub fn switch_page(routes: &Route) -> yew::virtual_dom::VNode {
    match routes {
        Route::Home => html! { <MarkdownPage/> },
        Route::Login => html! {
            <LoginPage/>
        },
        Route::Error => html! { <ErrorPage/> },
    }
}
