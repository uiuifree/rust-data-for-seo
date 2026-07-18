use crate::entity::{MerchantApiDeliveryInfo, MerchantApiPrice, MerchantApiRating};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of an Amazon sellers task (`task_get/advanced`, `live/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/amazon/sellers/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonSellers {
    /// ASIN the seller offers were requested for.
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
    /// Direct URL to the Amazon offer-listing page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Title of the product the offers belong to.
    pub title: Option<String>,
    /// URL of the product image.
    pub image: Option<String>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed seller offer elements.
    pub items: Option<Vec<MerchantApiAmazonSellerItem>>,
}

/// A single offer element of an Amazon sellers result
/// (`amazon_seller_main_item`, `amazon_seller_item`).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonSellerItem {
    /// Element type (`amazon_seller_main_item`, `amazon_seller_item`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the offer within its own `item_type`.
    pub rank_group: Option<i32>,
    /// Absolute rank of the offer across all items.
    pub rank_absolute: Option<i32>,
    /// Position of the offer on the page (e.g. `left`).
    pub position: Option<String>,
    /// XPath of the element in the page markup.
    pub xpath: Option<String>,
    /// Display name of the seller.
    pub seller_name: Option<String>,
    /// URL of the seller's Amazon storefront.
    pub seller_url: Option<String>,
    /// Party the item ships from.
    pub ships_from: Option<String>,
    /// Offer price.
    pub price: Option<MerchantApiPrice>,
    /// Discount applied to the offer, as a percentage.
    pub percentage_discount: Option<f32>,
    /// Vouchers that can be applied to the offer.
    pub applicable_vouchers: Option<Vec<Value>>,
    /// Seller rating.
    pub rating: Option<MerchantApiRating>,
    /// Item condition (e.g. `New`, `Used - Like New`).
    pub condition: Option<String>,
    /// Free-text description of the item condition.
    pub condition_description: Option<String>,
    /// Delivery / shipping details for the offer.
    pub delivery_info: Option<MerchantApiDeliveryInfo>,
}
