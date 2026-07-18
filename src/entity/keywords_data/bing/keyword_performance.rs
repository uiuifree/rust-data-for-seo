use serde::{Deserialize, Serialize};

/// A keyword performance result item from the Bing `keyword_performance` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_performance/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordPerformance {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<i32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Calendar year the metrics cover.
    pub year: Option<i32>,
    /// Calendar month the metrics cover (1-12).
    pub month: Option<i32>,
    /// Performance metrics broken down by device.
    pub keyword_kpi: Option<KeywordsDataApiBingKeywordKpi>,
}

/// Per-device performance KPI breakdown.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordKpi {
    /// KPI estimates for desktop devices.
    pub desktop: Option<Vec<KeywordsDataApiBingKpiItem>>,
    /// KPI estimates for mobile devices.
    pub mobile: Option<Vec<KeywordsDataApiBingKpiItem>>,
    /// KPI estimates for tablet devices.
    pub tablet: Option<Vec<KeywordsDataApiBingKpiItem>>,
}

/// A KPI estimate for a given ad position.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKpiItem {
    /// Ad position the estimate applies to.
    pub ad_position: Option<String>,
    /// Estimated number of clicks.
    pub clicks: Option<f64>,
    /// Estimated number of impressions.
    pub impressions: Option<f64>,
    /// Estimated average cost per click, in USD.
    pub average_cpc: Option<f64>,
    /// Estimated click-through rate.
    pub ctr: Option<f64>,
    /// Estimated total cost, in USD.
    pub total_cost: Option<f64>,
    /// Estimated average bid, in USD.
    pub average_bid: Option<f64>,
}
