use crate::entity::SerpApiElementAdvertiser;
use serde::{Deserialize, Serialize};

/// Ads Multi Account Advertiser SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsMultiAccountAdvertiser {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Title of the result.
    pub title: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Approximate number of ads for the advertiser.
    pub approx_ads_count: Option<i64>,
    /// Advertiser accounts grouped under this entry.
    pub advertisers: Option<Vec<SerpApiElementAdvertiser>>,
}
