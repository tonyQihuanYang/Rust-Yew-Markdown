use yew::prelude::*;

#[function_component(ErrorPage)]
pub fn error_page() -> Html {
    html! {
        <div>
            <h1>{ "Unexpected Error" }</h1>
        </div>
    }
}
