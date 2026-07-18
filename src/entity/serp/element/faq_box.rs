use serde::{Deserialize, Serialize};

/// Faq Box SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFaqBox {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<String>,
}
