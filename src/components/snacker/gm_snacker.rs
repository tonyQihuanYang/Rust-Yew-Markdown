use super::super::modal::modal::GmModal;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct GmSnackerProps {
    pub visible: UseStateHandle<bool>,
    pub title: Option<String>,
    pub children: Option<Children>,
}

#[function_component(GmSnacker)]
pub fn gm_snacker(props: &GmSnackerProps) -> Html {
    let GmSnackerProps {
        visible,
        children,
        title,
    } = props;

    let children_dom = match children {
        Some(dom) => html! {for dom.iter()},
        None => html! {<></>},
    };
    return html! {
        <GmModal visible={visible.clone()} title={title.clone()}>
            {children_dom}
        </GmModal>
    };
}
