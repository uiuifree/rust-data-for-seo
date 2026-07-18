use serde::{Deserialize, Serialize};

/// Carousel Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementCarouselElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Subtitle text of the result.
    pub subtitle: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
}
