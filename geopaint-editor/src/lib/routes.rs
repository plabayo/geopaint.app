use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Editor,
    #[at("/print")]
    Print,
    #[not_found]
    #[at("/404")]
    NotFound,
}
