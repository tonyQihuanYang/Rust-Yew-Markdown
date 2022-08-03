use crate::{
    features::markdown_page::ui::update_error_notification::UpdateErrorNotification,
    graphql_requests::fetch_update_markdown::{fetch_update_markdown, MarkdownUpdateInput},
    models::markdown::DEFAULT_ID,
};
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub type OutputMarkdownUpdated = Callback<i64>;

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownEditorProps {
    pub markdown_id: String,
    pub markdown_version: i64,
    pub data: String,
    pub on_edit: Callback<String>,
    pub output_markdown_version_updated: OutputMarkdownUpdated,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let MarkdownEditorProps {
        markdown_id,
        markdown_version,
        data,
        on_edit,
        output_markdown_version_updated,
    } = props.clone();

    let update_error_visible = use_state(|| false);
    let is_mac = use_ref(|| is_running_on_mac());
    let timeout_id = use_state(|| -1);

    let oninput = {
        let markdown_id = markdown_id.clone();
        let output_markdown_version_updated = output_markdown_version_updated.clone();
        let update_error_visible = update_error_visible.clone();
        Callback::from(move |input_event: InputEvent| {
            let output_markdown_version_updated = output_markdown_version_updated.clone();
            let input = input_event.target_unchecked_into::<HtmlInputElement>();
            let markdown_version = markdown_version.clone();
            let context = input.value();
            let update_error_visible = update_error_visible.clone();
            on_edit.emit(context.clone());

            if markdown_id != DEFAULT_ID {
                cancel_existed_timeout(&*timeout_id);
                let timeout = set_update_timeout(
                    &markdown_id,
                    &markdown_version,
                    &context,
                    output_markdown_version_updated,
                    update_error_visible,
                    2_000,
                );
                timeout_id.set(timeout);
            }
        })
    };

    let onkeydown = {
        let markdown_id = markdown_id.clone();
        let markdown_version = markdown_version.clone();
        let data = data.clone();
        let output_markdown_version_updated = output_markdown_version_updated.clone();
        let is_mac = is_mac.clone();
        let update_error_visible = update_error_visible.clone();
        Callback::from(move |e: KeyboardEvent| {
            let output_markdown_version_updated = output_markdown_version_updated.clone();
            let update_error_visible = update_error_visible.clone();
            if *is_mac {
                if e.meta_key() && e.key_code() == 83 {
                    e.prevent_default();
                    update_markdown(
                        &markdown_id,
                        &markdown_version,
                        &data,
                        output_markdown_version_updated,
                        update_error_visible,
                    );
                }
            } else {
                if e.ctrl_key() && e.key_code() == 83 {
                    e.prevent_default();
                    update_markdown(
                        &markdown_id,
                        &markdown_version,
                        &data,
                        output_markdown_version_updated,
                        update_error_visible,
                    );
                }
            }
        })
    };

    html! {
        <>
            <textarea
                class="form-control"
                placeholder="Start from here..."
                value={data}
                {onkeydown}
                {oninput}
            />
            <UpdateErrorNotification visible={update_error_visible.clone()}/>
        </>
    }
}

fn is_running_on_mac() -> bool {
    let window = web_sys::window().unwrap();
    let platform = window.navigator().platform().unwrap();
    return platform.contains("Mac");
}

fn cancel_existed_timeout(timeout_id: &i32) {
    let window = web_sys::window().unwrap();
    window.clear_timeout_with_handle(*timeout_id);
}

fn set_update_timeout(
    markdown_id: &String,
    markdown_version: &i64,
    context: &String,
    output_markdown_version_updated: OutputMarkdownUpdated,
    update_error_visible: UseStateHandle<bool>,
    timeout_millis: u32,
) -> i32 {
    let markdown_id = markdown_id.clone();
    let markdown_version = markdown_version.clone();
    let context = context.clone();
    let output_markdown_version_updated = output_markdown_version_updated.clone();
    let update_error_visible = update_error_visible.clone();
    Timeout::new(timeout_millis, move || {
        update_markdown(
            &markdown_id,
            &markdown_version,
            &context,
            output_markdown_version_updated,
            update_error_visible,
        );
    })
    .forget()
}

fn update_markdown(
    markdown_id: &String,
    markdown_version: &i64,
    context: &String,
    output_markdown_version_updated: OutputMarkdownUpdated,
    update_error_visible: UseStateHandle<bool>,
) {
    let markdown_id = markdown_id.clone();
    let markdown_version = markdown_version.clone();
    let context = context.clone();
    let output_markdown_version_updated = output_markdown_version_updated.clone();
    let update_error_visible = update_error_visible.clone();
    spawn_local(async move {
        match fetch_update_markdown(MarkdownUpdateInput {
            id: markdown_id.to_string(),
            version: markdown_version,
            context: Some(context),
            title: None,
        })
        .await
        {
            Ok(updated) => {
                output_markdown_version_updated.emit(updated.version);
            }
            Err(error) => {
                update_error_visible.set(true);
                log::error!("ERROR => Updated Failed");
            }
        };
    });
}
