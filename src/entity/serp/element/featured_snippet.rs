use serde::{Deserialize, Serialize};

/// Featured Snippet SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFeaturedSnippet {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Breadcrumb shown under the result URL.
    pub breadcrumb: Option<String>,
}
