use async_trait::async_trait;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use super::{InternalLink, Page, PageContext};

use crate::types::Item;

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexPage(PageContext);

impl Page for IndexPage {
    // fn name() -> &'static str {
    //     "index"
    // }
    fn ctx(&self) -> &PageContext {
        &self.0
    }
    fn from_ctx(ctx: PageContext) -> Box<dyn Page> {
        Box::new(Self(ctx))
    }
    // fn clone_box(&self) -> Box<dyn Page> {
    //     Box::new(self.clone())
    // }
    fn into_view_ref(&self) -> View {
        self.clone().into_view()
    }
}

impl IntoView for IndexPage {
    fn into_view(self) -> View {
        let manifest = create_rw_signal(self.0.manifest().clone());
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
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(_) => view! {<ItemRowTable items=items/>}.into_view()
                }}
            </div>
        }
        .into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct ItemRow {
    #[table(key, skip)]
    id: Cid,
    item: ItemLink,
    date: String,
}

impl From<&Item> for ItemRow {
    fn from(item: &Item) -> Self {
        // Create a random id
        let id = item.cid().clone();
        let date = item.date().to_string();
        Self {
            id,
            item: ItemLink(item.clone()),
            date,
        }
    }
}

impl IntoView for ItemLink {
    fn into_view(self) -> View {
        let name = self.0.name().to_string();
        let title = self.0.title().to_string();
        let href = format!("?route=items&query={}", name);
        let html_element = view! {
            <InternalLink query=href msg=title/>
        };
        html_element.into_view()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ItemLink(Item);
