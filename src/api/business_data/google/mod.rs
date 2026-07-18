//! Google endpoints of the Business Data API.
//! See <https://docs.dataforseo.com/v3/business_data/google/overview/>.

use crate::api::business_data::BusinessDataApi;
use crate::entity::{
    BusinessDataApiGoogleTaskPost, BusinessDataApiHotelInfoResult,
    BusinessDataApiHotelInfoTaskPost, BusinessDataApiHotelSearchesResult,
    BusinessDataApiHotelSearchesTaskPost, BusinessDataApiMyBusinessInfoResult,
    BusinessDataApiMyBusinessUpdatesResult, BusinessDataApiQuestionsAndAnswersResult,
    BusinessDataApiReviewsResult, BusinessDataApiReviewsTaskPost, BusinessDataApiTaskReady,
    SerpApiLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation};
use serde_json::Value;

impl BusinessDataApi<'_> {
    /// Returns the Google sub-API of the Business Data domain.
    /// See <https://docs.dataforseo.com/v3/business_data/google/overview/>.
    pub fn google(&self) -> BusinessDataApiGoogle<'_> {
        BusinessDataApiGoogle {
            client: self.client,
        }
    }
}

/// Google endpoints: My Business Info/Updates, Hotels, Reviews, Q&A.
pub struct BusinessDataApiGoogle<'a> {
    client: &'a DataForSeoClient,
}

