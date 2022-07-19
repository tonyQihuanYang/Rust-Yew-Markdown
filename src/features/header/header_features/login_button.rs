use yew::prelude::*;

use crate::components::button::GmButton;
use crate::components::modal::GmModal;

 #[function_component(LoginButton)]
 pub fn login_button() -> Html {

    let is_modal_visiable = use_state(|| false);
    let on_click = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
             is_modal_visiable.set(true);
        })
    };
   let handle_close = {
       let is_modal_visiable = is_modal_visiable.clone();
       Callback::from(move |_| {
            is_modal_visiable.set(false);
        })
   };

     return html! {
        <>
            <GmButton title="test"
            on_click={on_click}

            ></GmButton>

            <GmModal
               visible={*is_modal_visiable}
               handle_close={handle_close} 
            >
            {"xxxx"}
            </GmModal>    

        </>
     };
 }


