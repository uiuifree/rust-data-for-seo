//! Shared entity types used across the Business Data API domain.
//!
//! See <https://docs.dataforseo.com/v3/business_data/overview/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Rating object returned by most Business Data results and items.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiRating {
    /// Rating scale, e.g. `"Max5"` for a 1-5 star scale.
    pub rating_type: Option<String>,
    /// Rating value on the scale given by `rating_type`.
    pub value: Option<f64>,
    /// Number of votes that contributed to the rating.
    pub votes_count: Option<i64>,
    /// Maximum value the rating can reach (e.g. `5`).
    pub rating_max: Option<i64>,
}

/// Structured breakdown of an address into its components.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiAddressInfo {
    /// Borough or neighborhood the business is located in.
    pub borough: Option<String>,
    /// Street address (house number and street name).
    pub address: Option<String>,
    /// City the business is located in.
    pub city: Option<String>,
    /// Postal or ZIP code.
    pub zip: Option<String>,
    /// Region, state, or province.
    pub region: Option<String>,
    /// Two-letter ISO country code (e.g. `"US"`).
    pub country_code: Option<String>,
}

/// A single row returned by a `tasks_ready` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTaskReady {
    /// Identifier of the completed task, used to retrieve results.
    pub id: Option<String>,
    /// Search engine the task was set for (e.g. `"google"`).
    pub se: Option<String>,
    /// Type of the completed task (e.g. `"reviews"`).
    pub se_type: Option<String>,
    /// UTC time the task was posted.
    pub date_posted: Option<String>,
    /// User-defined task identifier supplied on `task_post`.
    pub tag: Option<String>,
    /// Endpoint to call to collect the task's results.
    pub endpoint: Option<String>,
}

/// Request body for the `id_list` and `errors` endpoints.
/// See <https://docs.dataforseo.com/v3/business_data/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiIdListRequest {
    /// Start of the time range, in `yyyy-mm-dd hh-mm-ss +00:00` format.
    pub datetime_from: String,
    /// End of the time range, in `yyyy-mm-dd hh-mm-ss +00:00` format.
    pub datetime_to: String,
    /// Maximum number of rows to return (default `1000`, max `1000`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of rows to skip before returning results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sorting rule applied to the returned rows (e.g. `"datetime,asc"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// When `true`, includes per-task metadata in each row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl BusinessDataApiIdListRequest {
    /// Builds a request bounded by the `[datetime_from, datetime_to]` window.
    pub fn new<T: Into<String>>(datetime_from: T, datetime_to: T) -> Self {
        BusinessDataApiIdListRequest {
            datetime_from: datetime_from.into(),
            datetime_to: datetime_to.into(),
            ..BusinessDataApiIdListRequest::default()
        }
    }
}

/// A single row returned by the `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiIdList {
    /// Identifier of the task.
    pub id: Option<String>,
    /// Endpoint the task was set on.
    pub url: Option<String>,
    /// UTC time the task was posted.
    pub datetime_posted: Option<String>,
    /// UTC time the task finished processing.
    pub datetime_done: Option<String>,
    /// Current processing status of the task.
    pub status: Option<String>,
    /// Money charged for the task, in USD.
    pub cost: Option<f64>,
    /// Per-task metadata, present when `include_metadata` was requested.
    pub metadata: Option<Value>,
}

/// A single row returned by the `errors` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiErrorItem {
    /// Identifier of the task that produced the error.
    pub id: Option<String>,
    /// UTC time the error occurred.
    pub datetime: Option<String>,
    /// Internal function that raised the error.
    pub function: Option<String>,
    /// DataForSEO error code.
    pub error_code: Option<i32>,
    /// Human-readable error message.
    pub error_message: Option<String>,
    /// URL involved in the failed request, when applicable.
    pub http_url: Option<String>,
    /// HTTP method of the failed request.
    pub http_method: Option<String>,
    /// HTTP status code returned by the failed request.
    pub http_code: Option<i32>,
    /// Time the failed request took, in seconds.
    pub http_time: Option<f64>,
    /// Raw HTTP response body of the failed request.
    pub http_response: Option<String>,
}
