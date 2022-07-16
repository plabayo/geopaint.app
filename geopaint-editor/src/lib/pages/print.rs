use yew::prelude::*;

use super::{page, Page};

#[function_component(Print)]
pub fn print() -> Html {
    page(
        Page::Print,
        html! {
            <div>
                <h1>{ "Print" }</h1>
                <p>{ "todo..." }</p>
            </div>
        },
    )
}
