use crate::entity::BacklinksApiElementBacklinksCompetitor;
use serde::{Deserialize, Serialize};

/// Result of the Competitors endpoint: domains sharing backlinks with a target.
/// See <https://docs.dataforseo.com/v3/backlinks/competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiCompetitors {
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksCompetitor>>,
}
