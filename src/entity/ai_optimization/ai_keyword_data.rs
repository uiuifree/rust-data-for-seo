//! Models for the AI Keyword Data sub-API.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/ai_keyword_data/overview/>.

use crate::entity::AiOptimizationMonthlySearch;
use serde::{Deserialize, Serialize};

/// Result of the `keywords_search_volume/live` endpoint.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/ai_keyword_data/keywords_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationKeywordDataSearchVolume {
    /// Location code the volumes were computed for.
    pub location_code: Option<i32>,
    /// Language code the volumes were computed for.
    pub language_code: Option<String>,
    /// Number of items in [`items`](Self::items).
    pub items_count: Option<i32>,
    /// Per-keyword AI search-volume data points.
    pub items: Option<Vec<AiOptimizationKeywordDataItem>>,
}

/// Per-keyword AI search-volume data point.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationKeywordDataItem {
    /// The keyword the metrics refer to.
    pub keyword: Option<String>,
    /// Current AI search volume rate for the keyword.
    pub ai_search_volume: Option<i64>,
    /// Monthly AI search-volume history for the keyword.
    pub ai_monthly_searches: Option<Vec<AiOptimizationMonthlySearch>>,
}
