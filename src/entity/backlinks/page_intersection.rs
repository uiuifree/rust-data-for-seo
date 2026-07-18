use crate::entity::{
    BacklinksApiElementBacklinksPageIntersection, BacklinksApiIntersectionSummary,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Result of the Page Intersection endpoint: pages linking to several targets at once.
/// See <https://docs.dataforseo.com/v3/backlinks/page_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiPageIntersection {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Option<Value>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiPageIntersectionItem>>,
}

/// A single page intersection item: per-target backlink records plus an intersection summary.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiPageIntersectionItem {
    /// Backlink records per target id that link to all analyzed pages.
    pub page_intersection:
        Option<HashMap<String, Vec<BacklinksApiElementBacklinksPageIntersection>>>,
    /// Intersection summary for this item.
    pub summary: Option<BacklinksApiIntersectionSummary>,
}
