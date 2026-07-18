use crate::api::app_data::{
    AppDataApi, AppDataApiAppInfoTaskPostRequest, AppDataApiAppListTaskPostRequest,
    AppDataApiAppReviewsTaskPostRequest, AppDataApiAppSearchesTaskPostRequest,
};
use crate::entity::{
    AppDataApiAppInfoResult, AppDataApiAppListResult, AppDataApiAppReviewsResult,
    AppDataApiAppSearchesResult, AppDataApiCategory, AppDataApiLanguage, AppDataApiLocation,
    AppDataApiTaskReadyResult,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde_json::Value;

impl AppDataApi<'_> {
    /// Apple App Store sub-API. See <https://docs.dataforseo.com/v3/app_data/apple/>.
    pub fn apple(&self) -> AppDataApiApple<'_> {
        AppDataApiApple {
            client: self.client,
        }
    }
}

/// Apple App Store endpoints of the App Data API.
/// See <https://docs.dataforseo.com/v3/app_data/apple/>.
pub struct AppDataApiApple<'a> {
    client: &'a DataForSeoClient,
}

impl AppDataApiApple<'_> {
    /// Locations supported by Apple App Store App Data.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<AppDataApiLocation> {
        self.client.http_get("/v3/app_data/apple/locations").await
    }

    /// Languages supported by Apple App Store App Data.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<AppDataApiLanguage> {
        self.client.http_get("/v3/app_data/apple/languages").await
    }

    /// Categories supported by Apple App Store App Data.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/categories/>.
    pub async fn categories(&self) -> DataForSeoApiResponse<AppDataApiCategory> {
        self.client.http_get("/v3/app_data/apple/categories").await
    }

    /// Sets App Searches tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_searches/task_post/>.
    pub async fn app_searches_task_post(
        &self,
        data: Vec<AppDataApiAppSearchesTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/app_data/apple/app_searches/task_post", &data)
            .await
    }

    /// Lists completed App Searches tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_searches/tasks_ready/>.
    pub async fn app_searches_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<AppDataApiTaskReadyResult> {
        self.client
            .http_get("/v3/app_data/apple/app_searches/tasks_ready")
            .await
    }

    /// Retrieves the advanced result of an App Searches task.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_searches/task_get/advanced/>.
    pub async fn app_searches_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AppDataApiAppSearchesResult> {
        self.client
            .http_get(&format!(
                "/v3/app_data/apple/app_searches/task_get/advanced/{id}"
            ))
            .await
    }

    /// Sets App List tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_list/task_post/>.
    pub async fn app_list_task_post(
        &self,
        data: Vec<AppDataApiAppListTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/app_data/apple/app_list/task_post", &data)
            .await
    }

    /// Lists completed App List tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_list/tasks_ready/>.
    pub async fn app_list_tasks_ready(&self) -> DataForSeoApiResponse<AppDataApiTaskReadyResult> {
        self.client
            .http_get("/v3/app_data/apple/app_list/tasks_ready")
            .await
    }

    /// Retrieves the advanced result of an App List task.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_list/task_get/advanced/>.
    pub async fn app_list_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AppDataApiAppListResult> {
        self.client
            .http_get(&format!(
                "/v3/app_data/apple/app_list/task_get/advanced/{id}"
            ))
            .await
    }

    /// Sets App Info tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_info/task_post/>.
    pub async fn app_info_task_post(
        &self,
        data: Vec<AppDataApiAppInfoTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/app_data/apple/app_info/task_post", &data)
            .await
    }

    /// Lists completed App Info tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_info/tasks_ready/>.
    pub async fn app_info_tasks_ready(&self) -> DataForSeoApiResponse<AppDataApiTaskReadyResult> {
        self.client
            .http_get("/v3/app_data/apple/app_info/tasks_ready")
            .await
    }

    /// Retrieves the advanced result of an App Info task.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_info/task_get/advanced/>.
    pub async fn app_info_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AppDataApiAppInfoResult> {
        self.client
            .http_get(&format!(
                "/v3/app_data/apple/app_info/task_get/advanced/{id}"
            ))
            .await
    }

    /// Sets App Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_reviews/task_post/>.
    pub async fn app_reviews_task_post(
        &self,
        data: Vec<AppDataApiAppReviewsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/app_data/apple/app_reviews/task_post", &data)
            .await
    }

    /// Lists completed App Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_reviews/tasks_ready/>.
    pub async fn app_reviews_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<AppDataApiTaskReadyResult> {
        self.client
            .http_get("/v3/app_data/apple/app_reviews/tasks_ready")
            .await
    }

    /// Retrieves the advanced result of an App Reviews task.
    /// See <https://docs.dataforseo.com/v3/app_data/apple/app_reviews/task_get/advanced/>.
    pub async fn app_reviews_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<AppDataApiAppReviewsResult> {
        self.client
            .http_get(&format!(
                "/v3/app_data/apple/app_reviews/task_get/advanced/{id}"
            ))
            .await
    }
}
