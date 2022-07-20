use yew::prelude::*;

const Z_INDEX: u32 = 100;

#[derive(PartialEq, Clone, Properties)]
pub struct GmOverlayProps {
    // pub handle_close: Callback<()>,
    pub children: Children
}

#[function_component(GmOverlay)]
pub fn gm_overlay(props: &GmOverlayProps) -> Html {

    let GmOverlayProps{children} = props;
    return html!(
        <div class="overlay">
            { for children.iter() }
        </div>
    );
}
