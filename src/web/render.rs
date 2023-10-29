use leptos::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

use crate::types::Item;

use super::utils::{get_item_text, get_url};

#[derive(Serialize, Deserialize, Clone)]
pub struct RenderItem(pub Option<Item>);

impl IntoView for RenderItem {
    fn into_view(self) -> View {
        let item = create_rw_signal(self.0.expect("item"));
        let item_view = create_resource(
            || (),
            move |_| async move { get_item_view(item.get()).await },
        );

        view! {
            <div>
                {move || match item_view.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(iv) => iv.into_view()
                }}
            </div>
        }
        .into_view()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemView(String);

impl IntoView for ItemView {
    fn into_view(self) -> View {
        let ItemView(content) = self;
        view! {
            <div class="prose max-w-none" inner_html=content></div>
        }
        .into_view()
    }
}

async fn get_item_view(item: Item) -> ItemView {
    // let collection = item.clone().collection();
    // let name = item.clone().name();
    match item.render() {
        "markdown" => {
            let text = get_item_text(item.name()).await.expect("text");
            let title = item.title();
            let date = item.date();
            let description = item.description();
            let markdown = markdown_to_html(text);

            let url = get_url().expect("url");
            let markdown = markdown.replace("src=\"./", &format!("src=\"{}/posts/", url));

            let inner_html = format!(
                r#"<div class="prose max-w-none">
                    <h1>{title}</h1>
                    <p><em>{description}</em></p>
                    <p><strong>{date}</strong></p>
                    {markdown}
                </div>"#
            )
            .to_string();
            ItemView(inner_html)
        }
        _ => {
            // Redirect to /posts/{name}
            let url = get_url().expect("url");
            let url = format!("{}/posts/{}", url, item.name());
            web_sys::window()
                .expect("window")
                .location()
                .set_href(&url)
                .expect("set href");
            ItemView("".to_string())
        }
    }
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
