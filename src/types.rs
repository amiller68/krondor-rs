
use std::convert::{From, TryFrom};

use cid::Cid as BaseCid;
use leptos::{IntoView, View};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cid(BaseCid);

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Post {
    cid: Cid,
    name: String,
    title: String,
    date: String,
}

impl Post {
    pub fn cid(&self) -> Cid {
        self.cid
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn date(&self) -> &str {
        self.date.as_str()
    }
}

impl Cid {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Cid> for BaseCid {
    fn from(cid: Cid) -> Self {
        cid.0
    }
}

impl IntoView for Cid {
    fn into_view(self, cx: leptos::Scope) -> View {
        use leptos::leptos_dom::Text;
        let text = self.to_string();
        let text = Text::new(text.into());
        text.into_view(cx)
    }
}

impl Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Cid(BaseCid::try_from(s).unwrap()))
    }
}