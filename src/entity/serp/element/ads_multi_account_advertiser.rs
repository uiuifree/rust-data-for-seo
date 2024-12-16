use crate::entity::SerpApiElementAdvertiser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsMultiAccountAdvertiser {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub title: Option<String>,
    pub location: Option<String>,
    pub approx_ads_count: Option<i32>,
    pub advertisers: Option<Vec<SerpApiElementAdvertiser>>,
}
