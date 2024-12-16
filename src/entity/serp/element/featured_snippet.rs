use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFeaturedSnippet {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub breadcrumb: Option<String>,
}
