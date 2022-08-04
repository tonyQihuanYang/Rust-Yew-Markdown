use crate::components::snacker::gm_snacker::GmSnacker;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct UpdateErrorNotificationProps {
    pub visible: UseStateHandle<bool>,
}

#[function_component(UpdateErrorNotification)]
pub fn update_error_notification(props: &UpdateErrorNotificationProps) -> Html {
    let UpdateErrorNotificationProps { visible } = props;
    html! {
        <GmSnacker
            title="Update Error"
            visible={visible.clone()}
        >
            <p>{"Error on updating markdown, please refresh your page. There could be new context updated in the current markdown."}</p>
        </GmSnacker>
    }
}
