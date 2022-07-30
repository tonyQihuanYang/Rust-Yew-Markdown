use crate::pages::router::{switch_page, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch_page)} />
        </BrowserRouter>
    }
}
