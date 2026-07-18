//! Entity models for the merchant API domain.
//!
//! See <https://docs.dataforseo.com/v3/merchant/>.

mod amazon;
mod google;

pub use amazon::*;
pub use google::*;
use serde::{Deserialize, Serialize};

/// A location supported by the Merchant API (`locations` endpoints).
/// See <https://docs.dataforseo.com/v3/merchant/amazon/locations/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiLocation {
    /// Unique location identifier used in `location_code` request fields.
    pub location_code: Option<i32>,
    /// Full location name (e.g. `"London,England,United Kingdom"`).
    pub location_name: Option<String>,
    /// `location_code` of the parent location in the hierarchy, if any.
    pub location_code_parent: Option<i32>,
    /// ISO 3166-1 country code the location belongs to.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. `Country`, `Region`, `City`).
    pub location_type: Option<String>,
}

/// A completed task returned by the `tasks_ready` endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/amazon/products/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiTaskReady {
    /// Identifier of the completed task, used to fetch its result.
    pub id: Option<String>,
    /// Search engine the task was run against (e.g. `amazon`, `google`).
    pub se: Option<String>,
    /// Type of the task (e.g. `products`, `asin`, `sellers`).
    pub se_type: Option<String>,
    /// UTC timestamp when the task was posted.
    pub date_posted: Option<String>,
    /// User-defined identifier supplied with the original task.
    pub tag: Option<String>,
    /// Ready-made path to fetch the advanced (parsed) result.
    pub endpoint_advanced: Option<String>,
    /// Ready-made path to fetch the raw HTML result.
    pub endpoint_html: Option<String>,
}

/// Spelling autocorrection applied to a search query.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiSpell {
    /// Autocorrected keyword the results were actually returned for.
    pub keyword: Option<String>,
    /// Kind of correction applied (e.g. `did_you_mean`, `including_results_for`).
    #[serde(rename = "type")]
    pub correction_type: Option<String>,
}

/// Price information attached to products and offers.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiPrice {
    /// Current price, in the units of `currency`.
    pub current: Option<f64>,
    /// Regular (pre-discount) price, in the units of `currency`.
    pub regular: Option<f64>,
    /// Upper bound of the price when a range is displayed.
    pub max_value: Option<f64>,
    /// ISO 4217 currency code of the price values.
    pub currency: Option<String>,
    /// `true` when the price is shown as a range rather than a single value.
    pub is_price_range: Option<bool>,
    /// Price exactly as rendered on the page (e.g. `"$799.00"`).
    pub displayed_price: Option<String>,
}

/// Rating information attached to products, sellers and reviews.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiRating {
    /// Rating scale type (e.g. `Max5`).
    pub rating_type: Option<String>,
    /// Rating value on the `rating_type` scale.
    pub value: Option<f64>,
    /// Number of votes the rating is based on.
    pub votes_count: Option<i64>,
    /// Maximum possible rating value (e.g. `5`).
    pub rating_max: Option<i64>,
}

/// Delivery / shipping details attached to products and offers.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiDeliveryInfo {
    /// Delivery message rendered on the page (e.g. `"FREE delivery"`).
    pub delivery_message: Option<String>,
    /// Shipping cost charged for the delivery.
    pub delivery_price: Option<MerchantApiPrice>,
    /// Earliest estimated delivery date for standard shipping.
    pub delivery_date_from: Option<String>,
    /// Latest estimated delivery date for standard shipping.
    pub delivery_date_to: Option<String>,
    /// Earliest estimated delivery date for the fastest shipping option.
    pub fastest_delivery_date_from: Option<String>,
    /// Latest estimated delivery date for the fastest shipping option.
    pub fastest_delivery_date_to: Option<String>,
}
