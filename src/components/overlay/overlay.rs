use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct GmOverlayProps {
    pub on_close: Callback<()>,
    pub children: Children,
}
use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_OVERLAY_ID: AtomicUsize = AtomicUsize::new(0);

fn generate_id(prefix: String) -> String {
    let id = GLOBAL_OVERLAY_ID.fetch_add(1, Ordering::SeqCst).to_string();
    let mut id_generated = prefix.clone();
    let under_score = String::from("_");

    id_generated.push_str(&under_score);
    id_generated.push_str(&id);
    return id_generated;
}

#[function_component(GmOverlay)]
pub fn gm_overlay(props: &GmOverlayProps) -> Html {
    let GmOverlayProps { on_close, children } = props;
    let id = generate_id(String::from("GM_OVERLAY"));
    let click_on_overlay = {
        let on_close = on_close.clone();
        let id = id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            match e.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                Some(current_element) => {
                    if current_element.id() == id {
                        log::info!("comes here");
                        on_close.emit({});
                    }
                }
                None => {}
            }
        })
    };

    return html!(
        <div
            id={id}
            class={"gm-overlay"}
            onclick={click_on_overlay.clone()}
        >
            { for children.iter() }
        </div>
    );
}
