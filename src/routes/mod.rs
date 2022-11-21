use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{
    login::Login,
    register::Register,
    users::Users,
    add::Add,
};

pub mod login;
pub mod register;
pub mod users;
pub mod add;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/users")]
    Users,
    #[at("/add")]
    Add,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {
            <Login/>
        },
        Route::Register => html! {
            <Register/>
        },
        Route::Users => html!{
            <Users/>
        },
        Route::Add => html! {
            <Add/>
        },
        Route::NotFound => html! {
            <h1>{"NotFound"}</h1>
        },
    }
}
