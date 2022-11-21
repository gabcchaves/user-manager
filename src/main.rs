use yew::prelude::*;
use yew_router::prelude::*;
use user_manager::routes::Route;
use user_manager::routes::switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div id="main-container">
                <Switch<Route> render={Switch::render(switch)}/>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
