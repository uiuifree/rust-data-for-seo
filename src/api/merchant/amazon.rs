use crate::api::merchant::MerchantApi;
use crate::entity::{
    MerchantApiAmazonAsin, MerchantApiAmazonProducts, MerchantApiAmazonSellers,
    MerchantApiLocation, MerchantApiTaskReady, SerpApiLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl MerchantApi<'_> {
    /// Access the Amazon Merchant API.
    /// See <https://docs.dataforseo.com/v3/merchant/amazon/overview/>.
    pub fn amazon(&self) -> MerchantApiAmazon<'_> {
        MerchantApiAmazon {
            client: self.client,
        }
    }
}

/// Amazon Merchant API (products, ASIN, sellers).
pub struct MerchantApiAmazon<'a> {
    client: &'a DataForSeoClient,
}

impl MerchantApiAmazon<'_> {
    /// <https://docs.dataforseo.com/v3/merchant/amazon/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<MerchantApiLocation> {
        self.client.http_get("/v3/merchant/amazon/locations").await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/locations/>
    pub async fn locations_country(
        &self,
        country: &str,
    ) -> DataForSeoApiResponse<MerchantApiLocation> {
        self.client
            .http_get(format!("/v3/merchant/amazon/locations/{country}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client.http_get("/v3/merchant/amazon/languages").await
    }

    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/task_post/>
    pub async fn products_task_post(
        &self,
        data: Vec<MerchantApiAmazonProductsPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/products/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/tasks_ready/>
    pub async fn products_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/amazon/products/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/task_get/advanced/>
    pub async fn products_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiAmazonProducts> {
        self.client
            .http_get(format!("/v3/merchant/amazon/products/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/task_get/html/>
    pub async fn products_task_get_html(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(format!("/v3/merchant/amazon/products/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/live/advanced/>
    pub async fn products_live_advanced(
        &self,
        data: Vec<MerchantApiAmazonProductsPost>,
    ) -> DataForSeoApiResponse<MerchantApiAmazonProducts> {
        self.client
            .http_post("/v3/merchant/amazon/products/live/advanced", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/products/live/html/>
    pub async fn products_live_html(
        &self,
        data: Vec<MerchantApiAmazonProductsPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/products/live/html", &data)
            .await
    }

    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/task_post/>
    pub async fn asin_task_post(
        &self,
        data: Vec<MerchantApiAmazonAsinPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/asin/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/tasks_ready/>
    pub async fn asin_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/amazon/asin/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/task_get/advanced/>
    pub async fn asin_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiAmazonAsin> {
        self.client
            .http_get(format!("/v3/merchant/amazon/asin/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/task_get/html/>
    pub async fn asin_task_get_html(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(format!("/v3/merchant/amazon/asin/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/live/advanced/>
    pub async fn asin_live_advanced(
        &self,
        data: Vec<MerchantApiAmazonAsinPost>,
    ) -> DataForSeoApiResponse<MerchantApiAmazonAsin> {
        self.client
            .http_post("/v3/merchant/amazon/asin/live/advanced", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/asin/live/html/>
    pub async fn asin_live_html(
        &self,
        data: Vec<MerchantApiAmazonAsinPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/asin/live/html", &data)
            .await
    }

    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/task_post/>
    pub async fn sellers_task_post(
        &self,
        data: Vec<MerchantApiAmazonSellersPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/sellers/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/tasks_ready/>
    pub async fn sellers_tasks_ready(&self) -> DataForSeoApiResponse<MerchantApiTaskReady> {
        self.client
            .http_get("/v3/merchant/amazon/sellers/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/task_get/advanced/>
    pub async fn sellers_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<MerchantApiAmazonSellers> {
        self.client
            .http_get(format!("/v3/merchant/amazon/sellers/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/task_get/html/>
    pub async fn sellers_task_get_html(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(format!("/v3/merchant/amazon/sellers/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/live/advanced/>
    pub async fn sellers_live_advanced(
        &self,
        data: Vec<MerchantApiAmazonSellersPost>,
    ) -> DataForSeoApiResponse<MerchantApiAmazonSellers> {
        self.client
            .http_post("/v3/merchant/amazon/sellers/live/advanced", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/merchant/amazon/sellers/live/html/>
    pub async fn sellers_live_html(
        &self,
        data: Vec<MerchantApiAmazonSellersPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/merchant/amazon/sellers/live/html", &data)
            .await
    }
}

/// Request payload for the Amazon products endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/amazon/products/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonProductsPost {
    /// Search keyword (up to 700 characters, URL-decoded).
    pub keyword: String,
    /// Direct Amazon search URL to parse instead of building one from `keyword`.
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
    /// Full language name (e.g. `"English (United States)"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en_US`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Amazon domain (e.g. `amazon.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Number of results to retrieve (default `100`, max `700`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Maximum number of result pages to crawl (max `7`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
    /// Amazon product category (department) to search within.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// Raw price/sorting URL parameters; ignored when `price_min`/`price_max`/`sort_by` are set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_param: Option<String>,
    /// Minimum product price filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_min: Option<i32>,
    /// Maximum product price filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_max: Option<i32>,
    /// Sort order (e.g. `relevance`, `price_low_to_high`, `avg_customer_review`).
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

impl MerchantApiAmazonProductsPost {
    /// Creates a request for the given search `keyword`.
    pub fn new(keyword: &str) -> MerchantApiAmazonProductsPost {
        MerchantApiAmazonProductsPost {
            keyword: keyword.to_string(),
            ..Default::default()
        }
    }
}

/// Request payload for the Amazon ASIN endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/amazon/asin/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonAsinPost {
    /// Product ASIN to look up.
    pub asin: String,
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
    /// Full language name (e.g. `"English (United States)"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en_US`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Amazon domain (e.g. `amazon.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
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

impl MerchantApiAmazonAsinPost {
    /// Creates a request for the given product `asin`.
    pub fn new(asin: &str) -> MerchantApiAmazonAsinPost {
        MerchantApiAmazonAsinPost {
            asin: asin.to_string(),
            ..Default::default()
        }
    }
}

/// Request payload for the Amazon sellers endpoints.
/// See <https://docs.dataforseo.com/v3/merchant/amazon/sellers/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiAmazonSellersPost {
    /// ASIN of the product to list sellers for.
    pub asin: String,
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
    /// Full language name (e.g. `"English (United States)"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code (e.g. `en_US`); alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom Amazon domain (e.g. `amazon.co.uk`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
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

impl MerchantApiAmazonSellersPost {
    /// Creates a request for the given product `asin`.
    pub fn new(asin: &str) -> MerchantApiAmazonSellersPost {
        MerchantApiAmazonSellersPost {
            asin: asin.to_string(),
            ..Default::default()
        }
    }
}
