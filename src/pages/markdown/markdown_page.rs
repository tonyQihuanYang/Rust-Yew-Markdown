use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::{function_component, html, use_state, Callback, TargetCast};

use crate::{
    features::{
        header::header_entry::HeaderEntryComponent, markdown_editor::markdown_entry::MarkdownEntry,
        markdown_list::markdown_list::MarkdownList,
    },
    graphql_requests::{
        fetch_markdown_by_id::fetch_markdown_by_id, fetch_markdowns::fetch_markdowns,
    },
    models::{
        markdown::{Markdown, DEFAULT_ID},
        user_setting::Setting,
    },
};

#[function_component(MarkdownPage)]
pub fn markdown_page() -> Html {
    let user_setting = use_state(|| Setting::new());
    let markdowns = use_state(|| Vec::<Markdown>::new());
    let markdown_selected = use_state(|| Markdown::default());
    let drawer_toggled = use_state(|| false);

    let on_user_setting_update = {
        let user_setting = user_setting.clone();
        Callback::from(move |setting: Setting| {
            log::info!("Detect user setting updated {:?}", setting);
            user_setting.set(setting);
        })
    };

    let on_markdown_edit = {
        let markdown_selected = markdown_selected.clone();
        Callback::from(move |input_string: String| {
            markdown_selected.set((*markdown_selected).clone().update_context(input_string));
        })
    };

    let on_markdown_selected = {
        let markdown_selected = markdown_selected.clone();
        let drawer_toggled = drawer_toggled.clone();
        Callback::from(move |selected: Markdown| {
            markdown_selected.set(selected.clone());
            drawer_toggled.set(false);
            if selected.id != DEFAULT_ID {
                let markdown_selected = markdown_selected.clone();
                let selected = selected.clone();
                spawn_local(async move {
                    let resposnse = fetch_markdown_by_id(selected.id).await.unwrap();
                    markdown_selected.set(resposnse);
                });
            } else {
                markdown_selected.set(selected);
            }
        })
    };

    let on_drawer_toggled = {
        let markdowns = markdowns.clone();
        let drawer_toggled = drawer_toggled.clone();
        Callback::from(move |e: InputEvent| {
            drawer_toggled.set(!(*drawer_toggled));
            let checkbox = e.target_unchecked_into::<HtmlInputElement>();
            if checkbox.checked() {
                let markdowns = markdowns.clone();
                spawn_local(async move {
                    let response_markdowns = fetch_markdowns().await.unwrap();
                    if !response_markdowns.is_empty() {
                        markdowns.set(response_markdowns);
                    }
                });
            }
        })
    };

    return html! {
        <>
        <input
            type="checkbox"
            id="drawer-toggle"
            name="drawer-toggle"
            checked={(*drawer_toggled).clone()}
            oninput={on_drawer_toggled.clone()}
        />
        <label for="drawer-toggle" id="drawer-toggle-label"></label>
        <header>
            <HeaderEntryComponent
                default_user_setting={(*user_setting).clone()}
                on_user_setting_update={on_user_setting_update}
                on_new_markdown={on_markdown_selected.clone()}
            />
        </header>
        <div id="drawer">
        <MarkdownList
            markdowns={(*markdowns).clone()}
            on_select={on_markdown_selected.clone()}
        />
        </div>
        <div id="page-content">
                if ((*markdown_selected).clone()).id == DEFAULT_ID {
                    <EmptyContent></EmptyContent>
                }
                else {
                    <MarkdownEntry
                        on_input={on_markdown_edit.clone()}
                        selected_markdown={(*markdown_selected).clone()}
                        is_preview_visiable={(*user_setting).clone().is_show_preview}
                    />
                }
        </div>
        </>
    };
}

#[function_component(EmptyContent)]
pub fn empty_content() -> Html {
    html! {
        <div class="no-content-wrapper">
            // <img src="img/avatar.png"
            // class="rounded mx-auto d-block p-2"
            // width="300" height="300"
            // />
            <h5 style="font-size: 260px;">{"🤔"}</h5>
            <h5>{"Nahh! No markdown selected/found..."}</h5>
        </div>
    }
}
