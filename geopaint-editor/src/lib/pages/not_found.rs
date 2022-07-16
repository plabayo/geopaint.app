use yew::prelude::*;

use super::{page, Page};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    page(
        Page::NotFound,
        html! {
            <div>
                <h1>{ "404 — Not Found" }</h1>
            </div>
        },
    )
}
