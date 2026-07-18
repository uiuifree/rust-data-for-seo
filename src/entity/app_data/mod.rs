//! Entity models for the app_data API domain.
//!
//! Covers Google Play and Apple App Store app search, listing, info and review
//! data. See <https://docs.dataforseo.com/v3/app_data/>.

mod app_info;
mod app_list;
mod app_reviews;
mod app_searches;

pub use app_info::*;
pub use app_list::*;
pub use app_reviews::*;
pub use app_searches::*;

use serde::{Deserialize, Serialize};

/// Rating block shared by app search, listing, info and review items.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiRating {
    /// Rating scale identifier (e.g. `Max5`).
    pub rating_type: Option<String>,
    /// Rating value on the scale.
    pub value: Option<f64>,
    /// Number of votes the rating is based on.
    pub votes_count: Option<i64>,
    /// Maximum value of the rating scale.
    pub rating_max: Option<i64>,
}

/// Price block shared by app search, listing and info items.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiPrice {
    /// Current price of the application.
    pub current: Option<f64>,
    /// Regular (non-discounted) price of the application.
    pub regular: Option<f64>,
    /// Upper bound of the price when it is a range.
    pub max_value: Option<f64>,
    /// ISO currency code of the price.
    pub currency: Option<String>,
    /// `true` when the price is a range rather than a single value.
    pub is_price_range: Option<bool>,
    /// Price as displayed in the store.
    pub displayed_price: Option<String>,
}

/// A location supported by the App Data endpoints.
/// See <https://docs.dataforseo.com/v3/app_data/google/locations/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiLocation {
    /// Unique location identifier used in requests.
    pub location_code: Option<i32>,
    /// Full location name.
    pub location_name: Option<String>,
    /// Location code of the parent location, if any.
    pub location_code_parent: Option<i32>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. `Country`).
    pub location_type: Option<String>,
}

/// A language supported by the App Data endpoints.
/// See <https://docs.dataforseo.com/v3/app_data/google/languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiLanguage {
    /// Full language name.
    pub language_name: Option<String>,
    /// Language code used in requests.
    pub language_code: Option<String>,
}

/// A category supported by the App Data endpoints.
/// See <https://docs.dataforseo.com/v3/app_data/google/categories/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiCategory {
    /// Unique category identifier used in requests.
    pub category_code: Option<String>,
    /// Full category name.
    pub category_name: Option<String>,
    /// Category code of the parent category, if any.
    pub category_code_parent: Option<String>,
}

/// A ready task returned by the `tasks_ready` endpoints.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_searches/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiTaskReadyResult {
    /// Identifier of the completed task.
    pub id: Option<String>,
    /// Search engine the task was set for.
    pub se: Option<String>,
    /// Type of the task (endpoint the task was set for).
    pub se_type: Option<String>,
    /// UTC timestamp when the task was set.
    pub date_posted: Option<String>,
    /// User-defined task identifier supplied on `task_post`.
    pub tag: Option<String>,
    /// Relative path used to fetch the advanced result.
    pub endpoint_advanced: Option<String>,
    /// Relative path used to fetch the raw HTML result.
    pub endpoint_html: Option<String>,
}

/// An error record returned by the App Data `errors` endpoint.
/// See <https://docs.dataforseo.com/v3/app_data/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiError {
    /// Identifier of the task that errored.
    pub id: Option<String>,
    /// UTC timestamp when the error occurred.
    pub datetime: Option<String>,
    /// API function that produced the error.
    pub function: Option<String>,
    /// DataForSEO error code.
    pub error_code: Option<i32>,
    /// Human-readable error message.
    pub error_message: Option<String>,
    /// URL that caused the error.
    pub http_url: Option<String>,
    /// HTTP method used for the failed request.
    pub http_method: Option<String>,
    /// HTTP status code returned.
    pub http_code: Option<i32>,
    /// Response time of the failed request, in seconds.
    pub http_time: Option<f64>,
    /// Raw response body of the failed request.
    pub http_response: Option<String>,
}

/// A task record returned by the App Data `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/app_data/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiIdList {
    /// Task identifier.
    pub id: Option<String>,
    /// URL of the API call that set the task.
    pub url: Option<String>,
    /// UTC timestamp when the task was set.
    pub datetime_posted: Option<String>,
    /// UTC timestamp when the task completed.
    pub datetime_done: Option<String>,
    /// Task status code.
    pub status: Option<String>,
    /// Cost of the task, in USD.
    pub cost: Option<f64>,
}
