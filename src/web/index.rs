use async_trait::async_trait;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use super::InternalRoute;

use crate::types::{Item, Manifest, SerializedCid};

#[derive(Serialize, Deserialize, Clone)]
pub struct Index(pub Manifest);

impl IntoView for Index {
    fn into_view(self) -> View {
        let manifest = create_rw_signal(self.0);
        let items = create_rw_signal(vec![]);
        // High level fetch resource
        let resource = create_resource(
            || (),
            move |_| async move {
                let manifest = manifest.get();
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
                <p><strong>"Hi there, this is just some stuff i like to work on: "</strong></p>
                <p><strong>"- Al"</strong></p>
                {move || match resource.get() {
                    // TODO: Loading animation
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(_) => view! {<ItemRowTable items=items/>}.into_view()
                }}
            </div>
        }
        .into_view()
    }
}

impl IntoView for Item {
    fn into_view(self) -> View {
        let name = self.name().to_string();
        let title = self.title().to_string();
        let href = format!("?route=posts&name={}", name);
        let html_element = view! {
            <InternalRoute query=href msg=title/>
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
