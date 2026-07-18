use crate::entity::BacklinksApiElementBacklinksPageSummary;
use serde::{Deserialize, Serialize};

/// Result of the Domain Pages Summary endpoint: backlink summary per page of a domain.
/// See <https://docs.dataforseo.com/v3/backlinks/domain_pages_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainPagesSummary {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i32>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i32>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksPageSummary>>,
}
