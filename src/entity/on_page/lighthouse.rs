use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Lighthouse audit results for a page.
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_get/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiLighthouseTaskGet {
    /// User agent Lighthouse emulated while auditing the page.
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    /// Details of the environment the audit ran in.
    pub environment: Option<OnPageDataApiLighthouseEnvironment>,
    /// Version of Lighthouse used for the audit.
    #[serde(rename = "lighthouseVersion")]
    pub lighthouse_version: Option<String>,
    /// Timestamp when the page was fetched for auditing.
    #[serde(rename = "fetchTime")]
    pub fetch_time: Option<String>,
    /// URL originally requested for auditing.
    #[serde(rename = "requestedUrl")]
    pub requested_url: Option<String>,
    /// Final URL audited after following redirects.
    #[serde(rename = "finalUrl")]
    pub final_url: Option<String>,
    /// Warnings raised during the audit run.
    #[serde(rename = "runWarnings")]
    pub run_warnings: Option<Vec<Value>>,
    /// Raw individual audit results keyed by audit id.
    pub audits: Option<Value>,
    /// Category scores (performance, accessibility, SEO, etc.).
    pub categories: Option<Value>,
}

/// Environment in which the Lighthouse audit was executed.
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_get/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiLighthouseEnvironment {
    /// User agent used for network requests during the audit.
    #[serde(rename = "networkUserAgent")]
    pub network_user_agent: Option<String>,
    /// User agent of the host that ran the audit.
    #[serde(rename = "hostUserAgent")]
    pub host_user_agent: Option<String>,
    /// CPU/performance benchmark score of the audit host.
    #[serde(rename = "benchmarkIndex")]
    pub benchmark_index: Option<f64>,
    /// Credits for the libraries used during the audit.
    pub credits: Option<Value>,
}

/// A Lighthouse task ready to be collected.
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_get/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiLighthouseTasksReady {
    /// Unique task identifier.
    pub id: Option<String>,
    /// Date and time the task was posted (UTC).
    pub date_posted: Option<String>,
    /// User-defined tag assigned to the task.
    pub tag: Option<String>,
    /// Endpoint to call to retrieve the task's result.
    pub endpoint: Option<String>,
}
