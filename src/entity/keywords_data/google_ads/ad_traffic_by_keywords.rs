use serde::{Deserialize, Serialize};

/// A keyword result item from the Google Ads Ad Traffic By Keywords endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask {
    /// The keyword this estimate refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<u32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Forecast interval, e.g. "next_week", "next_month" or "next_quarter".
    pub date_interval: Option<String>,
    /// Whether search/partner network data was included.
    pub search_partners: Option<bool>,
    /// Maximum custom bid used for the estimate, in USD.
    pub bid: Option<f32>,
    /// Match type: "exact", "broad" or "phrase".
    #[serde(rename = "match")]
    pub match_type: Option<String>,
    /// Estimated ad impressions.
    pub impressions: Option<f32>,
    /// Estimated click-through rate.
    pub ctr: Option<f32>,
    /// Estimated average cost per click, in USD.
    pub average_cpc: Option<f32>,
    /// Estimated total ad spend, in USD.
    pub cost: Option<f32>,
    /// Estimated number of ad clicks.
    pub clicks: Option<f32>,
}
