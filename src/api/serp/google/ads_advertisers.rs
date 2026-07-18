use crate::entity::SerpApiGoogleAdsAdvertisersTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Advertisers SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/task_post/>
    pub async fn ads_advertisers_task_post(
        &self,
        data: Vec<SerpApiGoogleAdsAdvertisersTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/ads_advertisers/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/tasks_ready/>
    pub async fn ads_advertisers_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/ads_advertisers")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/task_get/advanced/>
    pub async fn ads_advertisers_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleAdsAdvertisersTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/ads_advertisers/task_get/advanced/{id}").as_str())
            .await
    }
}

/// Request body for Google Ads Advertisers task_post.
/// See <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAdsAdvertisersTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates the search was run for ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
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

impl SerpApiGoogleAdsAdvertisersTaskPostRequest {
    /// Creates a request for the given advertiser search term.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        SerpApiGoogleAdsAdvertisersTaskPostRequest {
            keyword: keyword.into(),
            ..SerpApiGoogleAdsAdvertisersTaskPostRequest::default()
        }
    }
}
