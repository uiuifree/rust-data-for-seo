use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsDomain {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub domain: Option<String>,
}
