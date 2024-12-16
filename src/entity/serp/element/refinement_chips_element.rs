use crate::entity::SerpApiElementRefinementChipsOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRefinementChipsElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub options: Option<Vec<SerpApiElementRefinementChipsOption>>,
}
