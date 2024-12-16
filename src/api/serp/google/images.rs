use crate::entity::{SerpApiGoogleImagesAdvanced, SerpApiGoogleImagesHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Section Job
/// https://docs.dataforseo.com/v3/serp/google/images/overview/?bash
impl SerpApiGoogle<'_> {
    pub async fn images_task_post(
        &self,
        data: Vec<SerpApiGoogleImagesTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/google/images/task_post",
                &data,
            )
            .await
    }
    pub async fn images_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/images").await
    }
    pub async fn images_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/images").await
    }

    pub async fn images_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleImagesAdvanced> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/images/task_get/advanced/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn images_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleImagesHtml> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/images/task_get/html/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleImagesTaskPostRequest {
    pub keyword: String,
    pub url: Option<String>,
    pub priority: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_coordinate: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub os: Option<String>,
    pub se_domain: Option<String>,
    pub depth: Option<i32>,
    pub max_crawl_pages: Option<i32>,
    pub search_param: Option<String>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}
impl SerpApiGoogleImagesTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiGoogleImagesTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleImagesTaskPostRequest::default()
        };
        request
    }
}
