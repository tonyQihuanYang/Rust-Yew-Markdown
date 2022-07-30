use crate::graphql_requests::fetch_update_markdown::{fetch_update_markdown, MarkdownUpdateInput};
use crate::models::markdown::DEFAULT_ID;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    function_component, html, use_effect_with_deps, use_state, Callback, Properties, TargetCast,
};

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownTitleProps {
    pub markdown_title: String,
    pub markdown_id: String,
}

#[function_component(MarkdownTitle)]
pub fn markdown_title(props: &MarkdownTitleProps) -> Html {
    let MarkdownTitleProps {
        markdown_title,
        markdown_id,
    } = props;
    let title = use_state(|| markdown_title.clone());
    let is_form_dirty = use_state(|| true);

    use_effect_with_deps(
        {
            let title = title.clone();
            let markdown_title = markdown_title.clone();
            move |_| {
                title.set(markdown_title);
                || ()
            }
        },
        markdown_title.clone(), // dependents
    );

    let on_input = {
        let title = title.clone();
        let is_form_dirty = is_form_dirty.clone();
        Callback::from(move |input_event: InputEvent| {
            let input = input_event.target_unchecked_into::<HtmlInputElement>();
            title.set(input.value());
            is_form_dirty.set(false);
        })
    };

    let on_save_title = {
        let markdown_id = markdown_id.clone();
        let title = title.clone();
        let is_form_dirty = is_form_dirty.clone();
        Callback::from(move |_| {
            if markdown_id != DEFAULT_ID {
                let markdown_id = markdown_id.clone();
                let title = title.clone();
                let is_form_dirty = is_form_dirty.clone();
                spawn_local(async move {
                    log::info!("Update markdown title {} {}", markdown_id, *title);
                    fetch_update_markdown(MarkdownUpdateInput {
                        id: markdown_id,
                        title: Some((*title).to_string()),
                        context: None,
                    })
                    .await
                    .unwrap();
                    is_form_dirty.set(true);
                });
            }
        })
    };

    return html!(
        <div class="input-group mb-3">
            <input
                type="text"
                class="form-control" placeholder="Markdown name" aria-label="Recipient's username" aria-describedby="button-addon2"
                value={(*title).clone()}
                oninput={on_input.clone()}
            />
            <div class="input-group-append">
                <button
                    class="btn btn-outline-secondary"
                    type="button"
                    disabled={(*is_form_dirty).clone()}
                    onclick={on_save_title.clone()}
                >
                    {"Save"}
                </button>
            </div>
        </div>
    );
}
