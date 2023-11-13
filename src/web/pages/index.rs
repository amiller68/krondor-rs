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
            <div class="max-w-4xl mx-auto text-center">
                <h1 class="text-4xl font-bold mb-4">howdy</h1>
                <p class="text-gray-700 text-lg">you can call me al, i write sfotware for a living and create some other stuff in my free time.</p>
                <p class="text-gray-700 text-lg">right now i enjoy working on decentralized infrastructure, listening to and making music, and trying to make interesting things.</p>
                <p class="text-gray-700 text-lg">you can find me most days making filecoin usable with the team at <a href="https://www.banyan.computer/" class="text-teal-500"> banyan </a> </p>
                <p class="text-gray-700 text-lg"></p>
                <p class="text-gray-700 text-lg"></p>
                <p class="text-gray-700 text-lg mb-3">Try and find me on:</p>
                <div class="flex flex-col space-y-2">
                    <a href="https://github.com/amiller68" class="text-teal-500 hover:text-teal-700 hover:italic">Github</a>
                    <a href="https://twitter.com/lord_krondor" class="text-teal-500 hover:text-teal-700 hover:italic">Twitter</a>
                    <a href="mailto:al@krondor.org" class="text-teal-500 hover:text-teal-700 hover:italic">Personal Email: al@krondor.org</a>
                    <a href="mailto:alex@banyan.computer" class="text-teal-500 hover:text-teal-700 hover:italic">Work Email: alex@banyan.computer</a>
                    <a href="tg://resolve?domain=lord_krondor" class="text-teal-500 hover:text-teal-700 hover:italic">Telegram: @lord_krondor</a>
                </div>
            </div>
        }.into_view()
    }
}
