use serde::{Deserialize, Serialize};

/// A single failed task reported by the Domain Analytics `errors` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiErrorItem {
    /// Identifier of the failed task.
    pub id: Option<String>,
    /// Time the error occurred (UTC).
    pub datetime: Option<String>,
    /// API function that produced the error.
    pub function: Option<String>,
    /// DataForSEO status code of the error.
    pub error_code: Option<i32>,
    /// Human-readable error message.
    pub error_message: Option<String>,
    /// URL that was requested when the error occurred.
    pub http_url: Option<String>,
    /// HTTP method used for the failed request.
    pub http_method: Option<String>,
    /// HTTP status code returned for the failed request.
    pub http_code: Option<i32>,
    /// Time (seconds) the failed request took.
    pub http_time: Option<f64>,
    /// Raw HTTP response body captured for the error.
    pub http_response: Option<String>,
}
