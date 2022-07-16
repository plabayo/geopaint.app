use yew::prelude::*;

mod editor;
pub use editor::Editor;

mod print;
pub use print::Print;

mod not_found;
pub use not_found::NotFound;

#[derive(PartialEq)]
enum Page {
    Editor,
    Print,
    NotFound,
}

fn page(page: Page, content: Html) -> Html {
    html! {
        <div id="wrapper">
            <header>
                <h2>{ "Geopaint App" }</h2>
                <nav id="site-nav-main">
                    <div class="nav-buttons">
                        <a class="nav-button" href="/">
                            <div class={ if page == Page::Editor { "button-selected" } else { "" } }>
                                <span>{ "Editor" }</span>
                            </div>
                        </a>
                        <a class="nav-button" href="/print">
                            <div class={ if page == Page::Print { "button-selected" } else { "" } }>
                                <span>{ "Print" }</span>
                            </div>
                        </a>
                    </div>
                </nav>
            </header>
            <main>
                <div id="content">
                    { content }
                </div>
            </main>
            <footer>
                <div id="nav-footer">
                    <p>
                        { "Website made with ♥ by " }
                        <a href="https://plabayo.tech/" rel="nofollow">{ "Plabayo" }</a>
                        { "." }
                    </p>
                </div>
            </footer>
        </div>
    }
}