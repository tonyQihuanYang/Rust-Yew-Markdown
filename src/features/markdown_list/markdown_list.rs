use yew::use_state;
use yew::Properties;
use yew::{function_component, html, Callback, Html};

use crate::models::markdown::Markdown;

use super::markdown_list_features::list_item_delete_button::ListItemDeleteButton;

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownListProps {
    pub markdowns: Vec<Markdown>,
    pub on_select: Callback<Markdown>,
}

#[function_component(MarkdownList)]
pub fn markdown_list(props: &MarkdownListProps) -> Html {
    let selected = use_state(|| "".to_string());
    let MarkdownListProps {
        on_select,
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
                            let on_select = on_select.clone();
                            Callback::from(move |_| {
                                selected.set(markdown.title.clone());
                                on_select.emit(markdown.clone());
                            })
                        }
                    >
                        {markdown.title.clone()}
                        <ListItemDeleteButton
                            markdown_id={markdown.id.clone()}
                            on_delete={
                                let on_select = on_select.clone();
                                Callback::from(move |_| {
                                    on_select.emit(Markdown::default())
                                })
                            }
                        />
                    </span>
                }
            ).collect::<Html>() }
        </ul>
    }
}
