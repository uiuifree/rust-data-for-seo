use serde::{Deserialize, Serialize};

/// Advertiser SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdvertiser {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Google Ads advertiser identifier.
    pub advertiser_id: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// `true` if the advertiser is verified.
    pub verified: Option<bool>,
    /// Approximate number of ads for the advertiser.
    pub approx_ads_count: Option<i64>,
}
