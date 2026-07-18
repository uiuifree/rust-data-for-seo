use serde::{Deserialize, Serialize};

/// Result object for the `aggregation_technologies` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/aggregation_technologies/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiAggregationTechnologies {
    /// Total number of aggregation buckets matching the request.
    pub total_count: Option<i64>,
    /// Number of aggregation buckets returned in `items`.
    pub items_count: Option<i64>,
    /// The aggregated technology usage buckets.
    pub items: Option<Vec<DomainAnalyticsApiAggregationTechnologiesItem>>,
}

/// A single aggregated technology usage bucket.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiAggregationTechnologiesItem {
    /// Result element type, e.g. `aggregation_technologies_item`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Technology group ID of this bucket.
    pub group: Option<String>,
    /// Technology category ID of this bucket.
    pub category: Option<String>,
    /// Technology name of this bucket.
    pub technology: Option<String>,
    /// Number of domains using any technology from this group.
    pub groups_count: Option<i64>,
    /// Number of domains using any technology from this category.
    pub categories_count: Option<i64>,
    /// Number of domains using this specific technology.
    pub technologies_count: Option<i64>,
}
