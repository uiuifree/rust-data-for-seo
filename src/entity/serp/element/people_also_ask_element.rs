use crate::entity::SerpApiElementPeopleAlsoAskExpandedElement;
use serde::{Deserialize, Serialize};

/// PeopleAlsoAskElement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Seed question.
    pub seed_question: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Expanded element.
    pub expanded_element: Option<Vec<SerpApiElementPeopleAlsoAskExpandedElement>>,
}
