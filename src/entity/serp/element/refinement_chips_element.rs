use crate::entity::SerpApiElementRefinementChipsOption;
use serde::{Deserialize, Serialize};

/// Refinement Chips Element SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRefinementChipsElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Options.
    pub options: Option<Vec<SerpApiElementRefinementChipsOption>>,
}
