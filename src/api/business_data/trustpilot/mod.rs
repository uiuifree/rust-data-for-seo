//! Trustpilot endpoints of the Business Data API.
//! See <https://docs.dataforseo.com/v3/business_data/trustpilot/overview/>.

use crate::api::business_data::BusinessDataApi;
use crate::entity::{
    BusinessDataApiTaskReady, BusinessDataApiTrustpilotReviewsResult,
    BusinessDataApiTrustpilotReviewsTaskPost, BusinessDataApiTrustpilotSearchResult,
    BusinessDataApiTrustpilotSearchTaskPost,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde_json::Value;

impl BusinessDataApi<'_> {
    /// Returns the Trustpilot sub-API of the Business Data domain.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/overview/>.
    pub fn trustpilot(&self) -> BusinessDataApiTrustpilot<'_> {
        BusinessDataApiTrustpilot {
            client: self.client,
        }
    }
}

/// Trustpilot endpoints: Search and Reviews.
pub struct BusinessDataApiTrustpilot<'a> {
    client: &'a DataForSeoClient,
}

impl BusinessDataApiTrustpilot<'_> {
    /// Sets Trustpilot Search tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/search/task_post/>.
    pub async fn search_task_post(
        &self,
        data: Vec<BusinessDataApiTrustpilotSearchTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/trustpilot/search/task_post", &data)
            .await
    }

    /// Lists completed Trustpilot Search tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/search/tasks_ready/>.
    pub async fn search_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/trustpilot/search/tasks_ready")
            .await
    }

    /// Retrieves a completed Trustpilot Search task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/search/task_get/>.
    pub async fn search_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiTrustpilotSearchResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/trustpilot/search/task_get/{id}"
            ))
            .await
    }

    /// Sets Trustpilot Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/reviews/task_post/>.
    pub async fn reviews_task_post(
        &self,
        data: Vec<BusinessDataApiTrustpilotReviewsTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/trustpilot/reviews/task_post", &data)
            .await
    }

    /// Lists completed Trustpilot Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/reviews/tasks_ready/>.
    pub async fn reviews_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/trustpilot/reviews/tasks_ready")
            .await
    }

    /// Retrieves a completed Trustpilot Reviews task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/trustpilot/reviews/task_get/>.
    pub async fn reviews_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiTrustpilotReviewsResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/trustpilot/reviews/task_get/{id}"
            ))
            .await
    }
}
