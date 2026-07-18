use serde::{Deserialize, Serialize};

/// A result item from the Bing `audience_estimation` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingAudienceEstimation {
    /// Estimated monthly impressions range.
    pub est_impressions: Option<KeywordsDataApiBingAudienceRange>,
    /// Estimated monthly reach (unique users) range.
    pub est_audience_size: Option<KeywordsDataApiBingAudienceRange>,
    /// Estimated monthly clicks range.
    pub est_clicks: Option<KeywordsDataApiBingAudienceRange>,
    /// Estimated monthly spend range, in USD.
    pub est_spend: Option<KeywordsDataApiBingAudienceRange>,
    /// Estimated cost-per-event range.
    pub est_cost_per_event: Option<KeywordsDataApiBingAudienceRange>,
    /// Estimated click-through rate range.
    pub est_ctr: Option<KeywordsDataApiBingAudienceRange>,
    /// Recommended bid, in USD.
    pub suggested_bid: Option<f64>,
    /// Recommended daily budget, in USD.
    pub suggested_budget: Option<f64>,
    /// Events lost due to an insufficient bid.
    pub events_lost_to_bid: Option<i64>,
    /// Events lost due to an insufficient budget.
    pub events_lost_to_budget: Option<i64>,
    /// Estimated monthly reach (unique users).
    pub est_reach_audience_size: Option<i64>,
    /// Estimated monthly impressions.
    pub est_reach_impressions: Option<i64>,
    /// Currency of the monetary estimates, e.g. "USDollar".
    pub currency: Option<String>,
}

/// A `high`/`low` estimate range used by the audience estimation result.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingAudienceRange {
    /// Upper bound of the estimate.
    pub high: Option<f64>,
    /// Lower bound of the estimate.
    pub low: Option<f64>,
}

/// A job function usable for Bing audience targeting.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/job_functions/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingJobFunction {
    /// Unique identifier of the job function.
    pub job_function_id: Option<i64>,
    /// Name of the job function.
    pub job_function_name: Option<String>,
}

/// An industry usable for Bing audience targeting.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/industries/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingIndustry {
    /// Unique identifier of the industry.
    pub industry_id: Option<i64>,
    /// Name of the industry.
    pub industry_name: Option<String>,
}
