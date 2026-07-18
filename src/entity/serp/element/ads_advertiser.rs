use serde::{Deserialize, Serialize};

/// Ads Advertiser SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsAdvertiser {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Title of the result.
    pub title: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// `true` if the advertiser is verified.
    pub verified: Option<bool>,
    /// Approximate number of ads for the advertiser.
    pub approx_ads_count: Option<i32>,
}
