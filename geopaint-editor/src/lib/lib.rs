use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod routes;

use pages::{Editor, Print, NotFound};
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
        Route::Editor => html! { <Editor /> },
        Route::Print => html! { <Print /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
