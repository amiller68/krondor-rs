use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::env::{APP_NAME, APP_VERSION};
use crate::types::Manifest;
use crate::utils::web::get_manifest;
use crate::web::components::InternalLink;

// This router is an attempt to make SPAs easy
// Register and use pages here

mod index;
mod items;

use index::IndexPage;
use items::ItemsPage;

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
                "items" => ItemsPage::from_ctx(self),
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
                <InternalLink query="?route=items".to_string() msg="Blog".to_string()/>
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
