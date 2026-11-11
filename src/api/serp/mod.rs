use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

mod baidu;
mod bing;
mod common;
mod google;
mod naver;
mod seznam;
mod yahoo;
mod youtube;

pub use baidu::*;
pub use bing::*;
pub use common::*;
pub use google::*;
pub use naver::*;
pub use seznam::*;
pub use yahoo::*;
pub use youtube::*;

/// SERP data model.
pub struct SerpApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the SERP API domain entry point.
    /// See <https://docs.dataforseo.com/v3/serp/overview/>.
    pub fn serp(&self) -> SerpApi<'_> {
        SerpApi { client: self }
    }
}

impl SerpApi<'_> {
    /// Lists locations supported by the given search engine.
    pub async fn locations_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/{}/locations", se).as_str())
            .await
    }
    /// Lists locations within a country for the given search engine.
    pub async fn locations_country_se(
        &self,
        se: &str,
        country: &str,
    ) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/{}/locations/{}", se, country).as_str())
            .await
    }
    /// Lists completed tasks that are ready to be collected.
    pub async fn task_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.http_get("/v3/serp/tasks_ready").await
    }
    /// Lists completed tasks for the given search engine.
    pub async fn task_ready_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/serp/{}/tasks_ready", se).as_str())
            .await
    }
    /// Lists tasks that were force-fixed by DataForSEO.
    pub async fn task_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.http_get("/v3/serp/tasks_fixed").await
    }
    /// Lists force-fixed tasks for the given search engine.
    pub async fn task_fixed_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/serp/{}/tasks_fixed", se).as_str())
            .await
    }
    /// Captures a screenshot of the SERP of a completed task.
    /// See <https://docs.dataforseo.com/v3/serp/screenshot/>.
    pub async fn screenshot(
        &self,
        data: Vec<SerpApiScreenshotRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::SerpApiScreenshot> {
        self.client.http_post("/v3/serp/screenshot", &data).await
    }
    /// Generates an AI summary of the SERP of a completed task.
    /// See <https://docs.dataforseo.com/v3/serp/ai_summary/>.
    pub async fn ai_summary(
        &self,
        data: Vec<SerpApiAiSummaryRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::SerpApiAiSummary> {
        self.client.http_post("/v3/serp/ai_summary", &data).await
    }
    /// Lists the SERP tasks created within a time range.
    /// See <https://docs.dataforseo.com/v3/serp/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<crate::entity::DataForSeoIdListRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoIdListItem> {
        self.client.http_post("/v3/serp/id_list", &data).await
    }
    /// Lists the SERP tasks that returned an error in the last 7 days.
    /// See <https://docs.dataforseo.com/v3/serp/errors/>.
    pub async fn errors(
        &self,
        data: Vec<crate::entity::DataForSeoErrorsRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoErrorItem> {
        self.client.http_post("/v3/serp/errors", &data).await
    }
}

/// A completed task listed by the Tasks Ready endpoint.
/// See <https://docs.dataforseo.com/v3/serp/google/organic/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiTaskReadyResult {
    /// Identifier of the completed task, UUID format.
    pub id: Option<String>,
    /// Search engine the task was posted for (e.g. `google`).
    pub se: Option<String>,
    /// Search engine type (e.g. `organic`, `maps`).
    pub se_type: Option<String>,
    /// Date and time when the task was posted, UTC.
    pub date_posted: Option<String>,
    /// User-defined tag echoed from the task request.
    pub tag: Option<String>,
    /// URL for collecting the results in the regular format, if available.
    pub endpoint_regular: Option<String>,
    /// URL for collecting the results in the advanced format, if available.
    pub endpoint_advanced: Option<String>,
    /// URL for collecting the results in the HTML format, if available.
    pub endpoint_html: Option<String>,
}

/// Location SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiLocation {
    /// DataForSEO location code the search was run for.
    pub location_code: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    pub location_name: Option<String>,
    /// Location code of the parent region, if any.
    pub location_code_parent: Option<i32>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. City, Region, Country).
    pub location_type: Option<String>,
}
