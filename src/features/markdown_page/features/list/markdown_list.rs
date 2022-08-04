use yew::use_state;
use yew::Properties;
use yew::{function_component, html, Callback, Html};

use crate::models::markdown::Markdown;

use super::markdown_list_features::list_item_delete_button::ListItemDeleteButton;

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownListProps {
    pub markdowns: Vec<Markdown>,
    pub output_markdown_selected: Callback<Markdown>,
}

#[function_component(MarkdownList)]
pub fn markdown_list(props: &MarkdownListProps) -> Html {
    let selected = use_state(|| "".to_string());
    let MarkdownListProps {
        output_markdown_selected,
        markdowns,
    } = props.clone();

    html! {
        <ul
            class="list-group"
        >
            { markdowns.iter().map(|markdown|
                html!{
                    <span
                        class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
                        onclick={
                            let markdown = markdown.clone();
                            let selected = selected.clone();
                            let output_markdown_selected=output_markdown_selected.clone();
                            Callback::from(move |_| {
                                selected.set(markdown.title.clone());
                                output_markdown_selected.emit(markdown.clone());
                            })
                        }
                    >
                        {markdown.title.clone()}
                        <ListItemDeleteButton
                            markdown_id={markdown.id.clone()}
                            on_delete={
                                let output_markdown_selected=output_markdown_selected.clone();
                                Callback::from(move |_| {
                                    output_markdown_selected.emit(Markdown::default())
                                })
                            }
                        />
                    </span>
                }
            ).collect::<Html>() }
        </ul>
    }
}
