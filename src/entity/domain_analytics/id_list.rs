use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single task record returned by the Domain Analytics `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiIdList {
    /// Identifier of the task.
    pub id: Option<String>,
    /// URL path the task was posted to.
    pub url: Option<String>,
    /// Time the task was created (UTC).
    pub datetime_posted: Option<String>,
    /// Time the task finished (UTC).
    pub datetime_done: Option<String>,
    /// Task status code, as text.
    pub status: Option<String>,
    /// Cost of the task in USD.
    pub cost: Option<f64>,
    /// Original request parameters, present when metadata was requested.
    pub metadata: Option<Value>,
}
