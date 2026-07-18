use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use serde::{Deserialize, Serialize};

/// Ai Overview Reference SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverviewReference {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    /// Source / publisher of the result.
    pub source: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Text content.
    pub text: Option<String>,
    /// Images attached to the result.
    pub images: Option<Vec<SerpApiElementImagesElement>>,
}
