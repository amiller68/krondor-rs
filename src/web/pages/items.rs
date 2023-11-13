use leptos::*;
use leptos_use::use_event_listener;

use serde::{Deserialize, Serialize};

use crate::types::{Item, Render};
use crate::utils::web::{fix_src, get_item_text, get_item_url, markdown_to_html};
use crate::web::components::{ItemRow, ItemRowTable};

use super::{Page, PageContext};

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemsPage(PageContext);

impl Page for ItemsPage {
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

impl IntoView for ItemsPage {
    fn into_view(self) -> View {
        let items: leptos::RwSignal<Vec<ItemRow>> = create_rw_signal(
            self.0
                .manifest()
                .items
                .iter()
                .map(|item| {
                    let item_row: ItemRow = item.into();
                    item_row
                })
                .collect(),
        );
        let item = create_rw_signal(
            self.0
                .manifest()
                .item(&self.0.query().clone().unwrap_or_default()),
        );
        let page_view = create_rw_signal(None);
        let _render_view = create_resource(
            || (),
            move |_| async move {
                match item.get() {
                    None => {
                        gloo::console::log!("No item found");
                        page_view.set(Some(view! {
                            <div class="text-center top-0 left-0">
                                <p><strong>"this is just a jumble of stuff i got. i should really organize it but for now ur gonna have to explore ;)"</strong></p>
                                <ItemRowTable items=items/>
                            </div>
                        }.into_view()));
                    }
                    Some(item) => {
                        let iv = render_item_view(&item).await;
                        page_view.set(Some(iv));
                    }
                }
            },
        );

        view! {
            <div>
                {move || match page_view.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(pv) => pv.into_view()
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
                    <div class="flex justify-center">
                        <button
                            class="text-center bg-blue-500 hover:bg-yellow-500 text-white font-bold py-2 px-4 rounded"
                            node_ref=button_ref
                        ></button>
                    </div>
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
