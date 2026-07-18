use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{SerpApiElementAiOverviewReference, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// Ai Overview Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverviewElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    /// Title of the result.
    pub title: Option<String>,
    /// Text content.
    pub text: Option<String>,
    /// Images attached to the result.
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    /// Source references cited in the AI overview.
    pub references: Option<Vec<SerpApiElementAiOverviewReference>>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
