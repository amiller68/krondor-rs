use leptos::*;
use leptos_router::*;
use leptos_use::*;
use serde::{Deserialize, Serialize};

use crate::env::{APP_NAME, APP_VERSION};
use crate::types::Manifest;
use crate::utils::web::{get_manifest, get_url};

// This router is an attempt to make SPAs easy
// Register and use pages here

mod about;
mod index;
mod item;

use about::AboutPage;
use index::IndexPage;
use item::ItemPage;

/// A Shared page context to pass to all pages
#[derive(Clone, Serialize, Deserialize)]
pub struct PageContext {
    manifest: Manifest,
    route: Option<String>,
    query: Option<String>,
}

impl PageContext {
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
    pub fn route(&self) -> &Option<String> {
        &self.route
    }
    pub fn query(&self) -> &Option<String> {
        &self.query
    }
}

impl IntoView for PageContext {
    fn into_view(self) -> View {
        let page: Box<dyn Page> = match self.route() {
            Some(route) => match route.as_str() {
                "items" => ItemPage::from_ctx(self),
                "about" => AboutPage::from_ctx(self),
                _ => IndexPage::from_ctx(self),
            },
            _ => IndexPage::from_ctx(self),
        };
        page.into_view_ref()
    }
}

/// Trait object for passing page views to the router
pub trait Page: Send + Sync {
    fn ctx(&self) -> &PageContext;
    fn from_ctx(ctx: PageContext) -> Box<dyn Page>
    where
        Self: Sized;
    fn into_view_ref(&self) -> View;
}

#[component]
pub fn InternalRouter() -> impl IntoView {
    view! {
        <Router>
            <h1>{APP_NAME} v{APP_VERSION}</h1>
            <nav>
                <InternalLink query="".to_string() msg="Home".to_string()/>
                <InternalLink query="?route=about".to_string() msg="About".to_string()/>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=PageRoute/>
                </Routes>
            </main>
        </Router>
    }
}

/// An internal router should use the context to render a page
#[component]
fn PageRoute() -> impl IntoView {
    let (route, _) = create_query_signal::<String>("route");
    let (query, _) = create_query_signal::<String>("query");

    let ctx = create_resource(
        || (),
        move |_| async move {
            let route = route.get();
            let query = query.get();
            let manifest = get_manifest().await.expect("manifest");

            let ctx = PageContext {
                manifest,
                route,
                query,
            };

            ctx
        },
    );

    view! {
        <div>
            {move || match ctx.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(c) => c.into_view()
            }}
        </div>
    }
}

#[component]
fn InternalLink(query: String, msg: String) -> impl IntoView {
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
