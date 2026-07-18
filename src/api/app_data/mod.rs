use crate::entity::{AppDataApiError, AppDataApiIdList};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

mod apple;
mod google;

pub use apple::*;
pub use google::*;

/// Entry point for the App Data API domain (Google Play and Apple App Store).
/// See <https://docs.dataforseo.com/v3/app_data/>.
pub struct AppDataApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the App Data API. See <https://docs.dataforseo.com/v3/app_data/>.
    pub fn app_data(&self) -> AppDataApi<'_> {
        AppDataApi { client: self }
    }
}

impl AppDataApi<'_> {
    /// Task IDs of the App Data tasks set within a time range.
    /// See <https://docs.dataforseo.com/v3/app_data/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<AppDataApiIdListPost>,
    ) -> DataForSeoApiResponse<AppDataApiIdList> {
        self.client.http_post("/v3/app_data/id_list", &data).await
    }

    /// App Data tasks that responded with an error within the last 7 days.
    /// See <https://docs.dataforseo.com/v3/app_data/errors/>.
    pub async fn errors(
        &self,
        data: Vec<AppDataApiErrorsPost>,
    ) -> DataForSeoApiResponse<AppDataApiError> {
        self.client.http_post("/v3/app_data/errors", &data).await
    }
}

/// Request body for the App Data `id_list` endpoint.
/// See <https://docs.dataforseo.com/v3/app_data/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiIdListPost {
    /// Start of the time range to list tasks for (UTC).
    pub datetime_from: String,
    /// End of the time range to list tasks for (UTC).
    pub datetime_to: String,
    /// Maximum number of task IDs to return (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort order by execution time (`asc` or `desc`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Include the original POST parameters of each task in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl AppDataApiIdListPost {
    /// Builds an `id_list` request for the `[datetime_from, datetime_to]` range.
    pub fn new(datetime_from: String, datetime_to: String) -> Self {
        AppDataApiIdListPost {
            datetime_from,
            datetime_to,
            ..AppDataApiIdListPost::default()
        }
    }
}

/// Request body for the App Data `errors` endpoint.
/// See <https://docs.dataforseo.com/v3/app_data/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiErrorsPost {
    /// Maximum number of errored tasks to return (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Restrict results to a specific API function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_function: Option<String>,
    /// Start of the time range to filter errors for (UTC, within the last 7 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_from: Option<String>,
    /// End of the time range to filter errors for (UTC, within the last 7 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_to: Option<String>,
}

/// Request body item for App Searches `task_post`.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_searches/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppSearchesTaskPostRequest {
    /// Search query to run against the app store.
    pub keyword: String,
    /// Full location name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// App store domain to search against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Number of results to return (max 200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Task execution priority (`1` normal, `2` high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier echoed back in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed result via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result format to post back (`advanced` or `html`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl AppDataApiAppSearchesTaskPostRequest {
    /// Builds an App Searches request for `keyword` in the given location/language.
    pub fn new(keyword: String, location_code: i32, language_code: String) -> Self {
        AppDataApiAppSearchesTaskPostRequest {
            keyword,
            location_code: Some(location_code),
            language_code: Some(language_code),
            ..AppDataApiAppSearchesTaskPostRequest::default()
        }
    }
}

/// Request body item for App List `task_post`.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_list/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppListTaskPostRequest {
    /// Store collection to list (e.g. `topselling_free`).
    pub app_collection: String,
    /// Store category to filter the collection by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_category: Option<String>,
    /// Age-rating bucket to filter apps by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_rating: Option<String>,
    /// Full location name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Number of apps to return (max 200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Task execution priority (`1` normal, `2` high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier echoed back in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed result via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result format to post back (`advanced` or `html`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl AppDataApiAppListTaskPostRequest {
    /// Builds an App List request for `app_collection` in the given location/language.
    pub fn new(app_collection: String, location_code: i32, language_code: String) -> Self {
        AppDataApiAppListTaskPostRequest {
            app_collection,
            location_code: Some(location_code),
            language_code: Some(language_code),
            ..AppDataApiAppListTaskPostRequest::default()
        }
    }
}

/// Request body item for App Info `task_post`.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppInfoTaskPostRequest {
    /// Store identifier of the application to fetch.
    pub app_id: String,
    /// Full location name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Task execution priority (`1` normal, `2` high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier echoed back in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed result via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result format to post back (`advanced` or `html`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl AppDataApiAppInfoTaskPostRequest {
    /// Builds an App Info request for `app_id` in the given location/language.
    pub fn new(app_id: String, location_code: i32, language_code: String) -> Self {
        AppDataApiAppInfoTaskPostRequest {
            app_id,
            location_code: Some(location_code),
            language_code: Some(language_code),
            ..AppDataApiAppInfoTaskPostRequest::default()
        }
    }
}

/// Request body item for App Reviews `task_post`.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_reviews/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppReviewsTaskPostRequest {
    /// Store identifier of the application to fetch reviews for.
    pub app_id: String,
    /// Full location name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code to localize results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Number of reviews to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Task execution priority (`1` normal, `2` high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier echoed back in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed result via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result format to post back (`advanced` or `html`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl AppDataApiAppReviewsTaskPostRequest {
    /// Builds an App Reviews request for `app_id` in the given location/language.
    pub fn new(app_id: String, location_code: i32, language_code: String) -> Self {
        AppDataApiAppReviewsTaskPostRequest {
            app_id,
            location_code: Some(location_code),
            language_code: Some(language_code),
            ..AppDataApiAppReviewsTaskPostRequest::default()
        }
    }
}
