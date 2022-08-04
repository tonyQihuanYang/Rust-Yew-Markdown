use yew::prelude::*;

#[function_component(EmptyContent)]
pub fn empty_content() -> Html {
    html! {
        <div class="no-content-wrapper">
            // <img src="img/avatar.png"
            // class="rounded mx-auto d-block p-2"
            // width="300" height="300"
            // />
            <h5 style="font-size: 260px;">{"🤔"}</h5>
            <h5>{"Nahh! No markdown selected/found..."}</h5>
        </div>
    }
}
