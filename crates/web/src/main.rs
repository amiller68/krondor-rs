use std::convert::{TryFrom, From};

use leptos::*;
use leptos_struct_table::*;
use leptos_dom::Text;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use leptos_router::*;
use async_trait::async_trait;
use pulldown_cmark::{html, Options, Parser};
use cid::Cid as BaseCid;

mod config;
mod error;
use config::KrondorConfig;
use error::{WebError, WebResult};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cid(BaseCid);

impl Cid {
    fn to_string(&self) -> String {
        self.0.to_string()
        
    }
}

impl From<Cid> for BaseCid {
    fn from(cid: Cid) -> Self {
        cid.0
    }
}

impl IntoView for Cid {
    fn into_view(self, cx: leptos::Scope) -> View {
        let text = self.to_string();
        let text = Text::new(text.into());
        text.into_view(cx)
    }
}

impl Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Cid(BaseCid::try_from(s).unwrap()))
    }
}

// This generates the component PersonTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct Post {
    #[table(key)]
    pub(crate) cid: Cid,
    #[table(skip)]
    name: String,
    title: String,
    date: String,
}

fn config() -> KrondorConfig {
    KrondorConfig::new().unwrap()
}

async fn get_posts() -> WebResult<Vec<Post>> {
    let config = config();
    let root_cid = config.root_cid;
    let gateway = config.gateway;

    let root_cid = root_cid.get()
        .await
        .map_err(|_| WebError::msg("get_posts(): couldn't get cid"))?;

    let manifest_cid = format!("{}/manifest.json", root_cid);
    let manifest = gateway.get(&manifest_cid).await.unwrap();
    let manifest = serde_json::from_str::<serde_json::Value>(&manifest).unwrap();
    let posts = manifest["posts"].as_array().unwrap();
    let posts = posts
        .iter()
        .map(| post| {
            serde_json::from_value::<Post>(post.clone())
                .map_err(|_| WebError::msg("get_posts(): couldn't parse manifest")).unwrap()
        })
        .collect::<Vec<Post>>();
    Ok(posts)
}

async fn get_post(cid: String) -> String {
    let config = KrondorConfig::new().unwrap();
    let gateway = config.gateway;
    let post = gateway.get(&cid).await.unwrap();
    post
}

fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

fn index(cx: Scope) -> impl IntoView {
    let items = create_rw_signal(cx, vec![]);

    let posts = create_resource(cx, || (), move |_| async move {
        let posts = get_posts().await.unwrap();
        items.set(posts);
    });
    view! { cx,
        <div>
            {move || match posts.read(cx) {
                None => view! {cx,  <p>"Loading..."</p> }.into_view(cx),
                Some(_) => view! {cx, <PostTable items=items/>}.into_view(cx)
            }}
        </div>
    }
}

fn post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let post = create_resource(cx, || (), move |_| async move {
        let cid = move || params.with(|params| params.get("cid").cloned().unwrap_or_default());
        get_post(cid()).await
    });
    view! { cx,
        <div>
            {move || match post.read(cx) {
                None => view! {cx,  <p>"Loading..."</p> }.into_view(cx),
                Some(data) => view! {cx, <div class="prose max-w-none" inner_html=markdown_to_html(data)></div>}.into_view(cx)
            }}
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=index/>
                <Route path="/:cid" view=post/>
            </Routes>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}