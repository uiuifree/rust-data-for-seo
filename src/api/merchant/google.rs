use crate::api::merchant::MerchantApi;
use crate::entity::{
    MerchantApiGoogleProductInfo, MerchantApiGoogleProducts, MerchantApiGoogleReviews,
    MerchantApiGoogleSellers, MerchantApiLocation, MerchantApiTaskReady, SerpApiLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl MerchantApi<'_> {
    /// Access the Google Shopping Merchant API.
    /// See <https://docs.dataforseo.com/v3/merchant/google/overview/>.
    pub fn google(&self) -> MerchantApiGoogle<'_> {
        MerchantApiGoogle {
            client: self.client,
        }
    }
}

/// Google Shopping Merchant API (products, sellers, product_info, reviews).
pub struct MerchantApiGoogle<'a> {
    client: &'a DataForSeoClient,
}

impl MerchantApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/merchant/google/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<MerchantApiLocation> {
        self.client.http_get("/v3/merchant/google/locations").await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/locations/>
    pub async fn locations_country(
        &self,
        country: &str,
    ) -> DataForSeoApiResponse<MerchantApiLocation> {
        self.client
            .http_get(format!("/v3/merchant/google/locations/{country}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client.http_get("/v3/merchant/google/languages").await
    }

    /// <https://docs.dataforseo.com/v3/merchant/google/products/task_post/>
    pub async fn products_task_post(
        &self,
        data: Vec<MerchantApiGoogleProductsPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/google/products/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/products/tasks_ready/>
    pub async fn products_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/google/products/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/products/task_get/advanced/>
    pub async fn products_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiGoogleProducts> {
        self.client
            .http_get(format!("/v3/merchant/google/products/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/products/task_get/html/>
    pub async fn products_task_get_html(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(format!("/v3/merchant/google/products/task_get/html/{id}").as_str())
            .await
    }

    /// <https://docs.dataforseo.com/v3/merchant/google/sellers/task_post/>
    pub async fn sellers_task_post(
        &self,
        data: Vec<MerchantApiGoogleSellersPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/google/sellers/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/sellers/tasks_ready/>
    pub async fn sellers_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/google/sellers/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/sellers/task_get/advanced/>
    pub async fn sellers_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiGoogleSellers> {
        self.client
            .http_get(format!("/v3/merchant/google/sellers/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/sellers/ad_url/>
    pub async fn sellers_ad_url(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(format!("/v3/merchant/google/sellers/ad_url/{id}").as_str())
            .await
    }

    /// <https://docs.dataforseo.com/v3/merchant/google/product_info/task_post/>
    pub async fn product_info_task_post(
        &self,
        data: Vec<MerchantApiGoogleProductInfoPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/google/product_info/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/product_info/tasks_ready/>
    pub async fn product_info_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/google/product_info/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/product_info/task_get/advanced/>
    pub async fn product_info_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiGoogleProductInfo> {
        self.client
            .http_get(format!("/v3/merchant/google/product_info/task_get/advanced/{id}").as_str())
            .await
    }

    /// <https://docs.dataforseo.com/v3/merchant/google/reviews/task_post/>
    pub async fn reviews_task_post(
        &self,
        data: Vec<MerchantApiGoogleReviewsPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/google/reviews/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/reviews/tasks_ready/>
    pub async fn reviews_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/google/reviews/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/google/reviews/task_get/advanced/>
    pub async fn reviews_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiGoogleReviews> {
        self.client
            .http_get(format!("/v3/merchant/google/reviews/task_get/advanced/{id}").as_str())
            .await
    }
}

/// Request payload for the Google Shopping products endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/google/products/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductsPost {
    /// Search keyword (up to 700 characters, URL-decoded).
    pub keyword: String,
    /// Direct Google Shopping search URL to parse instead of building one from `keyword`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra charge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. `"London,England,United Kingdom"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name (e.g. `"English"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Google domain (e.g. `google.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Number of results to retrieve (default `40`, max `120`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Maximum number of result pages to crawl (max `7`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
    /// Raw price/sorting URL parameters; mutually exclusive with `price_min`/`price_max`/`sort_by`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_param: Option<String>,
    /// Minimum product price filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_min: Option<i32>,
    /// Maximum product price filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_max: Option<i32>,
    /// Sort order (`review_score`, `price_low_to_high`, or `price_high_to_low`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed task via POST; requires `postback_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result type delivered to `postback_url` (`advanced` or `html`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL pinged with a GET request when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl MerchantApiGoogleProductsPost {
    /// Creates a request for the given search `keyword`.
    pub fn new(keyword: &str) -> MerchantApiGoogleProductsPost {
        MerchantApiGoogleProductsPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request payload for the Google Shopping sellers endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/google/sellers/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleSellersPost {
    /// Google Shopping product identifier (one of `product_id`/`data_docid`/`gid`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Unique identifier of the SERP data element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_docid: Option<String>,
    /// Global product identifier on Google Shopping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// Product variant filter selecting a specific variation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pvf: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra charge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. `"London,England,United Kingdom"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name (e.g. `"English"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Google domain (e.g. `google.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Number of results to retrieve (default `10`, max `200`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Include "buy on Google" shops in the result (doubles the cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_shops_on_google: Option<bool>,
    /// Extra specification parameters carried over from the products endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_specifications: Option<Value>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed task via POST; requires `postback_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result type delivered to `postback_url` (`advanced`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL pinged with a GET request when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl MerchantApiGoogleSellersPost {
    /// Creates a request for the given `product_id`.
    pub fn new(product_id: &str) -> MerchantApiGoogleSellersPost {
        MerchantApiGoogleSellersPost {
            product_id: Some(product_id.to_string()),
            ..Default::default()
        }
    }
}

/// Request payload for the Google Shopping product_info endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/google/product_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleProductInfoPost {
    /// Google Shopping product identifier (one of `product_id`/`data_docid`/`gid`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Unique identifier of the SERP data element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_docid: Option<String>,
    /// Global product identifier on Google Shopping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra charge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. `"London,England,United Kingdom"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name (e.g. `"English"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Google domain (e.g. `google.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed task via POST; requires `postback_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result type delivered to `postback_url` (`advanced`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL pinged with a GET request when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl MerchantApiGoogleProductInfoPost {
    /// Creates a request for the given `product_id`.
    pub fn new(product_id: &str) -> MerchantApiGoogleProductInfoPost {
        MerchantApiGoogleProductInfoPost {
            product_id: Some(product_id.to_string()),
            ..Default::default()
        }
    }
}

/// Request payload for the Google Shopping reviews endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/google/reviews/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleReviewsPost {
    /// Global product identifier on Google Shopping (recommended primary identifier).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// Google Shopping product identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Unique identifier of the SERP data element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_docid: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra charge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. `"London,England,United Kingdom"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as `"latitude,longitude,radius"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name (e.g. `"English"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Google domain (e.g. `google.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Number of reviews to parse (default `10`, max `8000`; use multiples of 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to receive the completed task via POST; requires `postback_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result type delivered to `postback_url` (`advanced`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL pinged with a GET request when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl MerchantApiGoogleReviewsPost {
    /// Creates a request for the given `gid` (global product identifier).
    pub fn new(gid: &str) -> MerchantApiGoogleReviewsPost {
        MerchantApiGoogleReviewsPost {
            gid: Some(gid.to_string()),
            ..Default::default()
        }
    }
}
