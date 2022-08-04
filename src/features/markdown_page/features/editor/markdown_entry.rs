use wasm_bindgen_futures::spawn_local;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Properties,
};

use super::markdown_features::markdown_editor::MarkdownEditor;
use super::markdown_features::markdown_preview::MarkdownPrview;
use super::markdown_features::markdown_title::MarkdownTitle;
use crate::{
    graphql_requests::fetch_markdown_by_id::fetch_markdown_by_id, models::markdown::Markdown,
};

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownEntryProps {
    pub selected_markdown: Option<Markdown>,
    pub is_preview_visiable: bool,
}

#[function_component(MarkdownEntry)]
pub fn markdown_entry(props: &MarkdownEntryProps) -> Html {
    let MarkdownEntryProps {
        selected_markdown,
        is_preview_visiable,
    } = props.clone();

    let markdown = use_state(|| Markdown::default());
    let selected_markdown = selected_markdown.clone();
    use_effect_with_deps(
        {
            let markdown = markdown.clone();
            let selected_markdown = selected_markdown.clone();
            move |_| {
                if let Some(current_markdown) = selected_markdown.clone() {
                    let markdown = markdown.clone();
                    spawn_local(async move {
                        match fetch_markdown_by_id(current_markdown.id).await {
                            Ok(res_markdown) => markdown.set(res_markdown),
                            Err(_) => log::error!("Could not load current markdown!"),
                        };
                    });
                }
                || ()
            }
        },
        selected_markdown.clone(),
    );

    let on_markdown_edit = {
        let markdown = markdown.clone();
        Callback::from(move |new_context: String| {
            markdown.set((*markdown).clone().update_context(new_context));
        })
    };

    let output_markdown_version_updated = {
        let markdown = markdown.clone();
        Callback::from(move |new_version: i64| {
            markdown.set((*markdown).clone().update_version(new_version));
        })
    };

    return html!(
        <>
            <MarkdownTitle
                markdown_id={markdown.id.clone()}
                markdown_version={markdown.version.clone()}
                markdown_title={markdown.title.clone()}
            />
            <div class="md-editor-wrapper">
                <div class="md-editor-textarea">
                    <MarkdownEditor
                        markdown_id={markdown.id.clone()}
                        markdown_version={markdown.version.clone()}
                        data={markdown.context.clone()}
                        on_edit={on_markdown_edit.clone()}
                        output_markdown_version_updated={output_markdown_version_updated.clone()}
                    />
                </div>

                if is_preview_visiable.clone() {
                        <div class={classes!("md-editor-preview", "mx-1", "border", "rounded-sm")}>
                            <MarkdownPrview data={markdown.context.clone()}/>
                        </div>
                }
            </div>
        </>
    );
}
