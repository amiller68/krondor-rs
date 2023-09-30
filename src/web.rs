use async_trait::async_trait;
use leptos::*;
use leptos_router::*;
use leptos_struct_table::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        leptos::mount_to_body(|cx| leptos::view! { cx, <WebApp/> })
    }
}

#[component]
fn WebApp(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=Index/>
                <Route path="/:cid" view=Post/>
            </Routes>
        </Router>
    }
}

#[component]
fn Index(cx: Scope) -> impl IntoView {
    let items = create_rw_signal(cx, vec![]);

    let posts = create_resource(
        cx,
        || (),
        move |_| async move {
            let posts = get_post_rows().await.unwrap();
            items.set(posts);
        },
    );
    view! { cx,
        <div>
            <p>"Welcome to Krondor-Rs!"</p>
            <p>"You're currently using a static wasm-app hosted on IPFS."</p>
            <p>"Look at some of the stuff I've written:"</p>
            {move || match posts.read(cx) {
                None => view! {cx,  <p>"Loading..."</p> }.into_view(cx),
                Some(_) => view! {cx, <PostRowTable items=items/>}.into_view(cx)
            }}
        </div>
    }
}

#[component]
fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let post = create_resource(
        cx,
        || (),
        move |_| async move {
            let cid = move || params.with(|params| params.get("cid").cloned().unwrap_or_default());
            get_post_content(cid()).await.unwrap()
        },
    );
    view! { cx,
        <div>
            {move || match post.read(cx) {
                None => view! {cx,  <p>"Loading..."</p> }.into_view(cx),
                Some(data) => view! {cx, <div class="prose max-w-none" inner_html=markdown_to_html(data)></div>}.into_view(cx)
            }}
        </div>
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PostLink((String, Cid));

impl From<Post> for PostLink {
    fn from(post: Post) -> Self {
        let cid = post.cid();
        let title = post.title();
        PostLink((title.to_string(), cid))
    }
}

impl IntoView for PostLink {
    fn into_view(self, cx: leptos::Scope) -> View {
        let (title, cid) = self.0;
        let cid = cid.to_string();
        let href = format!("/{}", cid);
        let html_element = view! { cx,
            <a href=href>
                {title}
            </a>
        };
        html_element.into_view(cx)
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct PostRow {
    #[table(key, skip)]
    id: Cid,
    link: PostLink,
    date: String,
}

impl From<Post> for PostRow {
    fn from(post: Post) -> Self {
        let cid = post.cid();
        let link = PostLink::from(post.clone());
        let date = post.date().to_string();
        PostRow {
            id: cid,
            link,
            date,
        }
    }
}

async fn get_post_rows() -> KrondorResult<Vec<PostRow>> {
    let config = KrondorConfig::new()?;
    let root_cid = config.root_cid;
    let gateway = config.gateway;
    let root_cid = root_cid
        .get()
        .await
        .map_err(|_| KrondorError::msg("get_post_rows(): couldn't get cid"))?;

    let manifest_cid = format!("{}/manifest.json", root_cid);
    let manifest = gateway.get(&manifest_cid).await.unwrap();
    let manifest = serde_json::from_str::<serde_json::Value>(&manifest).unwrap();
    let posts = manifest["posts"].as_array().unwrap();
    let posts = posts
        .iter()
        .map(|post| {
            serde_json::from_value::<Post>(post.clone())
                .map_err(|_| KrondorError::msg("get_post_rows(): couldn't parse manifest"))
                .unwrap()
        })
        .collect::<Vec<Post>>();
    let post_rows = posts
        .iter()
        .map(|post| PostRow::from(post.clone()))
        .collect::<Vec<PostRow>>();
    Ok(post_rows)
}

async fn get_post_content(cid: String) -> KrondorResult<String> {
    let config = KrondorConfig::new()?;
    let gateway = config.gateway;
    let post = gateway.get(&cid).await.map_err(|_| KrondorError::msg("get_post_content(): couldn't get post"))?;
    Ok(post)
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
