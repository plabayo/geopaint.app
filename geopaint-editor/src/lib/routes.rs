use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editor")]
    Editor,
    #[at("/print")]
    Print,
    #[not_found]
    #[at("/404")]
    NotFound,
}
