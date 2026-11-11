//! Task listing models shared by the API domains.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request body for the `id_list` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoIdListRequest {
    /// Start of the time range (UTC).
    pub datetime_from: String,
    /// End of the time range (UTC).
    pub datetime_to: String,
    /// Maximum number of returned task IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort order by execution time (`asc` or `desc`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Include the task metadata in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl DataForSeoIdListRequest {
    /// Creates a request for the `[datetime_from, datetime_to]` range.
    pub fn new<F: Into<String>, T: Into<String>>(datetime_from: F, datetime_to: T) -> Self {
        DataForSeoIdListRequest {
            datetime_from: datetime_from.into(),
            datetime_to: datetime_to.into(),
            ..Default::default()
        }
    }
}

/// A task returned by the `id_list` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoIdListItem {
    /// Task identifier.
    pub id: Option<String>,
    /// URL used for the API call.
    pub url: Option<String>,
    /// UTC time the task was made.
    pub datetime_posted: Option<String>,
    /// UTC time the task was completed.
    pub datetime_done: Option<String>,
    /// Status code of the task. The API returns a number although the docs describe a message.
    pub status: Option<serde_json::Value>,
    /// Task cost in USD.
    pub cost: Option<f64>,
    /// Parameters specified in the POST request.
    pub metadata: Option<Value>,
}

/// Request body for the `errors` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoErrorsRequest {
    /// Maximum number of returned tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset in the results array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Return only tasks of this function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_function: Option<String>,
    /// Start of the time range (UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_from: Option<String>,
    /// End of the time range (UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_to: Option<String>,
}

/// A task returned by the `errors` endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoErrorItem {
    /// Task identifier.
    pub id: Option<String>,
    /// UTC time the error occurred.
    pub datetime: Option<String>,
    /// API function that raised the error.
    pub function: Option<String>,
    /// DataForSEO error code.
    pub error_code: Option<i32>,
    /// Error message.
    pub error_message: Option<String>,
    /// URL that caused the error.
    pub http_url: Option<String>,
    /// HTTP method that caused the error.
    pub http_method: Option<String>,
    /// HTTP status code.
    pub http_code: Option<i32>,
    /// Time taken by the HTTP request, in seconds.
    pub http_time: Option<f64>,
    /// Server response.
    pub http_response: Option<String>,
}
