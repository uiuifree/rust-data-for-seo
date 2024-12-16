use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementLinkElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}
