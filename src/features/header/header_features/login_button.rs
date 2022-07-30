use yew::prelude::*;

use crate::components::button::GmButton;
use crate::components::modal::modal::GmModal;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
// use material_yew::MatButton;

#[function_component(LoginButton)]
pub fn login_button() -> Html {
    let is_modal_visiable = use_state(|| false);
    let on_click = {
        let is_modal_visiable = is_modal_visiable.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_visiable.set(true);
        })
    };
    let login = {
        Callback::from(move |_: ()| spawn_local(async move {}));
    };
    return html! {
       <>
           <GmButton title="Login"
           on_click={on_click}

           ></GmButton>

           <GmModal
              visible={is_modal_visiable}
           >
               <LoginForm/>
           </GmModal>

       </>
    };
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    return html! {
        <>
            <form>
                <label for="fname"> {"First name:"} </label>
                <input type="text" id="fname" name="fname"/>
                <label for="lname"> {"Last name:"} </label>
                <input type="text" id="lname" name="lname"/>
                <input type="submit" value="Submit"/>
            </form>
        </>
    };
}

// use serde::Deserialize;
// #[derive(Clone, PartialEq, Deserialize)]
// struct Video {
//     id: usize,
//     title: String,
//     speaker: String,
//     url: String,
// }

// pub async fn login() {
//     let fetched_videos = Request::get("http//localhost:1338/login")
//         .send()
//         .await
//         .unwrap()
//         .json()
//         .await
//         .unwrap();
// }
