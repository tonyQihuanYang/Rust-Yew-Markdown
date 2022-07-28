use yew::prelude::*;
use super::overlay::overlay::GmOverlay;
#[derive(PartialEq, Clone, Properties)]
pub struct GmModalProps {
    pub visible: bool,
    pub handle_close: Callback<()>,
    pub children: Children,
}

#[function_component(GmModal)]
pub fn gm_modal(props: &GmModalProps) -> Html {
    let GmModalProps {visible, children, handle_close} = props;
    let is_modal_visiable = use_state(|| visible.clone());

    use_effect_with_deps(
        {
            let is_modal_visiable = is_modal_visiable.clone();
            let visible = visible.clone();
        move |_| {
            is_modal_visiable.set(visible);
            || ()
        }},
        visible.clone() // dependents
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

    return html! (
        <GmOverlay>
            <div
                class={classes!("modal", hidden_class)}
                tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-centered" role="document">
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
                </div>
            </div>
        </GmOverlay>
    );
}

