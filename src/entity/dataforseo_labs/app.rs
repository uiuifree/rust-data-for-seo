//! App-store item models (Google Play and Apple App Store) for the Labs API.

use crate::entity::DataForSeoLabsKeywordData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One keyword an app ranks for.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppKeywordItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The ranked SERP position the target holds for this keyword.
    pub ranked_serp_element: Option<DataForSeoLabsAppRankedSerpElement>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
}

/// A ranked position on the app-store SERP, wrapping the app item.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppRankedSerpElement {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// The SERP element occupying this ranked position.
    pub serp_item: Option<DataForSeoLabsAppSerpItem>,
    /// URL used to check the SERP.
    pub check_url: Option<String>,
    /// Timestamp when this data was last updated (UTC).
    pub last_updated_time: Option<String>,
}

/// A single app listing on the app-store SERP.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppSerpItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Type of SERP element (serialized as `type`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank among all SERP elements.
    pub rank_absolute: Option<i64>,
    /// Store identifier of the app.
    pub app_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result (for an id-list entry, the endpoint the task was posted to).
    pub url: Option<String>,
    /// URL of the app icon.
    pub icon: Option<String>,
    /// Whether the app is free.
    pub is_free: Option<bool>,
    /// Price of the app.
    pub price: Option<DataForSeoLabsAppPrice>,
    /// Star rating of the product or app.
    pub rating: Option<DataForSeoLabsAppRating>,
}

/// Price of an app listing.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppPrice {
    /// Current price value.
    pub current: Option<f64>,
    /// ISO currency code of the prices.
    pub currency: Option<String>,
}

/// Star rating of an app listing.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/keywords_for_app/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppRating {
    /// Type of the rating.
    pub rating_type: Option<String>,
    /// Rating value (e.g. average number of stars).
    pub value: Option<f64>,
    /// Number of ratings.
    pub votes_count: Option<i64>,
    /// Maximum possible rating value.
    pub rating_max: Option<i64>,
}

/// Ranking metrics of an app for one SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_app_metrics/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppMetricsInfo {
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

/// A competing app and its ranking-metric intersection with the target app.
/// The metric maps are keyed by SERP feature type (e.g. `"google_play_organic"`).
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_competitors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppCompetitorItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Store identifier of the app.
    pub app_id: Option<String>,
    /// Average SERP position across intersecting keywords.
    pub avg_position: Option<f64>,
    /// Sum of SERP positions across intersecting keywords.
    pub sum_position: Option<i64>,
    /// Number of keywords the targets have in common.
    pub intersections: Option<i64>,
    /// Metrics limited to keywords shared with the target.
    pub competitor_metrics: Option<HashMap<String, DataForSeoLabsAppMetricsInfo>>,
    /// Metrics across all of the app's keywords.
    pub full_metrics: Option<HashMap<String, DataForSeoLabsAppMetricsInfo>>,
}

/// A keyword and the SERP element each analyzed app holds for it.
/// `intersection_result` is keyed by the app-ids-map keys from the request.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/app_intersection/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsAppIntersectionItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Keyword metrics for the returned keyword.
    pub keyword_data: Option<DataForSeoLabsKeywordData>,
    /// SERP element each analyzed target holds for the keyword, keyed by target.
    pub intersection_result: Option<HashMap<String, DataForSeoLabsAppSerpItem>>,
}

/// Aggregated ranking metrics of a single app.
/// The metric map is keyed by SERP feature type.
/// See <https://docs.dataforseo.com/v3/dataforseo_labs/google/bulk_app_metrics/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoLabsBulkAppMetricsItem {
    /// Search engine type (e.g. `"google"`).
    pub se_type: Option<String>,
    /// Store identifier of the app.
    pub app_id: Option<String>,
    /// Ranking and traffic metrics, grouped by SERP feature type.
    pub metrics: Option<HashMap<String, DataForSeoLabsAppMetricsInfo>>,
}
