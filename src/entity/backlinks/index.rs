use serde::{Deserialize, Serialize};

/// Result of the Index endpoint: total volume of the DataForSEO backlinks database.
/// See <https://docs.dataforseo.com/v3/backlinks/index/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIndex {
    /// Total number of backlinks in the database.
    pub total_backlinks: Option<u64>,
    /// Total number of pages in the database.
    pub total_pages: Option<u64>,
    /// Total number of domains in the database.
    pub total_domains: Option<u64>,
    /// Monthly snapshots of the index volume over the past 12 months.
    pub index_history: Option<Vec<BacklinksApiIndexHistory>>,
}

/// A single monthly snapshot of the backlinks index volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIndexHistory {
    /// Date and time of this data point (UTC).
    pub date: Option<String>,
    /// Total number of backlinks in the database.
    pub total_backlinks: Option<u64>,
    /// Total number of pages in the database.
    pub total_pages: Option<u64>,
    /// Total number of domains in the database.
    pub total_domains: Option<u64>,
}
