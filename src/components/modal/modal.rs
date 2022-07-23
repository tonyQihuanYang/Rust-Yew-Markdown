use crate::components::overlay::overlay::GmOverlay;
use yew::prelude::*;
#[derive(PartialEq, Clone, Properties)]
pub struct GmModalProps {
    pub children: Children,
    pub visible: UseStateHandle<bool>,
    pub on_modal_close: Option<Callback<()>>,
}

#[function_component(GmModal)]
pub fn gm_modal(props: &GmModalProps) -> Html {
    let GmModalProps {
        children,
        visible,
        on_modal_close,
    } = props;

    let close_modal = {
        let is_modal_visiable = visible.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(false);
            if let Some(on_modal_close) = on_modal_close.clone() {
                on_modal_close.emit({});
            }
        })
    };

    if (*visible.clone()) == false {
        return html!(<></>);
    } else {
        return html! (
            <GmOverlay
                on_click_outside={close_modal.clone()}
            >
                <div class="modal-content">
                    <div class="modal-header">
                    <h5 class="modal-title" id="exampleModalLabel">{"Create a new markdown"}</h5>
                    </div>
                    <div class="modal-body">
                        { for children.iter() }
                    </div>
                    <div class="modal-footer">
                        <button
                            type="button"
                            class="btn btn-secondary"
                            data-dismiss="modal"
                            onclick={close_modal.clone()}
                        >{"Close"}</button>
                     </div>
                </div>
            </GmOverlay>
        );
    }
}
