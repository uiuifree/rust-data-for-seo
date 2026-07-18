use crate::entity::{AppDataApiPrice, AppDataApiRating};
use serde::{Deserialize, Serialize};

/// Result item of an App Searches `task_get/advanced` response.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_searches/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppSearchesResult {
    /// Keyword the app search was run for.
    pub keyword: Option<String>,
    /// Result type identifying the app store search.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// App store domain the results were fetched from (e.g. `play.google.com`).
    pub se_domain: Option<String>,
    /// Location code the search was localized to.
    pub location_code: Option<i32>,
    /// Language code the search was localized to.
    pub language_code: Option<String>,
    /// Direct URL to the app store search results.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Types of result elements present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Total number of results the app store reported for the query.
    pub se_results_count: Option<i64>,
    /// Number of elements returned in `items`.
    pub items_count: Option<i64>,
    /// Applications found for the query.
    pub items: Option<Vec<AppDataApiAppSearchesItem>>,
}

/// A single application found by an App Searches task.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_searches/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppSearchesItem {
    /// Element type (e.g. `google_play_search_organic`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Position among items of the same `type`.
    pub rank_group: Option<i64>,
    /// Absolute position among all returned items.
    pub rank_absolute: Option<i64>,
    /// Alignment of the element within the results (e.g. `left`).
    pub position: Option<String>,
    /// Store identifier of the application.
    pub app_id: Option<String>,
    /// Application name.
    pub title: Option<String>,
    /// URL of the application's store page.
    pub url: Option<String>,
    /// URL of the application icon.
    pub icon: Option<String>,
    /// Short application description.
    pub description: Option<String>,
    /// Total number of reviews the application has received.
    pub reviews_count: Option<i64>,
    /// Aggregated user rating of the application.
    pub rating: Option<AppDataApiRating>,
    /// `true` when the application is free to install.
    pub is_free: Option<bool>,
    /// Price of the application.
    pub price: Option<AppDataApiPrice>,
    /// Name of the application's developer.
    pub developer: Option<String>,
    /// URL of the developer's store profile.
    pub developer_url: Option<String>,
}
