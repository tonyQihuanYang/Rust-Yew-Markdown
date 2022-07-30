use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Properties};

use crate::graphql_requests::fetch_delete_markdown::fetch_delete_markdown;

#[derive(PartialEq, Clone, Properties)]
pub struct ListItemDeleteButtonProps {
    pub markdown_id: String,
    pub on_delete: Callback<String>,
}

#[function_component(ListItemDeleteButton)]
pub fn list_item_delete_button(props: &ListItemDeleteButtonProps) -> Html {
    let ListItemDeleteButtonProps {
        markdown_id,
        on_delete,
    } = props;
    let is_modal_visiable = use_state(|| false);
    let hidden_class = get_visiable_class(&is_modal_visiable);

    let on_save = {
        let is_modal_visiable = is_modal_visiable.clone();
        let markdown_id = markdown_id.clone();
        let on_delete = on_delete.clone();
        Callback::from(move |_: MouseEvent| {
            let is_modal_visiable = is_modal_visiable.clone();
            let markdown_id = markdown_id.clone();
            let on_delete = on_delete.clone();

            spawn_local(async move {
                is_modal_visiable.set(false);
                fetch_delete_markdown(markdown_id.clone()).await.unwrap();
                on_delete.emit(markdown_id.clone());
            });
        })
    };

    let open_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(true);
        })
    };

    let close_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(false);
        })
    };

    html! {
        <>
            <button
                type="button"
                class="btn btn-outline-danger btn-sm"
                onclick={open_modal.clone()}
            >{"D"}</button>
            <div class={classes!("modal", hidden_class)}
                tabindex="-1"
                aria-hidden="true"
            >
              <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Are you sure you want to delete?"}</h5>
                    </div>
                    <div class="modal-footer">
                            <button
                            type="button"
                            class="btn btn-secondary"
                            data-dismiss="modal"
                            onclick={close_modal.clone()}
                        >{"Close"}</button>
                        <button
                            type="button"
                            class="btn btn-primary"
                            onclick={on_save.clone()}
                        >{"Delete"}</button>
                    </div>
                </div>
              </div>
            </div>

        </>


    }
}

fn get_visiable_class(is_visiable: &bool) -> String {
    if !(*is_visiable) {
        "hidden".to_string()
    } else {
        "visiable-block".to_string()
    }
}
