use crate::entity::BacklinksApiElementBacklinksTimeseriesSummary;
use serde::{Deserialize, Serialize};

/// Result of the Timeseries Summary endpoint: backlink metrics grouped over time.
/// See <https://docs.dataforseo.com/v3/backlinks/timeseries_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiTimeseriesSummary {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Start date of the range (`yyyy-mm-dd`).
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`).
    pub date_to: Option<String>,
    /// Grouping interval: `day`, `week`, `month`, or `year`.
    pub group_range: Option<String>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i32>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksTimeseriesSummary>>,
}
