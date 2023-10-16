use leptos::*;
use leptos_router::*;

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
                <Route path="/:name" view=Render/>
            </Routes>
        </Router>
    }
}
