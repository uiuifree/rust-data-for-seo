//! Tripadvisor endpoints of the Business Data API.
//! See <https://docs.dataforseo.com/v3/business_data/tripadvisor/overview/>.

use crate::api::business_data::BusinessDataApi;
use crate::entity::{
    BusinessDataApiTaskReady, BusinessDataApiTripadvisorReviewsResult,
    BusinessDataApiTripadvisorReviewsTaskPost, BusinessDataApiTripadvisorSearchResult,
    BusinessDataApiTripadvisorSearchTaskPost, SerpApiLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation};
use serde_json::Value;

impl BusinessDataApi<'_> {
    /// Returns the Tripadvisor sub-API of the Business Data domain.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/overview/>.
    pub fn tripadvisor(&self) -> BusinessDataApiTripadvisor<'_> {
        BusinessDataApiTripadvisor {
            client: self.client,
        }
    }
}

/// Tripadvisor endpoints: Search and Reviews.
pub struct BusinessDataApiTripadvisor<'a> {
    client: &'a DataForSeoClient,
}

impl BusinessDataApiTripadvisor<'_> {
    /// Lists Tripadvisor locations supported by the Business Data API.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get("/v3/business_data/tripadvisor/locations")
            .await
    }

    /// Lists Tripadvisor languages supported by the Business Data API.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client
            .http_get("/v3/business_data/tripadvisor/languages")
            .await
    }

    /// Sets Tripadvisor Search tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/search/task_post/>.
    pub async fn search_task_post(
        &self,
        data: Vec<BusinessDataApiTripadvisorSearchTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/tripadvisor/search/task_post", &data)
            .await
    }

    /// Lists completed Tripadvisor Search tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/search/tasks_ready/>.
    pub async fn search_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/tripadvisor/search/tasks_ready")
            .await
    }

    /// Retrieves a completed Tripadvisor Search task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/search/task_get/>.
    pub async fn search_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiTripadvisorSearchResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/tripadvisor/search/task_get/{id}"
            ))
            .await
    }

    /// Sets Tripadvisor Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/reviews/task_post/>.
    pub async fn reviews_task_post(
        &self,
        data: Vec<BusinessDataApiTripadvisorReviewsTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/tripadvisor/reviews/task_post", &data)
            .await
    }

    /// Lists completed Tripadvisor Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/reviews/tasks_ready/>.
    pub async fn reviews_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/tripadvisor/reviews/tasks_ready")
            .await
    }

    /// Retrieves a completed Tripadvisor Reviews task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/reviews/task_get/>.
    pub async fn reviews_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiTripadvisorReviewsResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/tripadvisor/reviews/task_get/{id}"
            ))
            .await
    }
}
