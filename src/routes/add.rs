use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Add;

pub enum AddMsg {
    Add,
}

impl Component for Add {
    type Message = AddMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="container-add">
                <button id="btn-return">
                    <img src="assets/img/bx-chevron-left.svg" alt="Return"/>
                </button>
                <h1>{"Add User"}</h1>
                <form id="form-add">
                    <input name="name" type="text" placeholder="Name"/>
                    <input type="submit" value="Add"/>
                </form>
            </section>
        }
    }
}
