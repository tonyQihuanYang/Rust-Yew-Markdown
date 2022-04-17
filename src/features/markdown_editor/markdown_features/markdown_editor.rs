use crate::{
    graphql_requests::fetch_update_markdown::{fetch_update_markdown, MarkdownUpdateInput},
    models::markdown::DEFAULT_ID,
};
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct MarkdownEditorProps {
    pub id: String,
    pub data: String,
    pub on_edit: Callback<String>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let MarkdownEditorProps { id, data, on_edit } = props.clone();
    let is_mac = use_ref(|| is_running_on_mac());
    let timeout_id = use_state(|| -1);

    let oninput = {
        let id = id.clone();
        Callback::from(move |input_event: InputEvent| {
            let input = input_event.target_unchecked_into::<HtmlInputElement>();
            let context = input.value();
            on_edit.emit(context.clone());

            if id != DEFAULT_ID {
                cancel_existed_timeout(&*timeout_id);
                let timeout = set_update_timeout(&id, &context, 2_000);
                timeout_id.set(timeout);
            }
        })
    };

    let onkeydown = {
        let id = id.clone();
        let data = data.clone();
        let is_mac = is_mac.clone();
        Callback::from(move |e: KeyboardEvent| {
            if *is_mac {
                if e.meta_key() && e.key_code() == 83 {
                    e.prevent_default();
                    update_markdown(&id, &data);
                }
            } else {
                if e.ctrl_key() && e.key_code() == 83 {
                    e.prevent_default();
                    update_markdown(&id, &data);
                }
            }
        })
    };

    html! {
        <textarea
        class="form-control"
        placeholder="Start from here..."
        value={data}
        {onkeydown}
        {oninput}
    />
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

fn set_update_timeout(id: &String, context: &String, timeout_millis: u32) -> i32 {
    let id = id.clone();
    let context = context.clone();
    Timeout::new(timeout_millis, move || {
        update_markdown(&id, &context);
    })
    .forget()
}

fn update_markdown(id: &String, context: &String) {
    let id = id.clone();
    let context = context.clone();
    spawn_local(async move {
        fetch_update_markdown(MarkdownUpdateInput {
            id: id.to_string(),
            context: Some(context),
            title: None,
        })
        .await
        .unwrap();
        log::info!("Save success...");
    });
}
