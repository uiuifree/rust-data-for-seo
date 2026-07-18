use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Result item of `content_analysis/id_list`: a previously set task and its
/// metadata.
/// See <https://docs.dataforseo.com/v3/content_analysis/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiIdList {
    /// Task ID.
    pub id: Option<String>,
    /// Endpoint URL the task was posted to.
    pub url: Option<String>,
    /// UTC datetime the task was created (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub datetime_posted: Option<String>,
    /// UTC datetime the task completed (`yyyy-mm-dd hh-mm-ss +00:00`).
    pub datetime_done: Option<String>,
    /// Task informational status message.
    pub status: Option<String>,
    /// Cost of the task, in USD.
    pub cost: Option<f64>,
    /// Parameters supplied in the original POST request.
    pub metadata: Option<HashMap<String, Value>>,
}
