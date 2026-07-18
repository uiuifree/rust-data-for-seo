use crate::entity::{MerchantApiRating, MerchantApiSpell};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of an Amazon ASIN task (`task_get/advanced`, `live/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/amazon/asin/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonAsin {
    /// ASIN the product information was requested for.
    pub asin: Option<String>,
    /// Result type identifier returned by the API.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Amazon domain the lookup was run on (e.g. `amazon.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to (e.g. `en_US`).
    pub language_code: Option<String>,
    /// Direct URL to the Amazon product page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Spelling autocorrection applied to the query, if any.
    pub spell: Option<MerchantApiSpell>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed product information elements.
    pub items: Option<Vec<MerchantApiAmazonProductInfoItem>>,
}

/// Product information element of an Amazon ASIN result (`amazon_product_info`).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonProductInfoItem {
    /// Element type (`amazon_product_info`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the element within its own `item_type`.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across all items.
    pub rank_absolute: Option<i32>,
    /// Position of the element on the page (e.g. `left`).
    pub position: Option<String>,
    /// XPath of the element in the page markup.
    pub xpath: Option<String>,
    /// Product title.
    pub title: Option<String>,
    /// Short product details / subtitle line.
    pub details: Option<String>,
    /// URL of the main product image.
    pub image_url: Option<String>,
    /// Product author (for books and similar media).
    pub author: Option<String>,
    /// ASIN of this product.
    pub data_asin: Option<String>,
    /// ASIN of the parent product when this is a variation.
    pub parent_asin: Option<String>,
    /// ASINs of the product's variations.
    pub product_asins: Option<Vec<String>>,
    /// Lower bound of the product price, in the units of `currency`.
    pub price_from: Option<f32>,
    /// Upper bound of the product price, in the units of `currency`.
    pub price_to: Option<f32>,
    /// Discount applied to the product, as a percentage.
    pub percentage_discount: Option<f32>,
    /// ISO 4217 currency code of the price values.
    pub currency: Option<String>,
    /// `true` when the product carries the "Amazon's Choice" badge.
    pub is_amazon_choice: Option<bool>,
    /// Product rating.
    pub rating: Option<MerchantApiRating>,
    /// `true` when Amazon links to a newer model of this product.
    pub is_newer_model_available: Option<bool>,
    /// `true` when the item is a Prime Video title.
    pub is_prime_video: Option<bool>,
    /// Vouchers that can be applied to the product.
    pub applicable_vouchers: Option<Vec<Value>>,
    /// Reference to the newer model when `is_newer_model_available` is set.
    pub newer_model: Option<Value>,
    /// Amazon categories the product belongs to.
    pub categories: Option<Vec<Value>>,
    /// Structured product information rows (specifications, details, ...).
    pub product_information: Option<Vec<Value>>,
    /// URLs of the product image gallery.
    pub product_images_list: Option<Vec<String>>,
    /// URLs of the product videos.
    pub product_videos_list: Option<Vec<String>>,
    /// Product description text.
    pub description: Option<String>,
    /// `true` when the product is currently available for purchase.
    pub is_available: Option<bool>,
    /// Top reviews from the requested marketplace.
    pub top_local_reviews: Option<Vec<Value>>,
    /// Top reviews aggregated across all marketplaces.
    pub top_global_reviews: Option<Vec<Value>>,
}
