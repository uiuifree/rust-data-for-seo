use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A previously created OnPage task listed by the id_list endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiIdList {
    /// Unique task identifier.
    pub id: Option<String>,
    /// Endpoint URL used to retrieve the task's result.
    pub url: Option<String>,
    /// Date and time the task was posted (UTC).
    pub datetime_posted: Option<String>,
    /// Date and time the task was completed (UTC).
    pub datetime_done: Option<String>,
    /// Current status of the task.
    pub status: Option<String>,
    /// Cost of the task in USD.
    pub cost: Option<f64>,
    /// Additional metadata describing the task parameters.
    pub metadata: Option<Value>,
}
