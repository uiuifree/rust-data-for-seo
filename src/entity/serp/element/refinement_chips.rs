use crate::entity::SerpApiElementRefinementChipsElement;
use serde::{Deserialize, Serialize};

/// Refinement Chips SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRefinementChips {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementRefinementChipsElement>>,
}
