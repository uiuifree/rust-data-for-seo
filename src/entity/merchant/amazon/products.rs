use crate::entity::{MerchantApiDeliveryInfo, MerchantApiRating, MerchantApiSpell};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of an Amazon products task (`task_get/advanced`, `live/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/amazon/products/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonProducts {
    /// Search keyword the results were returned for.
    pub keyword: Option<String>,
    /// Result type identifier returned by the API.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Amazon domain the search was run on (e.g. `amazon.com`).
    pub se_domain: Option<String>,
    /// Location code the search was localized to.
    pub location_code: Option<i32>,
    /// Language code the search was localized to (e.g. `en_US`).
    pub language_code: Option<String>,
    /// Direct URL to the Amazon search results page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Spelling autocorrection applied to the keyword, if any.
    pub spell: Option<MerchantApiSpell>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Total number of results Amazon reported for the query.
    pub se_results_count: Option<i64>,
    /// Amazon departments / subcategories detected for the query.
    pub categories: Option<Vec<Value>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed search-result elements.
    pub items: Option<Vec<MerchantApiAmazonProductsItem>>,
}

/// A single element of an Amazon products result (`amazon_serp`, `amazon_paid`, ...).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonProductsItem {
    /// Element type (e.g. `amazon_serp`, `amazon_paid`, `related_searches`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the element within its own `item_type`.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across all items.
    pub rank_absolute: Option<i64>,
    /// Position of the element on the page (e.g. `left`).
    pub position: Option<String>,
    /// XPath of the element in the page markup.
    pub xpath: Option<String>,
    /// Amazon domain the element was found on.
    pub domain: Option<String>,
    /// Product title.
    pub title: Option<String>,
    /// URL of the product page.
    pub url: Option<String>,
    /// URL of the product image.
    pub image_url: Option<String>,
    /// Number of units bought in the past month, as reported by Amazon.
    pub bought_past_month: Option<i64>,
    /// Lower bound of the product price, in the units of `currency`.
    pub price_from: Option<f64>,
    /// Upper bound of the product price, in the units of `currency`.
    pub price_to: Option<f64>,
    /// ISO 4217 currency code of the price values.
    pub currency: Option<String>,
    /// Special offers and discounts attached to the product.
    pub special_offers: Option<Vec<Value>>,
    /// Amazon Standard Identification Number (ASIN) of the product.
    pub data_asin: Option<String>,
    /// Product rating.
    pub rating: Option<MerchantApiRating>,
    /// `true` when the product carries the "Amazon's Choice" badge.
    pub is_amazon_choice: Option<bool>,
    /// `true` when the product carries the "Best Seller" badge.
    pub is_best_seller: Option<bool>,
    /// Delivery / shipping details for the product.
    pub delivery_info: Option<MerchantApiDeliveryInfo>,
}
