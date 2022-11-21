use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Register;

pub enum RegisterMsg {
    Register,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="container-register">
                <h1>{"Administrator"}</h1>
                <form id="form-register">
                    <input name="name" type="text" placeholder="Name"/>
                    <input name="password" type="password" placeholder="Password"/>
                    <input name="password_confirmation" type="password" placeholder="Password"/>
                    <input type="submit" value="Register"/>
                </form>
                <Link<Route> to={Route::Login}>{"Already registered?"}</Link<Route>>
            </section>
        }
    }
}
