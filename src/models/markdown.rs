use serde::{Deserialize, Serialize};

pub const DEFAULT_ID: &str = "-1";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownId {
    pub id: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Markdown {
    pub id: String,
    pub title: String,
    pub context: String,
}
impl Markdown {
    pub fn default() -> Self {
        Self {
            id: DEFAULT_ID.to_string(),
            title: "".to_string(),
            context: "".to_string(),
        }
    }

    pub fn update_context(mut self, context: String) -> Self {
        self.context = context;
        return self;
    }
}
