use serde::{Deserialize, Serialize};

pub const DEFAULT_ID: &str = "-1";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownId {
    pub id: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownUpdated {
    pub id: String,
    pub version: i64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Markdown {
    pub id: String,
    pub title: String,
    pub context: String,
    pub version: i64,
}
impl Markdown {
    pub fn default() -> Self {
        Self {
            id: DEFAULT_ID.to_string(),
            title: "".to_string(),
            context: "".to_string(),
            version: 1,
        }
    }

    pub fn update_context(mut self, context: String) -> Self {
        self.context = context;
        return self;
    }

    pub fn update_version(mut self, version: i64) -> Self {
        self.version = version;
        return self;
    }
}
