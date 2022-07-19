use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct GmButtonProps {
    pub title: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(GmButton)]
pub fn gm_button(props: &GmButtonProps) -> Html {
    let GmButtonProps { title, on_click} = props;
    return html! (
        <>
            <button
                type="button"
                class="btn btn-primary"
                onclick={on_click.clone()}
            >
                {title}
            </button>
        </>
    );
}
