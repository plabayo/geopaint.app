use yew::prelude::*;

use super::{page, Page};
use crate::{browser::navigator, canvas::Canvas};

#[function_component(Editor)]
pub fn editor() -> Html {
    if navigator::is_mobile() {
        editor_mobile()
    } else {
        editor_desktop()
    }
}

pub fn editor_mobile() -> Html {
    page(
        Page::Editor,
        html! {
            <div>
                <h1>{ "Editor" }</h1>
                <p>{ "We do not support the editor on mobile devices for now." }</p>
                <p>{ "This because the User-Experience would be suboptimal, given the lack of support on our end." }</p>
                <p>
                    { "Feel free to open " }
                    <a href="https://github.com/plabayo/geopaint.app/issues/new/choose">
                        { "a proposal at GitHub" }
                    </a>
                    { "should you wish to request support for this." }
                </p>
            </div>
        },
    )
}

pub fn editor_desktop() -> Html {
    page(
        Page::Editor,
        html! {
            <div>
                <h1>{ "Editor" }</h1>
                <p>{ "todo..." }</p>
                <div>
                    <Canvas />
                </div>
            </div>
        },
    )
}
