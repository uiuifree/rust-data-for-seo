use serde::{Deserialize, Serialize};

/// Freshness status of Google Ads keyword data.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/status/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsStatus {
    /// Whether Google updated keyword data for the previous month.
    pub actual_data: Option<bool>,
    /// Date of the latest Google Ads data update, in "yyyy-mm-dd" format.
    pub date_update: Option<String>,
    /// Latest year for which monthly search volume is available.
    pub last_year_in_monthly_searches: Option<i32>,
    /// Latest month for which monthly search volume is available (1-12).
    pub last_month_in_monthly_searches: Option<i32>,
}
