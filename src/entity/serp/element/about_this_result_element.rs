use serde::{Deserialize, Serialize};

/// About This Result Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAboutThisResultElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Source / publisher of the result.
    pub source: Option<String>,
    /// Source info.
    pub source_info: Option<String>,
    /// URL of the source that published the result.
    pub source_url: Option<String>,
    /// Language code of the track.
    pub language: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Search terms.
    pub search_terms: Option<Vec<String>>,
    /// Related terms.
    pub related_terms: Option<Vec<String>>,
}
