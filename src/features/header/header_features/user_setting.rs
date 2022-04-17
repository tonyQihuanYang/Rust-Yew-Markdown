use std::any::Any;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::models::user_setting::Setting;

#[derive(PartialEq, Clone, Properties)]
pub struct UserSettingProps {
    pub user_setting: Setting,
    pub on_update: Callback<Setting>,
}

#[function_component(UserSetting)]
pub fn user_setting(props: &UserSettingProps) -> Html {
    let UserSettingProps {
        user_setting,
        on_update,
    } = props;
    let is_dropdown_visiable = use_state(|| false);

    let onchange = {
        let on_update = on_update.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let is_checked = input.checked();
            on_update.emit(Setting {
                is_show_preview: is_checked,
            });
        }
    };

    let on_click_dropdown = {
        let is_dropdown_visiable = is_dropdown_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_dropdown_visiable.set(!(*is_dropdown_visiable));
            // let element = e.target_unchecked_into::<HtmlInputElement>();
            // element.dropdown();
        })
    };

    html! {
        <div class="dropdown">
            <button
                class="btn btn-primary dropdown-toggle"
                type="button"
                id="dropdownMenuButton"
                data-toggle="dropdown"
                aria-expanded="false"
                onclick={on_click_dropdown.clone()}
            >
              {"🥷🏽"}
            </button>
            <div
                class={
                    classes!(
                        "dropdown-menu",
                        "dropdown-menu-right",
                        get_visiable_class(&is_dropdown_visiable)
                    )}
                    aria-labelledby="dropdownMenuButton"
                >
                    // <div class="dropdown-item">
                    <form class="px-4 py-3">
                        <div class="form-group">
                            <div class="form-check">
                                <input
                                    {onchange}
                                    checked={(user_setting).clone().is_show_preview}
                                    type="checkbox"
                                    class="form-check-input"
                                />
                                <label class="form-check-label"
                                    for="customCheck1"
                                    id="customCheck1"
                                >{"Preview"}</label>
                            </div>
                        </div>
                    </form>
            </div>
        </div>
    }
}

fn get_visiable_class(is_dropdown_visiable: &bool) -> String {
    if *is_dropdown_visiable {
        return "show".to_string();
    } else {
        return "".to_string();
    }
}