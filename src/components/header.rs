use yew::prelude::*;
use yew_router::prelude::*;

pub struct Header {
    pub user_logged: bool,
}

pub enum HeaderMsg {
    Exit,
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user_logged: false,
        }
    }

    // TODO: fn update()
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <header id="main-header">
                <h1>{"User Manager"}</h1>
                if self.user_logged {
                    <button id="btn-exit">
                        <img src="assets/img/bx-exit.svg" alt="Exit"/>
                    </button>
                }
            </header>
        }
    }
}
