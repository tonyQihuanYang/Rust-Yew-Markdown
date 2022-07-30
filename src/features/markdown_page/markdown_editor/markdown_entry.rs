use crate::models::markdown::Markdown;
use yew::{classes, function_component, html, Callback, Properties};

use super::markdown_features::markdown_editor::MarkdownEditor;
use super::markdown_features::markdown_preview::MarkdownPrview;
use super::markdown_features::markdown_title::MarkdownTitle;

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownEntryProps {
    pub selected_markdown: Option<Markdown>,
    pub is_preview_visiable: bool,
    pub on_input: Callback<String>,
}

#[function_component(MarkdownEntry)]
pub fn markdown_entry(props: &MarkdownEntryProps) -> Html {
    let MarkdownEntryProps {
        on_input,
        selected_markdown,
        is_preview_visiable,
    } = props.clone();

    let markdown = match selected_markdown {
        Some(markdown) => markdown,
        None => Markdown::default(),
    };

    let on_markdown_edit = {
        Callback::from(move |input_string: String| {
            on_input.emit(input_string);
        })
    };

    return html!(
        <>
            <MarkdownTitle
                markdown_id={markdown.id.clone()}
                markdown_title={markdown.title.clone()}
            />
            <div class="md-editor-wrapper">
                <div class="md-editor-textarea">
                    <MarkdownEditor
                        id={markdown.id.clone()}
                        data={markdown.context.clone()}
                        on_edit={on_markdown_edit.clone()}
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