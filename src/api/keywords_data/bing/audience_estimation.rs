use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::{
    KeywordsDataApiBingAudienceEstimation, KeywordsDataApiBingIndustry,
    KeywordsDataApiBingJobFunction,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Audience Estimation.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/live/>.
impl KeywordsDataApiBing<'_> {
    /// Job functions available for audience targeting.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/job_functions/>.
    pub async fn audience_estimation_job_functions(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingJobFunction> {
        self.client
            .http_get("/v3/keywords_data/bing/audience_estimation/job_functions")
            .await
    }
    /// Industries available for audience targeting.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/audience_estimation/industries/>.
    pub async fn audience_estimation_industries(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingIndustry> {
        self.client
            .http_get("/v3/keywords_data/bing/audience_estimation/industries")
            .await
    }
    /// Posts an Audience Estimation task for asynchronous processing.
    pub async fn audience_estimation_task_post(
        &self,
        data: Vec<KeywordsDataApiBingAudienceEstimationTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/audience_estimation/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Audience Estimation tasks ready to be collected.
    pub async fn audience_estimation_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/audience_estimation")
            .await
    }
    /// Collects the result of a previously posted Audience Estimation task.
    pub async fn audience_estimation_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingAudienceEstimation> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/audience_estimation/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Audience Estimation data synchronously.
    pub async fn audience_estimation_live(
        &self,
        data: Vec<KeywordsDataApiBingAudienceEstimationTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingAudienceEstimation> {
        self.client
            .http_post("/v3/keywords_data/bing/audience_estimation/live", &data)
            .await
    }
}

/// Request body for the Bing Audience Estimation endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingAudienceEstimationTaskPostRequest {
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as "latitude,longitude,radius(km)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Age brackets to target, e.g. "eighteen_to_twenty_four".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Vec<String>>,
    /// Genders to target: "male", "female" or "unknown".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Vec<String>>,
    /// Desired bid, in USD (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<f64>,
    /// Daily budget, in USD (max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_budget: Option<f64>,
    /// Industry identifiers to target (see `audience_estimation_industries`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<Vec<i64>>,
    /// Job function identifiers to target (see `audience_estimation_job_functions`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_function: Option<Vec<i64>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingAudienceEstimationTaskPostRequest {
    /// Creates an empty request; set a location before sending.
    pub fn new() -> Self {
        KeywordsDataApiBingAudienceEstimationTaskPostRequest::default()
    }
}
