use yew::prelude::*;
use yew_router::prelude::*;

mod browser;
mod canvas;
mod pages;
mod routes;

use pages::{Editor, Home, NotFound, Print};
use routes::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Editor => html! { <Editor /> },
        Route::Print => html! { <Print /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
