//! Shared/utility endpoint models for the DataForSEO Labs API.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A previously posted task returned by the `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsIdListItem {
    /// Task identifier (UUID).
    pub id: Option<String>,
    /// When the task was created (UTC).
    pub datetime_posted: Option<String>,
    /// When the task finished (UTC).
    pub datetime_done: Option<String>,
    /// URL of the result (for an id-list entry, the endpoint the task was posted to).
    pub url: Option<String>,
    /// Task status.
    pub status: Option<String>,
    /// Cost of the task (USD).
    pub cost: Option<f64>,
    /// Original POST parameters of the task.
    pub metadata: Option<Value>,
}

/// A location supported by the Labs API, with the languages available for it.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/locations_and_languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsLocation {
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i64>,
    /// Full location name (e.g. `"United States"`); alternative to `location_code`.
    pub location_name: Option<String>,
    /// Code of the parent location.
    pub location_code_parent: Option<i64>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. `"Country"`, `"City"`).
    pub location_type: Option<String>,
    /// Languages available for this location.
    pub available_languages: Option<Vec<DataForSeoLabsLanguage>>,
}

/// A language available for a location.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/locations_and_languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsLanguage {
    /// Full language name (e.g. `"English"`); alternative to `language_code`.
    pub language_name: Option<String>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Number of keywords available in the database for this language.
    pub keywords: Option<i64>,
    /// Number of SERPs available in the database for this language.
    pub serps: Option<i64>,
}

/// A node of the product-category tree returned by `categories`.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/categories/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsCategory {
    /// Category code.
    pub category_code: Option<i64>,
    /// Human-readable category name.
    pub category_name: Option<String>,
    /// Code of the parent category.
    pub category_code_parent: Option<i64>,
}

/// Rate-limit / availability info returned by the Labs `status` endpoint.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/status/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsStatus {
    /// Date and time of the record (UTC).
    pub datetime: Option<String>,
    /// Rate-limit information.
    pub rates: Option<Value>,
    /// Available metric types and their limits.
    pub metric_types: Option<Value>,
}
