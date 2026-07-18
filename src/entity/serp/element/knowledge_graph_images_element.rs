use serde::{Deserialize, Serialize};

/// Knowledge Graph Images Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementKnowledgeGraphImagesElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Alternative text of the image.
    pub alt: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
}
