use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDiscussionsAndForumsElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    pub title: Option<String>,
    pub url: Option<String>,
    pub source: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<String>,
    pub posts_count: Option<i32>,
}
