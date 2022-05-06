use yew::prelude::*;
use yew::{function_component, Callback};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/add")]
    Add,
    #[at("/edit/:id")]
    Edit { id: u32 },
}
