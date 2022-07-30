use material_yew::MatTextField;
use yew::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    return html! {
        <>
            <form>
                // <MatTextField outlined=true helper="Helper Text" />
                <label for="username"> {"user name:"} </label>
                <input type="text" id="username" name="username"/>
                <label for="password"> {"password"} </label>
                <input type="text" id="password" name="password"/>
                <input type="submit" value="Login"/>
            </form>
        </>
    };
}
