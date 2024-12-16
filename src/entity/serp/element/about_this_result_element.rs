use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAboutThisResultElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub url: Option<String>,
    pub source: Option<String>,
    pub source_info: Option<String>,
    pub source_url: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,
    pub search_terms: Option<Vec<String>>,
    pub related_terms: Option<Vec<String>>,
}
