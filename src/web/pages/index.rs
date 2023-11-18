use leptos::*;
use serde::{Deserialize, Serialize};

use super::{Page, PageContext};

#[derive(Clone, Serialize, Deserialize)]
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
        view! {
            <div>
                <h1>howdy</h1>
                <p>you can call me al, i write sfotware for a living and create some other stuff in my free time.</p>
                <p>right now i enjoy working on decentralized infrastructure, listening to and making music, and trying to make interesting things.</p>
                <p>you can find me most days making filecoin usable with the team at <a class="link" href="https://www.banyan.computer/"> banyan </a> </p>
            </div>
        }.into_view()
    }
}
