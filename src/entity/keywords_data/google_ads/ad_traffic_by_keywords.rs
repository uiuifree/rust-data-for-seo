use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask {
    pub keyword: Option<String>,
    pub location_code: Option<u32>,
    pub language_code: Option<String>,
    pub date_interval: Option<String>,
    pub search_partners: Option<bool>,
    pub bid: Option<f32>,
    #[serde(rename = "match")]
    pub match_type: Option<String>,
    pub impressions: Option<f32>,
    pub ctr: Option<f32>,
    pub average_cpc: Option<f32>,
    pub cost: Option<f32>,
    pub clicks: Option<f32>,
}
