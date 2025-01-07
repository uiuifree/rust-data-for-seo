use crate::entity::SerpApiElementImagesElement;
use serde::{Deserialize, Serialize};

/// PeopleAlsoAskExtendedElement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskExpandedElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub featured_title: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    pub timestamp: Option<String>,
    pub table: Option<SerpApiElementPeopleAlsoAskExpandedElementTable>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskExpandedElementTable {
    pub table_header: Option<Vec<String>>,
    pub table_content: Option<Vec<Vec<String>>>,
}
