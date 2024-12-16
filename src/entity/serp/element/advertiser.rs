use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdvertiser {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub advertiser_id: Option<String>,
    pub location: Option<String>,
    pub verified: Option<bool>,
    pub approx_ads_count: Option<i32>,
}
