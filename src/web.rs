use leptos::*;
use leptos_router::*;

mod index;
mod render;
mod utils;

use index::Index;
use render::Render;

use crate::env::{APP_NAME, APP_VERSION};

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
            <h1>{APP_NAME} {APP_VERSION}</h1>
            <div>
                <a href="/">Home</a>
                <a href="/about">About</a>
                <a href="./posts/assets">Assets</a>
            </div>
        </div>
        <Router>
            <Routes>
                <Route path="/" view=Index/>
                <Route path="/:name" view=Render/>
                <Route path="/about" view=About/>
            </Routes>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h1>"About Me "</h1>
            <p> I write software and like to write things. </p>
            <p> Try and find me on: </p>
            <ul>
                <li> <a href="https://github.com/amiller68">Github</a> </li>
                <li> <a href="https://twitter.com/lord_krondor">Twitter</a> </li>
            </ul>
        </div>
    }
}
