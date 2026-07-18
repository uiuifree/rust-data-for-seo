use crate::entity::SerpApiElementAiOverviewElement;
use serde::{Deserialize, Serialize};

/// Ai Overview SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverview {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// `true` if the AI overview is loaded asynchronously.
    pub asynchronous_ai_overview: Option<bool>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementAiOverviewElement>>,
}
