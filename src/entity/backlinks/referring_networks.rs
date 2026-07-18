use crate::entity::BacklinksApiElementBacklinksReferringNetwork;
use serde::{Deserialize, Serialize};

/// Result of the Referring Networks endpoint: IPs or subnets linking to a target.
/// See <https://docs.dataforseo.com/v3/backlinks/referring_networks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiReferringNetworks {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksReferringNetwork>>,
}
