use leptos::*;
use serde::{Deserialize, Serialize};

use super::{InternalLink, Page, PageContext};
use crate::web::components::item_table::ItemRow;
use crate::web::components::item_table::ItemRowTable;
use crate::types::Item;

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexPage(PageContext);

impl Page for IndexPage {
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

impl IntoView for IndexPage {
    fn into_view(self) -> View {
        let manifest = create_rw_signal(self.0.manifest().clone());
        let items: leptos::RwSignal<Vec<ItemRow>> = create_rw_signal(self.0.manifest()
            .items
            .iter()
            .map(|item| {
                let item_row: ItemRow = item.into();
                item_row
            })
            .collect());

        view! {
            <div>
                <p><strong>"Hi there, this is just some stuff i like to work on: "</strong></p>
                <p><strong>"- Al"</strong></p>
                <ItemRowTable items=items/>
            </div>
        }
        .into_view()
    }
}