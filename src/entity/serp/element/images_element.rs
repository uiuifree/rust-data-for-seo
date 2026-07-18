use serde::{Deserialize, Serialize};

/// Images Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementImagesElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Alternative text of the image.
    pub alt: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
}
