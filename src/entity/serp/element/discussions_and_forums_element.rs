use serde::{Deserialize, Serialize};

/// Discussions And Forums Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDiscussionsAndForumsElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Source / publisher of the result.
    pub source: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Number of posts.
    pub posts_count: Option<i32>,
}
