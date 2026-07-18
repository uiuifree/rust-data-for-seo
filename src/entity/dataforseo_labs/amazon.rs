//! Amazon-specific item models for the DataForSEO Labs API.

use crate::entity::{DataForSeoLabsKeywordData, DataForSeoLabsMonthlySearch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One keyword an Amazon product (ASIN) ranks for.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRankedKeywordItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The ranked SERP position the target holds for this keyword.
    pub ranked_serp_element: Option<DataForSeoLabsAmazonRankedSerpElement>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
}

/// A ranked position on the Amazon SERP, wrapping the product item.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRankedSerpElement {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The SERP element occupying this ranked position.
    pub serp_item: Option<DataForSeoLabsAmazonSerpItem>,
    /// URL used to check the SERP.
    pub check_url: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
}

/// A single product listing on the Amazon SERP.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonSerpItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Type of SERP element (serialized as `type`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank among all SERP elements.
    pub rank_absolute: Option<i64>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result (for an id-list entry, the endpoint the task was posted to).
    pub url: Option<String>,
    /// Amazon product identifier (ASIN).
    pub asin: Option<String>,
    /// Lowest listed price, in `currency`.
    pub price_from: Option<f64>,
    /// Highest listed price, in `currency`.
    pub price_to: Option<f64>,
    /// ISO currency code of the prices.
    pub currency: Option<String>,
    /// Star rating of the product or app.
    pub rating: Option<DataForSeoLabsAmazonRating>,
    /// Whether the product carries the Best Seller badge.
    pub is_best_seller: Option<bool>,
    /// Whether the product carries the Amazon's Choice badge.
    pub is_amazon_choice: Option<bool>,
}

/// Star rating of an Amazon product.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/ranked_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRating {
    /// Type of the rating.
    pub rating_type: Option<String>,
    /// Rating value (e.g. average number of stars).
    pub value: Option<f64>,
    /// Number of ratings.
    pub votes_count: Option<i64>,
    /// Maximum possible rating value.
    pub rating_max: Option<i64>,
}

/// A keyword with its Amazon search volume.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/bulk_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonSearchVolumeItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword the data relates to.
    pub keyword: Option<String>,
    /// Google Ads keyword metrics.
    pub keyword_info: Option<DataForSeoLabsAmazonKeywordInfo>,
}

/// Amazon keyword metrics (a reduced set compared to Google).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/bulk_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonKeywordInfo {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
    /// Search volume for each of the preceding months.
    pub monthly_searches: Option<Vec<DataForSeoLabsMonthlySearch>>,
}

/// One keyword related to a seed keyword on Amazon.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/related_keywords/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonRelatedKeywordItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// Keyword expansion depth (0-4); higher values return more keywords.
    pub depth: Option<i64>,
    /// Keywords related to this keyword.
    pub related_keywords: Option<Vec<String>>,
}

/// A competing Amazon product (ASIN) and its ranking metrics.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductCompetitorItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Amazon product identifier (ASIN).
    pub asin: Option<String>,
    /// Average SERP position across intersecting keywords.
    pub avg_position: Option<f64>,
    /// Sum of SERP positions across intersecting keywords.
    pub sum_position: Option<i64>,
    /// Number of keywords the targets have in common.
    pub intersections: Option<i64>,
    /// Metrics limited to keywords shared with the target.
    pub competitor_metrics: Option<DataForSeoLabsAmazonMetrics>,
    /// Metrics across all of the app's keywords.
    pub full_metrics: Option<DataForSeoLabsAmazonMetrics>,
}

/// Ranking metrics of an Amazon product in one location/language.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_rank_overview/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductRankOverviewItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Location code (e.g. `2840`); alternative to `location_name`.
    pub location_code: Option<i32>,
    /// Language code (e.g. `"en"`); alternative to `language_name`.
    pub language_code: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<DataForSeoLabsAmazonMetrics>,
}

/// Amazon ranking metrics grouped by SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonMetrics {
    /// Metrics for Amazon organic results.
    pub amazon_serp: Option<DataForSeoLabsAmazonMetricsInfo>,
    /// Metrics for Amazon sponsored results.
    pub amazon_paid: Option<DataForSeoLabsAmazonMetricsInfo>,
}

/// Amazon ranking metrics for one SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonMetricsInfo {
    /// Number of SERPs where the target ranks #1.
    pub pos_1: Option<i64>,
    /// Number of SERPs where the target ranks #2-3.
    pub pos_2_3: Option<i64>,
    /// Number of SERPs where the target ranks #4-10.
    pub pos_4_10: Option<i64>,
    /// Number of SERPs where the target ranks #11-100.
    pub pos_11_100: Option<i64>,
    /// Number of SERPs containing the target for this feature type.
    pub count: Option<i64>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
}

/// A keyword and the Amazon SERP element each analyzed product holds for it.
/// `intersection_result` is keyed by the asins-map keys from the request.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/amazon/product_keyword_intersections/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAmazonProductKeywordIntersectionItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// SERP element each analyzed target holds for the keyword, keyed by target.
    pub intersection_result: Option<HashMap<String, DataForSeoLabsAmazonSerpItem>>,
}
