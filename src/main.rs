use yew::prelude::*;
use yew_router::prelude::*;
use user_manager::routes::Route;
use user_manager::routes::switch;
use user_manager::components::{
    header::Header,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header/>
            <div id="main-container">
                <Switch<Route> render={Switch::render(switch)}/>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
