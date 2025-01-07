use crate::entity::SerpApiElementPeopleAlsoAskExpandedElement;
use serde::{Deserialize, Serialize};

/// PeopleAlsoAskElement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAskElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub seed_question: Option<String>,
    pub xpath: Option<String>,
    pub expanded_element: Option<Vec<SerpApiElementPeopleAlsoAskExpandedElement>>,
}
