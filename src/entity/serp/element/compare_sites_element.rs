use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementCompareSitesElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub image_url: Option<String>,
    pub source: Option<String>,
}
