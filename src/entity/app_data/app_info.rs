use crate::entity::{AppDataApiPrice, AppDataApiRating};
use serde::{Deserialize, Serialize};

/// Result item of an App Info `task_get/advanced` response.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppInfoResult {
    /// Store identifier of the application the info was requested for.
    pub app_id: Option<String>,
    /// Result type identifying the app info lookup.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// App store domain the results were fetched from (e.g. `play.google.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to.
    pub language_code: Option<String>,
    /// Direct URL to the application's store page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Types of result elements present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Total number of results the app store reported.
    pub se_results_count: Option<i64>,
    /// Number of elements returned in `items`.
    pub items_count: Option<i32>,
    /// Application info elements returned for the lookup.
    pub items: Option<Vec<AppDataApiAppInfoItem>>,
}

/// Detailed information about a single application.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_info/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppInfoItem {
    /// Element type (e.g. `google_play_info_organic`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Position among items of the same `type`.
    pub rank_group: Option<i32>,
    /// Absolute position among all returned items.
    pub rank_absolute: Option<i32>,
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
    /// Full application description.
    pub description: Option<String>,
    /// Total number of reviews the application has received.
    pub reviews_count: Option<i64>,
    /// Aggregated user rating of the application.
    pub rating: Option<AppDataApiRating>,
    /// `true` when the application is free to install.
    pub is_free: Option<bool>,
    /// Price of the application.
    pub price: Option<AppDataApiPrice>,
    /// Primary store category the application belongs to.
    pub main_category: Option<String>,
    /// Human-readable install count (e.g. `1,000,000+`).
    pub installs: Option<String>,
    /// Numeric install count.
    pub installs_count: Option<i64>,
    /// Name of the application's developer.
    pub developer: Option<String>,
    /// Store identifier of the developer.
    pub developer_id: Option<String>,
    /// URL of the developer's store profile.
    pub developer_url: Option<String>,
    /// Contact email of the developer.
    pub developer_email: Option<String>,
    /// Physical address of the developer.
    pub developer_address: Option<String>,
    /// Official website of the developer.
    pub developer_website: Option<String>,
    /// Current version of the application.
    pub version: Option<String>,
    /// Minimum OS version required to install the application.
    pub minimum_os_version: Option<String>,
    /// Download size of the application.
    pub size: Option<String>,
    /// Date the application was first released.
    pub released_date: Option<String>,
    /// Date of the most recent application update.
    pub last_update_date: Option<String>,
    /// Release notes of the latest update.
    pub update_notes: Option<String>,
    /// URLs of the application's screenshots.
    pub images: Option<Vec<String>>,
    /// URLs of the application's promotional videos.
    pub videos: Option<Vec<String>>,
    /// Genres assigned to the application.
    pub genres: Option<Vec<String>>,
    /// Tags associated with the application.
    pub tags: Option<Vec<String>>,
}
