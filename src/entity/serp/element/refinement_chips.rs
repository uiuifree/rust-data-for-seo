use crate::entity::SerpApiElementRefinementChipsElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRefinementChips {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub xpath: Option<String>,
    pub items: Option<Vec<SerpApiElementRefinementChipsElement>>,
}
