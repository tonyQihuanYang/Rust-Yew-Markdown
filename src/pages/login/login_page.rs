use crate::features::login_page::login_form::LoginForm;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
        <>
            <LoginForm/>
        </>
    }
}
