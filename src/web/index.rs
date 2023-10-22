use crate::types::{Item, SerializedCid};
use async_trait::async_trait;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use super::utils::get_manifest;

#[component]
pub fn Index() -> impl IntoView {
    let items = create_rw_signal(vec![]);
    let item_resource = create_resource(
        // cx,
        || (),
        move |_| async move {
            let manifest = get_manifest().await.expect("manifest");
            items.set(
                manifest
                    .items
                    .iter()
                    .map(|item| {
                        let item_row: ItemRow = item.into();
                        item_row
                    })
                    .collect(),
            );
        },
    );

    view! {
        <div>
            <h2>"This is just a jumble of stuff im hosting here: "</h2>
            {move || match item_resource.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(_) => view! {<ItemRowTable items=items/>}.into_view()
            }}
        </div>
    }
}

impl IntoView for Item {
    fn into_view(self) -> View {
        let name = self.name().to_string();
        let title = self.title().to_string();
        let href = name;
        let html_element = view! {
            <a href=href>
                <h3>{title}</h3>
            </a>
        };
        html_element.into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct ItemRow {
    #[table(key, skip)]
    id: SerializedCid,
    item: Item,
    date: String,
}

impl From<&Item> for ItemRow {
    fn from(item: &Item) -> Self {
        // Create a random id
        let id = item.cid().clone();
        let date = item.date().to_string();
        Self {
            id,
            item: item.clone(),
            date,
        }
    }
}
