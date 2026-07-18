use serde::{Deserialize, Serialize};

/// A single data point returned by the Timeseries New & Lost Summary endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/timeseries_new_lost_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksTimeseriesNewLostSummary {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Date and time of this data point (UTC).
    pub date: Option<String>,
    /// Number of newly discovered backlinks.
    pub new_backlinks: Option<i64>,
    /// Number of backlinks that were lost.
    pub lost_backlinks: Option<i64>,
    /// Number of newly discovered referring domains.
    pub new_referring_domains: Option<i64>,
    /// Number of referring domains that were lost.
    pub lost_referring_domains: Option<i64>,
    /// Number of newly discovered referring main domains.
    pub new_referring_main_domains: Option<i64>,
    /// Number of referring main domains that were lost.
    pub lost_referring_main_domains: Option<i64>,
}
