use crate::entity::{MerchantApiDeliveryInfo, MerchantApiRating, MerchantApiSpell};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Google Shopping products task (`task_get/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/google/products/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProducts {
    /// Search keyword the results were returned for.
    pub keyword: Option<String>,
    /// Result type identifier returned by the API.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Google domain the search was run on (e.g. `google.com`).
    pub se_domain: Option<String>,
    /// Location code the search was localized to.
    pub location_code: Option<i32>,
    /// Language code the search was localized to (e.g. `en`).
    pub language_code: Option<String>,
    /// Direct URL to the Google Shopping results page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Spelling autocorrection applied to the keyword, if any.
    pub spell: Option<MerchantApiSpell>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Total number of results Google reported for the query.
    pub se_results_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed search-result elements.
    pub items: Option<Vec<MerchantApiGoogleProductsItem>>,
}

/// A single element of a Google Shopping products result
/// (`google_shopping_serp`, `google_shopping_paid`, ...).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductsItem {
    /// Element type (e.g. `google_shopping_serp`, `google_shopping_paid`).
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
    /// Domain of the merchant offering the product.
    pub domain: Option<String>,
    /// Product title.
    pub title: Option<String>,
    /// Product description snippet.
    pub description: Option<String>,
    /// URL of the merchant's product page.
    pub url: Option<String>,
    /// URL of the product on Google Shopping.
    pub shopping_url: Option<String>,
    /// Annotation tags shown on the listing (e.g. `free shipping`).
    pub tags: Option<Vec<String>>,
    /// Product price, in the units of `currency`.
    pub price: Option<f64>,
    /// Multiplier applied to the price when sold in packs.
    pub price_multiplier: Option<f64>,
    /// Previous (pre-discount) price, in the units of `currency`.
    pub old_price: Option<f64>,
    /// ISO 4217 currency code of the price values.
    pub currency: Option<String>,
    /// Google Shopping product identifier.
    pub product_id: Option<String>,
    /// Unique identifier of the SERP data element.
    pub data_docid: Option<String>,
    /// Name of the seller offering the product.
    pub seller: Option<String>,
    /// Extra specification parameters carried over to the sellers endpoint.
    pub additional_specifications: Option<Value>,
    /// Number of product reviews.
    pub reviews_count: Option<i64>,
    /// `true` when Google marks the listing as the best match.
    pub is_best_match: Option<bool>,
    /// Aggregated rating of the product.
    pub product_rating: Option<MerchantApiRating>,
    /// Rating of the shop offering the product.
    pub shop_rating: Option<MerchantApiRating>,
    /// URLs of the product images.
    pub product_images: Option<Vec<String>>,
    /// Ad click-tracking URL for sponsored listings.
    pub shop_ad_aclk: Option<String>,
    /// Delivery / shipping details for the product.
    pub delivery_info: Option<MerchantApiDeliveryInfo>,
    /// Information about how many stores offer the product.
    pub stores_count_info: Option<Value>,
    /// Global product identifier on Google Shopping.
    pub gid: Option<String>,
}
