use yew::prelude::*;

use super::{page, Page};

#[function_component(Editor)]
pub fn editor() -> Html {
    page(
        Page::Editor,
        html! {
            <div>
                <h1>{ "Editor" }</h1>
                <p>{ "todo..." }</p>
            </div>
        },
    )
}
