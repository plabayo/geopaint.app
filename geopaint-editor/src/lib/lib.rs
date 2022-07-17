// Plabayo — geopaint.app
// Copyright (C) 2022 Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
