use leptos::*;
use leptos_use::use_event_listener;

use serde::{Deserialize, Serialize};

use crate::types::{Item, Render};
use crate::utils::web::{fix_src, get_item_text, get_item_url, markdown_to_html};

use super::{Page, PageContext};

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemPage(PageContext);

impl Page for ItemPage {
    fn ctx(&self) -> &PageContext {
        &self.0
    }
    fn from_ctx(ctx: PageContext) -> Box<dyn Page> {
        Box::new(Self(ctx))
    }
    fn into_view_ref(&self) -> View {
        self.clone().into_view()
    }
}

impl IntoView for ItemPage {
    fn into_view(self) -> View {
        let item = create_rw_signal(
            self.0
                .manifest()
                .item(&self.0.query().clone().expect("query")),
        );
        let item_view = create_rw_signal(None);
        let _render_view = create_resource(
            || (),
            move |_| async move {
                let item = item.get().expect("item");
                let iv = render_item_view(&item).await;
                item_view.set(Some(iv));
            },
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

async fn render_item_view(item: &Item) -> View {
    let content = match item.render() {
        Render::Markdown => {
            let text = get_item_text(item.name()).await.expect("text");
            let markdown = markdown_to_html(text);
            let markdown = fix_src(markdown);
            view! { <div class="prose max-w-none" inner_html=markdown/> }
        }
        Render::Html => {
            let url = get_item_url(item.name()).expect("url");
            view! { <div> <iframe src={url} width="800" height="800"></iframe> </div> }
        }
        Render::Jpg => {
            let url = get_item_url(item.name()).expect("url");
            let html = format!(r#"<img src="{url}"/>"#, url = url);
            view! { <div class="prose max-w-none"> {html} </div> }
        }
        Render::Mp3 => {
            let url = get_item_url(item.name()).expect("url");
            let audio_ref: NodeRef<html::Audio> = create_node_ref::<html::Audio>();
            let button_ref: NodeRef<html::Button> = create_node_ref::<html::Button>();

            audio_ref.on_load(move |_| {
                let audio = audio_ref.get().expect("audio");
                button_ref.on_load(move |_| {
                    let button = button_ref.get().expect("button");
                    button.set_inner_text("Play Audio");
                    let _ = use_event_listener(button_ref, leptos::ev::click, move |_| {
                        if audio.paused() {
                            let _ = audio.play().expect("play");
                            button.set_inner_text("Pause Audio");
                        } else {
                            audio.pause().expect("pause");
                            button.set_inner_text("Play Audio");
                        }
                    });
                });
            });

            view! {
                <div class="prose max-w-none"> 
                    <audio 
                        node_ref=audio_ref
                        src={url}
                    />
                    <button 
                        node_ref=button_ref
                    ></button>
                </div>
            }
        }
        Render::Blank => {
            view! { <div class="prose max-w-none"> </div> }
        }
    };
    view! {
        <div>
            <h1>{item.title().to_string()}</h1>
            <p><em>{item.description().to_string()}</em></p>
            <p><strong>{item.date().to_string()}</strong></p>
            {content}
        </div>
    }
    .into_view()
}

// impl IntoView for Item {
//     fn into_view() -> View {

//         Self(content.into())
//     }
// }

// struct SerializableIntoView(Box<dyn IntoView>);

// struct SerializableRenderableItem(Box<dyn RenderableItem>);

// impl Clone for SerializableRenderableItem {
//     fn clone(&self) -> Self {
//         SerializablePage(self.0.clone_box())
//     }
// }

// impl IntoView for SerializableRenderableItem {
//     fn into_view(self) -> View {
//         // Use a reference to call `into_view_ref`
//         self.0.as_ref().into_view_ref()
//     }
// }

// impl Serialize for SerializablePage {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // Extract the content from the trait object
//         let ctx = self.0.ctx();
//         // Serialize the context
//         let mut state = serializer.serialize_struct("PageContext", 3)?;
//         state.serialize_field("manifest", ctx.manifest())?;
//         state.serialize_field("route", ctx.route())?;
//         state.serialize_field("name", ctx.name())?;
//         state.end()
//     }
// }

// impl<'de> Deserialize<'de> for SerializablePage {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // Deserialize the context
//         let ctx = PageContext::deserialize(deserializer)?;
//         // Create a new page from the context
//         let page = IndexPage::from_ctx(ctx);
//         Ok(SerializablePage(page))
//     }
// }
