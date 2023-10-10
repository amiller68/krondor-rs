use async_trait::async_trait;
use leptos::*;
use leptos_router::*;
use leptos_struct_table::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

mod index;
mod render;
mod utils;

use index::Index;
use render::Render;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        console_error_panic_hook::set_once();
        leptos::mount_to_body(WebApp)
    }
}

#[component]
fn WebApp() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, World!"</h1>
            
        </div>
        <Router>
            <Routes>
                <Route path="/" view=Index/>
                <Route path="/:cid" view=Render/>
            </Routes>
        </Router>
    }
}
