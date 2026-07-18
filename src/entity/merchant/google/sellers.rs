use crate::entity::MerchantApiRating;
use serde::{Deserialize, Serialize};

/// Result of a Google Shopping sellers task (`task_get/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/google/sellers/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleSellers {
    /// Google Shopping product identifier the sellers were requested for.
    pub product_id: Option<String>,
    /// Result type identifier returned by the API (e.g. `shops_list`).
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Google domain the lookup was run on (e.g. `google.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to (e.g. `en`).
    pub language_code: Option<String>,
    /// Direct URL to the Google Shopping sellers page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Title of the product the sellers offer.
    pub title: Option<String>,
    /// URL of the product page on Google Shopping.
    pub url: Option<String>,
    /// URL of the product image.
    pub image_url: Option<String>,
    /// Aggregated product rating.
    pub rating: Option<MerchantApiRating>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed seller listing elements.
    pub items: Option<Vec<MerchantApiGoogleSellerItem>>,
}

/// A single seller listing of a Google Shopping sellers result
/// (`shops_list`, `buy_on_google`).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleSellerItem {
    /// Element type (`shops_list`, `buy_on_google`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the listing within its own `item_type`.
    pub rank_group: Option<i64>,
    /// Absolute rank of the listing across all items.
    pub rank_absolute: Option<i64>,
    /// Position of the listing on the page (e.g. `left`).
    pub position: Option<String>,
    /// XPath of the element in the page markup.
    pub xpath: Option<String>,
    /// Domain of the merchant offering the product.
    pub domain: Option<String>,
    /// Product title as shown by the seller.
    pub title: Option<String>,
    /// URL of the seller's product page.
    pub url: Option<String>,
    /// Additional seller-provided details.
    pub details: Option<String>,
    /// Base product price before tax and shipping, in the units of `currency`.
    pub base_price: Option<f64>,
    /// Tax added to the base price, in the units of `currency`.
    pub tax: Option<f64>,
    /// Shipping cost, in the units of `currency`.
    pub shipping_price: Option<f64>,
    /// Total price including tax and shipping, in the units of `currency`.
    pub total_price: Option<f64>,
    /// ISO 4217 currency code of the price values.
    pub currency: Option<String>,
    /// Display name of the seller.
    pub seller_name: Option<String>,
    /// Seller rating.
    pub rating: Option<MerchantApiRating>,
    /// Ad click-tracking URL for sponsored listings.
    pub shop_ad_aclk: Option<String>,
    /// Item condition (e.g. `New`, `Used`).
    pub product_condition: Option<String>,
    /// Additional annotation shown on the listing.
    pub product_annotation: Option<String>,
    /// Availability status (e.g. `in_stock`, `out_of_stock`).
    pub product_availability: Option<String>,
}
