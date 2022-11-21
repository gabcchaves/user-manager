use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Login;

pub enum LoginMsg {
    Log,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="container-login">
                <form id="login-form">
                    <input name="id" type="number" placeholder="ID"/>
                    <input name="password" type="password" placeholder="Password"/>
                    <input type="submit" value="Log In"/>
                </form>
                <Link<Route> to={Route::Register}>{"Not registered?"}</Link<Route>>
            </section>
        }
    }
}
