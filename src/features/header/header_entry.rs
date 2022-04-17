use yew::prelude::*;

use crate::models::{markdown::Markdown, user_setting::Setting};

use super::header_features::{add_markdown_button::AddMarkdownButton, user_setting::UserSetting};

#[derive(PartialEq, Clone, Properties)]
pub struct HeaderEntryProps {
    pub default_user_setting: Setting,
    pub on_user_setting_update: Callback<Setting>,
    pub on_new_markdown: Callback<Markdown>,
}

#[function_component(HeaderEntryComponent)]
pub fn header_entry(props: &HeaderEntryProps) -> Html {
    let HeaderEntryProps {
        default_user_setting,
        on_user_setting_update,
        on_new_markdown,
    } = props;
    return html! {
        <div class="d-flex flex-row-reverse header-wrapper">
            <UserSetting
                user_setting={default_user_setting.clone()}
                on_update={on_user_setting_update.clone()}
            />
            <AddMarkdownButton
                on_new_markdown={on_new_markdown.clone()}
            />
        </div>
    };
}
