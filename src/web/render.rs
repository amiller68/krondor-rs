use leptos::*;
use leptos_router::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

use crate::types::{Item, Manifest, PostItem, GalleryItem, Cid};
use crate::env::APP_IPFS_GATEWAY;
use crate::ipfs::GatewayClient;

use super::utils::{get_manifest, get_item_text, get_item_url, get_url};

#[component]
pub fn Render() -> impl IntoView {
    let params = use_params_map();
    let item_view = create_resource(
        || (),
        move |_| async move {
            let cid = move || params.with(|params| params.get("cid").cloned().unwrap_or_default());
            let manifest = super::utils::get_manifest().await.expect("manifest");
            let cid_string = cid();
            let cid = Cid::from_str(&cid_string).expect("cid");
            let item = manifest.item_by_cid(&cid).expect("item");
            get_item_view(item).await
        },
    );

    view! { ,
        <div>
            {move || match item_view.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(iv) => iv.into_view()
            }}
        </div>
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemView(String); 

impl IntoView for ItemView {
    fn into_view(self) -> View {
        let ItemView(content) = self; 
        view! {
            <div class="prose max-w-none" inner_html=content></div>
        }.into_view()
    }
}

async fn get_item_view(item: Item) -> ItemView {
    // let collection = item.clone().collection();
    // let name = item.clone().name();
    match item.clone() {
        Item::Post(p) => {
            // let text = GatewayClient(APP_IPFS_GATEWAY.to_string()).cat(&p.cid().to_string()).await.expect("text");
            let text = get_item_text(p.collection(), p.name()).await.expect("text");
            let title = p.title();
            let markdown = markdown_to_html(text);

            // To make images work, we need to replace the src with the url
            // Assumes assets are in a collection
            let url = get_url().expect("url");
            let markdown = markdown.replace("src=\"..", &format!("src=\"{}/collections", url));

            let inner_html = format!(r#"
                <div class="prose max-w-none">
                    <h1>{}</h1>
                    {}
                </div>"#,
                title,
                markdown
            ).to_string();
            ItemView(inner_html)

        }
        Item::Gallery(g) => {
            let url = get_item_url(g.collection(), g.name()).expect("url");
            let inner_html = format!(r#"
                <div class="prose max-w-none">
                    <img src="{}" alt="{}"/>
                    <p>{}</p>
                </div>"#,
                url,
                g.description(),
                g.description()
            ).to_string();
            ItemView(inner_html)
        }
    }
}



// String {
//     let content = match item {
//         Item::Post(post) => {
//             let text = get_item_text(post.collection(), post.name()).await.expect("text");
//             let html = markdown_to_html(text);
            
//             html
//         }
//         _ => String::new(),
//     };
//     content
// }

fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}
