use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementImagesElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub alt: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}
