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
                <h1>"About Me "</h1>
                <p> I write software and like to write things. </p>
                <p> Try and find me on: </p>
                <ul>
                    <li> <a href="https://github.com/amiller68">Github</a> </li>
                    <li> <a href="https://twitter.com/lord_krondor">Twitter</a> </li>
                </ul>
            </div>
        }
        .into_view()
    }
}
