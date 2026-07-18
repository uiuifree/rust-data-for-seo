use crate::entity::SerpApiElementImagesElement;
use serde::{Deserialize, Serialize};

/// PeopleAlsoAskExtendedElement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskExpandedElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Featured title.
    pub featured_title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Images attached to the result.
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Table.
    pub table: Option<SerpApiElementPeopleAlsoAskExpandedElementTable>,
}
/// People Also Ask Expanded Element Table SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskExpandedElementTable {
    /// Table header.
    pub table_header: Option<Vec<String>>,
    /// Table content.
    pub table_content: Option<Vec<Vec<String>>>,
}
