use crate::entity::{MerchantApiDeliveryInfo, MerchantApiPrice, MerchantApiRating};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Google Shopping product_info task (`task_get/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/google/product_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductInfo {
    /// Google Shopping product identifier the information was requested for.
    pub product_id: Option<String>,
    /// Result type identifier returned by the API.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Google domain the lookup was run on (e.g. `google.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to (e.g. `en`).
    pub language_code: Option<String>,
    /// Direct URL to the Google Shopping product page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed product information elements.
    pub items: Option<Vec<MerchantApiGoogleProductInfoItem>>,
}

/// A product information element of a Google Shopping product_info result.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductInfoItem {
    /// Element type identifier.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the element within its own `item_type`.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across all items.
    pub rank_absolute: Option<i64>,
    /// Position of the element on the page (e.g. `left`).
    pub position: Option<String>,
    /// Google Shopping product identifier.
    pub product_id: Option<String>,
    /// Product title.
    pub title: Option<String>,
    /// Product description text.
    pub description: Option<String>,
    /// URL of the product page on Google Shopping.
    pub url: Option<String>,
    /// URLs of the product images.
    pub images: Option<Vec<String>>,
    /// Highlighted product features.
    pub features: Option<Vec<Value>>,
    /// Aggregated product rating.
    pub rating: Option<MerchantApiRating>,
    /// Number of seller reviews aggregated for the product.
    pub seller_reviews_count: Option<i64>,
    /// Unique identifier of the SERP data element.
    pub data_docid: Option<String>,
    /// Global product identifier on Google Shopping.
    pub gid: Option<String>,
    /// Technical specifications of the product.
    pub specifications: Option<Vec<MerchantApiGoogleProductSpecification>>,
    /// Sellers offering the product.
    pub sellers: Option<Vec<MerchantApiGoogleProductSeller>>,
    /// Available variations of the product.
    pub variations: Option<Vec<MerchantApiGoogleProductVariation>>,
}

/// A seller offer nested inside a Google Shopping product_info element.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductSeller {
    /// Element type identifier.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Display name of the seller.
    pub title: Option<String>,
    /// URL of the seller's product page.
    pub url: Option<String>,
    /// Rating of the seller.
    pub seller_rating: Option<MerchantApiRating>,
    /// Number of reviews the seller rating is based on.
    pub seller_reviews_count: Option<i64>,
    /// Offer price.
    pub price: Option<MerchantApiPrice>,
    /// Delivery / shipping details for the offer.
    pub delivery_info: Option<MerchantApiDeliveryInfo>,
    /// Availability status (e.g. `in_stock`, `out_of_stock`).
    pub product_availability: Option<String>,
}

/// A specification row nested inside a Google Shopping product_info element.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductSpecification {
    /// Element type identifier.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Name of the specification block this row belongs to.
    pub block_name: Option<String>,
    /// Name of the specification (e.g. `Color`).
    pub specification_name: Option<String>,
    /// Value of the specification (e.g. `Black`).
    pub specification_value: Option<String>,
}

/// A product variation nested inside a Google Shopping product_info element.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductVariation {
    /// Element type identifier.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Google Shopping product identifier of the variation.
    pub product_id: Option<String>,
    /// Global product identifier of the variation.
    pub gid: Option<String>,
    /// Product variant filter value identifying the variation.
    pub pvf: Option<String>,
    /// Title of the variation.
    pub title: Option<String>,
    /// URL of the variation's product page.
    pub url: Option<String>,
    /// Category of the variation (e.g. `size`, `color`).
    pub variation_category: Option<String>,
}
