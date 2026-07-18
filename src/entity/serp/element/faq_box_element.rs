use crate::entity::SerpApiElementLinkElement;
use serde::{Deserialize, Serialize};

/// Faq Box Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFaqBoxElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Links associated with the result.
    pub links: Option<Vec<SerpApiElementLinkElement>>,
}
