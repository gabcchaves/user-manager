use yew::prelude::*;

pub struct Users;

pub enum UsersMsg {
    Add,
}

impl Component for Users {
    type Message = UsersMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="container-users">
                <h1>{"Users"}</h1>
                <button id="btn-add-user">
                    <img src="assets/img/bx-plus-circle.svg" alt="Add"/>
                </button>
                // User list
            </section>
        }
    }
}
