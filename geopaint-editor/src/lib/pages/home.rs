use yew::prelude::*;

use super::{page, Page};

#[function_component(Home)]
pub fn home() -> Html {
    page(
        Page::Home,
        html! {
            <div>
                <h1>{ "Welcome" }</h1>
                <p>{ "An editor which allows you to paint using geometric shapes..." }</p>
                <p>{ "todo..." }</p>
                <h2>{ "WIP" }</h2>
            </div>
        },
    )
}
