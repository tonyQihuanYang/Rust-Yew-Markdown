use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    graphql_requests::fetch_create_markdown::{fetch_create_new_markdown, MarkdownInput},
    models::markdown::Markdown,
};

#[derive(PartialEq, Clone, Properties)]
pub struct AddMarkdownButtonProps {
    pub on_new_markdown: Callback<Markdown>,
}

#[function_component(AddMarkdownButton)]
pub fn add_markdown_button(props: &AddMarkdownButtonProps) -> Html {
    let AddMarkdownButtonProps { on_new_markdown } = props;
    let new_markdown_title = use_state(|| "".to_string());
    let is_modal_visiable = use_state(|| false);
    let hidden_class = if !(*is_modal_visiable) {
        "hidden"
    } else {
        "visiable-block"
    };

    let on_save = {
        let is_modal_visiable = is_modal_visiable.clone();
        let new_markdown_title = new_markdown_title.clone();
        let on_new_markdown = on_new_markdown.clone();
        Callback::from(move |_: MouseEvent| {
            let is_modal_visiable = is_modal_visiable.clone();
            let new_markdown_title = new_markdown_title.clone();
            let on_new_markdown = on_new_markdown.clone();
            spawn_local(async move {
                let res = fetch_create_new_markdown(MarkdownInput {
                    title: ((*new_markdown_title).clone()).to_string(),
                    context: "".to_string(),
                })
                .await
                .unwrap();
                is_modal_visiable.set(false);
                new_markdown_title.set("".to_string());
                on_new_markdown.emit(res);
            });
        })
    };

    let oninput = {
        let new_markdown_title = new_markdown_title.clone();
        Callback::from(move |input_event: InputEvent| {
            let input = input_event.target_unchecked_into::<HtmlInputElement>();
            new_markdown_title.set(input.value());
        })
    };

    let open_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        let new_markdown_title = new_markdown_title.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(true);
            new_markdown_title.set("".to_string());
        })
    };

    let close_modal = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(false);
        })
    };

    return html! (
        <>

            <button
                type="button"
                class="btn btn-primary"
                data-toggle="modal"
                data-target="#exampleModal"
                onclick={open_modal.clone()}
            >
                {"Create ➕"}
            </button>

            <div
                class={classes!("modal", hidden_class)}
                tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-centered" role="document">
                <div class="modal-content">
                    <div class="modal-header">
                    <h5 class="modal-title" id="exampleModalLabel">{"Create a new markdown"}</h5>
                    // <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                    //     <span aria-hidden="true">{"'&times;'"}</span>
                    // </button>
                    </div>
                    <div class="modal-body">
                        <input
                            class="form-control"
                            value={(*new_markdown_title).clone()}
                            oninput={oninput.clone()}
                        />
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
                        >{"Save changes"}</button>
                    </div>
                </div>
                </div>
            </div>
        </>
    );
}
