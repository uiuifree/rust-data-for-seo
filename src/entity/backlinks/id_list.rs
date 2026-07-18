use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single task-id record returned by the ID List endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIdListItem {
    /// Unique task identifier (UUID).
    pub id: Option<String>,
    /// Endpoint path of the task (e.g. `v3/backlinks/backlinks/live`).
    pub url: Option<String>,
    /// Date and time the task was created (UTC).
    pub datetime_posted: Option<String>,
    /// Date and time the task finished (UTC).
    pub datetime_done: Option<String>,
    /// Task status code as a string (e.g. "20000").
    pub status: Option<String>,
    /// Cost of the task in USD.
    pub cost: Option<f64>,
    /// Original POST request parameters of the task.
    pub metadata: Option<Value>,
}
