use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFaqBox {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub items: Option<String>,
}