impl BusinessDataApiGoogle<'_> {
    /// Lists Google locations supported by the Business Data API.
    /// See <https://docs.dataforseo.com/v3/business_data/google/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get("/v3/business_data/google/locations")
            .await
    }

    /// Lists Google languages supported by the Business Data API.
    /// See <https://docs.dataforseo.com/v3/business_data/google/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client
            .http_get("/v3/business_data/google/languages")
            .await
    }

    /// Sets My Business Info tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/task_post/>.
    pub async fn my_business_info_task_post(
        &self,
        data: Vec<BusinessDataApiGoogleTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/my_business_info/task_post", &data)
            .await
    }

    /// Lists completed My Business Info tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/tasks_ready/>.
    pub async fn my_business_info_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/my_business_info/tasks_ready")
            .await
    }

    /// Retrieves a completed My Business Info task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/task_get/>.
    pub async fn my_business_info_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiMyBusinessInfoResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/my_business_info/task_get/{id}"
            ))
            .await
    }

    /// Runs a My Business Info request synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/live/>.
    pub async fn my_business_info_live(
        &self,
        data: Vec<BusinessDataApiGoogleTaskPost>,
    ) -> DataForSeoApiResponse<BusinessDataApiMyBusinessInfoResult> {
        self.client
            .http_post("/v3/business_data/google/my_business_info/live", &data)
            .await
    }

    /// Sets My Business Updates tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_updates/task_post/>.
    pub async fn my_business_updates_task_post(
        &self,
        data: Vec<BusinessDataApiGoogleTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/business_data/google/my_business_updates/task_post",
                &data,
            )
            .await
    }

    /// Lists completed My Business Updates tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_updates/tasks_ready/>.
    pub async fn my_business_updates_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/my_business_updates/tasks_ready")
            .await
    }

    /// Retrieves a completed My Business Updates task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/my_business_updates/task_get/>.
    pub async fn my_business_updates_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiMyBusinessUpdatesResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/my_business_updates/task_get/{id}"
            ))
            .await
    }

    /// Sets Hotel Searches tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/task_post/>.
    pub async fn hotel_searches_task_post(
        &self,
        data: Vec<BusinessDataApiHotelSearchesTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/hotel_searches/task_post", &data)
            .await
    }

    /// Lists completed Hotel Searches tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/tasks_ready/>.
    pub async fn hotel_searches_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/hotel_searches/tasks_ready")
            .await
    }

    /// Retrieves a completed Hotel Searches task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/task_get/>.
    pub async fn hotel_searches_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiHotelSearchesResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/hotel_searches/task_get/{id}"
            ))
            .await
    }

    /// Runs a Hotel Searches request synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/live/>.
    pub async fn hotel_searches_live(
        &self,
        data: Vec<BusinessDataApiHotelSearchesTaskPost>,
    ) -> DataForSeoApiResponse<BusinessDataApiHotelSearchesResult> {
        self.client
            .http_post("/v3/business_data/google/hotel_searches/live", &data)
            .await
    }

    /// Sets Hotel Info tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/task_post/>.
    pub async fn hotel_info_task_post(
        &self,
        data: Vec<BusinessDataApiHotelInfoTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/hotel_info/task_post", &data)
            .await
    }

    /// Lists completed Hotel Info tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/tasks_ready/>.
    pub async fn hotel_info_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/hotel_info/tasks_ready")
            .await
    }

    /// Retrieves a completed Hotel Info task (advanced) by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/task_get/advanced/>.
    pub async fn hotel_info_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiHotelInfoResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/hotel_info/task_get/advanced/{id}"
            ))
            .await
    }

    /// Retrieves a completed Hotel Info task (HTML) by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/task_get/html/>.
    pub async fn hotel_info_task_get_html(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/hotel_info/task_get/html/{id}"
            ))
            .await
    }

    /// Runs a Hotel Info request synchronously (advanced).
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/live/advanced/>.
    pub async fn hotel_info_live_advanced(
        &self,
        data: Vec<BusinessDataApiHotelInfoTaskPost>,
    ) -> DataForSeoApiResponse<BusinessDataApiHotelInfoResult> {
        self.client
            .http_post("/v3/business_data/google/hotel_info/live/advanced", &data)
            .await
    }

    /// Runs a Hotel Info request synchronously (HTML).
    /// See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/live/html/>.
    pub async fn hotel_info_live_html(
        &self,
        data: Vec<BusinessDataApiHotelInfoTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/hotel_info/live/html", &data)
            .await
    }

    /// Sets Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/reviews/task_post/>.
    pub async fn reviews_task_post(
        &self,
        data: Vec<BusinessDataApiReviewsTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/reviews/task_post", &data)
            .await
    }

    /// Lists completed Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/reviews/tasks_ready/>.
    pub async fn reviews_tasks_ready(&self) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/reviews/tasks_ready")
            .await
    }

    /// Retrieves a completed Reviews task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/reviews/task_get/>.
    pub async fn reviews_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiReviewsResult> {
        self.client
            .http_get(&format!("/v3/business_data/google/reviews/task_get/{id}"))
            .await
    }

    /// Sets Extended Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/extended_reviews/task_post/>.
    pub async fn extended_reviews_task_post(
        &self,
        data: Vec<BusinessDataApiReviewsTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/business_data/google/extended_reviews/task_post", &data)
            .await
    }

    /// Lists completed Extended Reviews tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/extended_reviews/tasks_ready/>.
    pub async fn extended_reviews_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/extended_reviews/tasks_ready")
            .await
    }

    /// Retrieves a completed Extended Reviews task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/extended_reviews/task_get/>.
    pub async fn extended_reviews_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiReviewsResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/extended_reviews/task_get/{id}"
            ))
            .await
    }

    /// Sets Questions and Answers tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/questions_and_answers/task_post/>.
    pub async fn questions_and_answers_task_post(
        &self,
        data: Vec<BusinessDataApiGoogleTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/business_data/google/questions_and_answers/task_post",
                &data,
            )
            .await
    }

    /// Lists completed Questions and Answers tasks.
    /// See <https://docs.dataforseo.com/v3/business_data/google/questions_and_answers/tasks_ready/>.
    pub async fn questions_and_answers_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<BusinessDataApiTaskReady> {
        self.client
            .http_get("/v3/business_data/google/questions_and_answers/tasks_ready")
            .await
    }

    /// Retrieves a completed Questions and Answers task by id.
    /// See <https://docs.dataforseo.com/v3/business_data/google/questions_and_answers/task_get/>.
    pub async fn questions_and_answers_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<BusinessDataApiQuestionsAndAnswersResult> {
        self.client
            .http_get(&format!(
                "/v3/business_data/google/questions_and_answers/task_get/{id}"
            ))
            .await
    }

    /// Runs a Questions and Answers request synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/google/questions_and_answers/live/>.
    pub async fn questions_and_answers_live(
        &self,
        data: Vec<BusinessDataApiGoogleTaskPost>,
    ) -> DataForSeoApiResponse<BusinessDataApiQuestionsAndAnswersResult> {
        self.client
            .http_post("/v3/business_data/google/questions_and_answers/live", &data)
            .await
    }
}
