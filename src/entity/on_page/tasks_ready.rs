use serde::{Deserialize, Serialize};

/// An OnPage task ready to be collected.
/// See <https://docs.dataforseo.com/v3/on_page/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiTasksReady {
    /// Unique task identifier.
    pub id: Option<String>,
    /// Target domain the task was created for.
    pub target: Option<String>,
    /// Date and time the task was posted (UTC).
    pub date_posted: Option<String>,
    /// User-defined tag assigned to the task.
    pub tag: Option<String>,
}
