use yew::prelude::*;

use super::{page, Page};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    page(
        Page::NotFound,
        html! {
            <div>
                <h1>{ "404" }</h1>
                <h2>{ "Not Found" }</h2>
            </div>
        },
    )
}
