use serde::{Deserialize, Serialize};

/// A single competitor item returned by the Competitors endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksCompetitor {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
    /// Number of backlinks shared with the target.
    pub intersections: Option<i32>,
}
