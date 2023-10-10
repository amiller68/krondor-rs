use async_trait::async_trait;
use leptos::*;
use leptos_router::*;
use leptos_struct_table::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use crate::types::{Cid, History, Item, Manifest};
use crate::env::APP_NAME;

use super::utils::get_manifest;

#[component]
pub fn Index() -> impl IntoView {
    let items = create_rw_signal(vec![]);
    let item_resource = create_resource(
        // cx,
        || (),
        move |_| async move {
            let manifest = get_manifest().await.expect("manifest");
            items.set(manifest.items.iter().map(|item| {
                let item_row: ItemRow = item.into();
                item_row
            }).collect());
        },
    );

    view! {
        <div>
            <p>Welcome to {APP_NAME}</p>
            <p>"You're currently using a static wasm-app hosted on IPFS."</p>
            <p>"This is just a jumble of stuff im hosting here: "</p>
            {move || match item_resource.read() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(_) => view! {<ItemRowTable items=items/>}.into_view()
            }}
        </div>
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemLink((String, String));

impl From<Item> for ItemLink {
    fn from(item: Item) -> Self {
        ItemLink((item.name().to_string(), item.cid().to_string()))
    }
}

impl IntoView for ItemLink {
    fn into_view(self) -> View {
        let (name, cid) = self.0;
        let href = format!("/{}", cid);
        let html_element = view! {
            <a href=href>
                {name}
            </a>
        };
        html_element.into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct ItemRow {
    #[table(key, skip)]
    id: Cid,
    link: ItemLink,
    collection: String,
    date: String,
}

impl From<&Item> for ItemRow {
    fn from(item: &Item) -> Self {
        let id = item.cid().clone();
        let link = item.clone().into();
        let collection = item.collection().to_string();
        let date = item.date().to_string();
        Self { id, link, collection, date }
    }
}
