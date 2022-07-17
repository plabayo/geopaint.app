use yew::prelude::*;

use super::{page, Page};
use crate::{browser::navigator, canvas::Canvas};

#[derive(Default)]
pub struct Editor {
    state: EditorState,
}

#[derive(Default)]
struct EditorState {
    use_mobile_editor: bool,
}

pub enum Msg {
    UseMobileEditor,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UseMobileEditor => {
                self.state.use_mobile_editor = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.state.use_mobile_editor && navigator::is_mobile() {
            self.view_mobile(ctx)
        } else {
            self.view_desktop(ctx)
        }
    }
}

impl Editor {
    fn view_mobile(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().batch_callback(|_| Some(Msg::UseMobileEditor));
        page(
            Page::Editor,
            html! {
                <div>
                    <h1>{ "Editor" }</h1>
                    <p>{ "We do not support the editor on mobile devices for now." }</p>
                    <p>{ "This because the User-Experience would be suboptimal, given the lack of support on our end." }</p>
                    <p>
                        { "You can join " }
                        <a href="https://github.com/plabayo/geopaint.app/discussions/2">
                            { "our discussion at GitHub" }
                        </a>
                        { " should you wish to request support for this, have ideas regarding it or would like to contribute practically or by financial means." }
                    </p>
                    <p>{ "You can never the less use the editor on your mobile device by pressing the following button:" }</p>
                    <p>
                        <button {onclick}>{ "Use Mobile Editor" }</button>
                    </p>
                    <p>{ "By pressing this button you agree that the experience might be below expectations." }</p>
                    <p>{ "For an optimal experience we suggest you to use the editor on a desktop device for the time being." }</p>
                </div>
            },
        )
    }

    pub fn view_desktop(&self, _ctx: &Context<Self>) -> Html {
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
}
