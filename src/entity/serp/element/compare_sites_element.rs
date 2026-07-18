use serde::{Deserialize, Serialize};

/// Compare Sites Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementCompareSitesElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// Source / publisher of the result.
    pub source: Option<String>,
}
