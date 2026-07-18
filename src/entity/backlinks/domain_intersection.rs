use crate::entity::BacklinksApiElementBacklinksDomainIntersection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Result of the Domain Intersection endpoint: domains linking to several targets at once.
/// See <https://docs.dataforseo.com/v3/backlinks/domain_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainIntersection {
    /// Targets to analyze (domains, subdomains, or webpages).
    pub targets: Option<Value>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiDomainIntersectionItem>>,
}

/// A single domain intersection item: per-target metrics plus an intersection summary.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainIntersectionItem {
    /// Per-target metrics keyed by target id.
    pub domain_intersection:
        Option<HashMap<String, BacklinksApiElementBacklinksDomainIntersection>>,
    /// Intersection summary for this item.
    pub summary: Option<BacklinksApiIntersectionSummary>,
}

/// Intersection summary shared by Domain Intersection and Page Intersection results.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIntersectionSummary {
    /// Number of targets that intersect for this item.
    pub intersections_count: Option<i64>,
}
