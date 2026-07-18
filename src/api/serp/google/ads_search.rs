use crate::entity::SerpApiGoogleAdsSearchTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Search SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/ads_search/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/ads_search/task_post/>
    pub async fn ads_search_task_post(
        &self,
        data: Vec<SerpApiGoogleAdsSearchTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/ads_search/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ads_search/tasks_ready/>
    pub async fn ads_search_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/ads_search").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ads_search/task_get/advanced/>
    pub async fn ads_search_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleAdsSearchTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/ads_search/task_get/advanced/{id}").as_str())
            .await
    }
}

/// Request body for Google Ads Search task_post.
/// You must set either `advertiser_ids` or `target`.
/// See <https://docs.dataforseo.com/v3/serp/google/ads_search/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAdsSearchTaskPostRequest {
    /// Advertiser ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertiser_ids: Option<Vec<String>>,
    /// Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates the search was run for ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Ad creative format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Date from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Date to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleAdsSearchTaskPostRequest {
    /// Creates a request scoped to a set of advertiser ids (from Ads Advertisers).
    pub fn from_advertiser_ids(advertiser_ids: Vec<String>) -> Self {
        SerpApiGoogleAdsSearchTaskPostRequest {
            advertiser_ids: Some(advertiser_ids),
            ..SerpApiGoogleAdsSearchTaskPostRequest::default()
        }
    }
    /// Creates a request scoped to an advertiser's target domain.
    pub fn from_target<T: Into<String>>(target: T) -> Self {
        SerpApiGoogleAdsSearchTaskPostRequest {
            target: Some(target.into()),
            ..SerpApiGoogleAdsSearchTaskPostRequest::default()
        }
    }
}
