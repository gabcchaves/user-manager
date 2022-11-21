use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{
    login::Login,
};

pub mod login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {
            <Login/>
        },
        Route::Register => html! {
            <h1>{"Register"}</h1>
        },
        Route::NotFound => html! {
            <h1>{"NotFound"}</h1>
        },
    }
}
