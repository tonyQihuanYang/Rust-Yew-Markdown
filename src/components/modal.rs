use super::overlay::overlay::GmOverlay;
use yew::prelude::*;
#[derive(PartialEq, Clone, Properties)]
pub struct GmModalProps {
    pub visible: bool,
    pub handle_close: Callback<()>,
    pub children: Children,
}

#[function_component(GmModal)]
pub fn gm_modal(props: &GmModalProps) -> Html {
    let GmModalProps {
        visible,
        children,
        handle_close,
    } = props;
    let is_modal_visiable = use_state(|| visible.clone());

    use_effect_with_deps(
        {
            let is_modal_visiable = is_modal_visiable.clone();
            let visible = visible.clone();
            move |_| {
                is_modal_visiable.set(visible);
                || ()
            }
        },
        visible.clone(), // dependents
    );

    let hidden_class = if !(*is_modal_visiable) {
        "hidden"
    } else {
        "visiable-block"
    };
    let open_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(true);
        })
    };

    let close_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        let handle_close = handle_close.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(false);
            handle_close.emit({});
        })
    };

    if *is_modal_visiable.clone() {
        return html! (
            <GmOverlay
                on_close={handle_close.clone()}
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
    } else {
        return html! (<></>);
    }
}
