use leptos::*;
use leptos_router::*;
use leptos_use::*;
use serde::{Deserialize, Serialize};

mod index;
mod render;
mod utils;

use index::Index;
use render::RenderItem;
use utils::{get_manifest, get_url};

use crate::env::{APP_NAME, APP_VERSION};
use crate::types::{Item, Manifest};

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
fn InternalRoute(query: String, msg: String) -> impl IntoView {
    let url = get_url().expect("url");
    let url = format!("{}/{}", url, query);
    let a_href_ref = create_node_ref::<leptos::html::A>();
    let _ = use_event_listener(
        a_href_ref,
        leptos::ev::click,
        move |_event: web_sys::MouseEvent| {
            let a_ref = a_href_ref.get().expect("a_ref");
            let url = a_ref.href();
            let window = web_sys::window().expect("window");
            window.location().set_href(&url).expect("href");
        },
    );

    view! {
        <a
            href=url
            ref=a_href_ref
        >
            {msg}
        </a>
    }
}

#[component]
fn WebApp() -> impl IntoView {
    view! {
        <Router>
            <h1>{APP_NAME} v{APP_VERSION}</h1>
            <nav>
                <InternalRoute query="".to_string() msg="Home".to_string()/>
                <InternalRoute query="?route=about".to_string() msg="About".to_string()/>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=InternalRouter/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum InternalRoute {
    Index(Manifest),
    About,
    RenderItem(Option<Item>),
}

impl IntoView for InternalRoute {
    fn into_view(self) -> View {
        match self {
            InternalRoute::Index(manifest) => Index(manifest).into_view(),
            InternalRoute::About => About.into_view(),
            InternalRoute::RenderItem(item) => RenderItem(item).into_view(),
        }
    }
}

fn match_route(manifest: Manifest, route: Option<String>, name: Option<String>) -> InternalRoute {
    logging::log!("route: {:?}", route);
    logging::log!("name: {:?}", name);
    match route {
        Some(route) => match route.as_str() {
            "posts" => match name {
                Some(name) => InternalRoute::RenderItem(manifest.item(&name)),
                _ => InternalRoute::Index(manifest),
            },
            "about" => InternalRoute::About,
            _ => InternalRoute::Index(manifest),
        },
        _ => InternalRoute::Index(manifest),
    }
}

#[component]
fn InternalRouter() -> impl IntoView {
    // Register inner components
    let (route, _) = create_query_signal::<String>("route");
    let (name, _) = create_query_signal::<String>("name");

    let component_view = create_resource(
        || (),
        move |_| async move {
            let route = route.get();
            let name = name.get();
            let manifest = get_manifest().await.expect("manifest");
            match_route(manifest, route, name)
        },
    );

    view! {
        <div>
            {move || match component_view.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(iv) => iv.into_view()
            }}
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct About;

impl IntoView for About {
    fn into_view(self) -> View {
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
        .into_view()
    }
}
