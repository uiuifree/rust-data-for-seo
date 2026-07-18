use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result object for the `technology_stats` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technology_stats/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyStats {
    /// Technology name the stats describe.
    pub technology: Option<String>,
    /// Start of the requested date range (`yyyy-mm-dd`).
    pub date_from: Option<String>,
    /// End of the requested date range (`yyyy-mm-dd`).
    pub date_to: Option<String>,
    /// Number of stat snapshots returned in `items`.
    pub items_count: Option<i64>,
    /// Adoption snapshots, one per date in the range.
    pub items: Option<Vec<DomainAnalyticsApiTechnologyStatsItem>>,
}

/// Technology adoption stats for a single point in time.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyStatsItem {
    /// Result element type, e.g. `technology_stats_item`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Date of this snapshot (UTC).
    pub date: Option<String>,
    /// Total number of domains using the technology on this date.
    pub domains_count: Option<i64>,
    /// Domain count per ISO country code.
    pub countries: Option<HashMap<String, i64>>,
    /// Domain count per language code.
    pub languages: Option<HashMap<String, i64>>,
    /// Domain count per domain-rank range.
    pub domains_rank: Option<HashMap<String, i64>>,
}
