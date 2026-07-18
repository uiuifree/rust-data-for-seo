use crate::entity::BacklinksApiElementBacklinksDomainPage;
use serde::{Deserialize, Serialize};

/// Result of the Domain Pages endpoint: pages of a domain that have backlinks.
/// See <https://docs.dataforseo.com/v3/backlinks/domain_pages/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiDomainPages {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksDomainPage>>,
}
