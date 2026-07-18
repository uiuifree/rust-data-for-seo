use serde::{Deserialize, Serialize};

/// A single error record returned by the OnPage Errors endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiError {
    /// Unique task identifier (UUID).
    pub id: Option<String>,
    /// Date and time the error occurred (UTC).
    pub datetime: Option<String>,
    /// API function the task called.
    pub function: Option<String>,
    /// DataForSEO error code.
    pub error_code: Option<i32>,
    /// Human-readable error description.
    pub error_message: Option<String>,
    /// URL of the API call that produced the error.
    pub http_url: Option<String>,
    /// HTTP method used for the failed call.
    pub http_method: Option<String>,
    /// HTTP status code returned for the failed call.
    pub http_code: Option<i32>,
    /// Duration of the HTTP request in seconds.
    pub http_time: Option<f64>,
    /// Body of the server response for the failed call.
    pub http_response: Option<String>,
}
