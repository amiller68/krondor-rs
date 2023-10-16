use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Cid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Item {
    cid: Cid,
    name: String,
    date: DateTime<Utc>, 
    title: String,
    description: String,
    // TODO: this should be a trait or something
    render: String,
}

impl Item {
    pub fn new(
        cid: Cid,
        name: &str,
        date: DateTime<Utc>,
        title: &str,
        description: &str,
        render: &str,
    ) -> Self {
        Self {
            cid,
            name: name.to_string(),
            date,
            title: title.to_string(),
            description: description.to_string(),
            render: render.to_string(),
        }
    }

    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn render(&self) -> &str {
        &self.render
    }
}