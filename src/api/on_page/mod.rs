use crate::entity::{
    OnPageDataApiKeywordDensity, OnPageDataApiMicrodata, OnPageDataApiPage, OnPageDataApiRawHtml,
    OnPageDataApiWaterfall,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct OnPageApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    pub fn on_page(&self) -> OnPageApi {
        OnPageApi { client: self }
    }
}

impl OnPageApi<'_> {
    pub async fn task_post(&self, data: Vec<OnPageApiTaskPost>) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("https://api.dataforseo.com/v3/on_page/task_post", &data)
            .await
    }
    pub async fn summary(&self, id: &str) -> DataForSeoApiResponse<Value> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/on_page/summary/".to_string() + id).as_str(),
                &{},
            )
            .await
    }
    pub async fn pages(
        &self,
        data: Vec<OnPageApiPagesPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiPage> {
        self.client
            .http_post("https://api.dataforseo.com/v3/on_page/pages", &data)
            .await
    }
    pub async fn pages_by_resource(
        &self,
        data: Vec<OnPageApiPagesByResourcePost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/on_page/pages_by_resource",
                &data,
            )
            .await
    }
    pub async fn waterfall(
        &self,
        data: Vec<OnPageApiWaterfallPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiWaterfall> {
        self.client
            .http_post("https://api.dataforseo.com/v3/on_page/waterfall", &data)
            .await
    }
    pub async fn keyword_density(
        &self,
        data: Vec<OnPageApiKeywordDensityPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiKeywordDensity> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/on_page/keyword_density",
                &data,
            )
            .await
    }
    pub async fn microdata(
        &self,
        data: Vec<OnPageApiMicrodataPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiMicrodata> {
        self.client
            .http_post("https://api.dataforseo.com/v3/on_page/microdata", &data)
            .await
    }
    pub async fn raw_html(
        &self,
        data: Vec<OnPageApiRawHtmlPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiRawHtml> {
        self.client
            .http_post("https://api.dataforseo.com/v3/on_page/raw_html", &data)
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiWaterfallPost {
    pub id: String,
    pub url: String,
    pub tag: Option<String>,
}

impl OnPageApiWaterfallPost {
    pub fn new(id: String, url: String) -> OnPageApiWaterfallPost {
        OnPageApiWaterfallPost { id, url, tag: None }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiKeywordDensityPost {
    pub id: String,
    pub keyword_length: i32,
    pub url: Option<String>,
    pub filters: Option<Vec<Vec<String>>>,
    pub order_by: Option<Vec<String>>,
    pub tag: Option<String>,
}

impl OnPageApiKeywordDensityPost {
    pub fn new(id: String, keyword_length: i32) -> OnPageApiKeywordDensityPost {
        OnPageApiKeywordDensityPost {
            id,
            keyword_length,
            url: None,
            filters: None,
            order_by: None,
            tag: None,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiMicrodataPost {
    pub id: String,
    pub url: String,
    pub tag: Option<String>,
}

impl OnPageApiMicrodataPost {
    pub fn new(id: String, url: String) -> OnPageApiMicrodataPost {
        OnPageApiMicrodataPost { id, url, tag: None }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiTaskPost {
    pub target: String,
    pub max_crawl_pages: i32,
    pub start_url: Option<String>,
    pub force_sitewide_checks: Option<bool>,
    pub priority_urls: Option<Vec<String>>,
    pub max_crawl_depth: Option<i32>,
    pub crawl_delay: Option<i32>,
    pub store_raw_html: Option<bool>,
    pub enable_content_parsing: Option<bool>,
    pub support_cookies: Option<bool>,
    pub accept_language: Option<String>,
    pub custom_robots_txt: Option<String>,
    pub robots_txt_merge_mode: Option<String>,
    pub custom_user_agent: Option<String>,
    pub browser_preset: Option<String>,
    pub browser_screen_width: Option<i32>,
    pub browser_screen_height: Option<i32>,
    pub browser_screen_scale_factor: Option<f32>,
    pub respect_sitemap: Option<bool>,
    pub custom_sitemap: Option<String>,
    pub crawl_sitemap_only: Option<bool>,
    pub load_resources: Option<bool>,
    pub enable_www_redirect_check: Option<bool>,
    pub enable_javascript: Option<bool>,
    pub enable_xhr: Option<bool>,
    pub enable_browser_rendering: Option<bool>,
    pub disable_cookie_popup: Option<bool>,
    pub custom_js: Option<String>,
    pub validate_micromarkup: Option<bool>,
    pub allow_subdomains: Option<bool>,
    pub allowed_subdomains: Option<Vec<String>>,
    pub disallowed_subdomains: Option<Vec<String>>,
    pub check_spell: Option<String>,
    pub check_spell_language: Option<String>,
    pub check_spell_exceptions: Option<Vec<String>>,
    pub calculate_keyword_density: Option<bool>,
    // pub checks_threshold: Option<Value>,
    pub disable_sitewide_checks: Option<Vec<String>>,
    pub disable_page_checks: Option<Vec<String>>,
    pub switch_pool: Option<bool>,
    pub return_despite_timeout: Option<bool>,
    pub tag: Option<String>,
    pub pingback_url: Option<String>,
}
impl OnPageApiTaskPost {
    pub fn new(target: String, max_crawl_pages: i32) -> Self {
        let request = OnPageApiTaskPost {
            target,
            max_crawl_pages,
            ..OnPageApiTaskPost::default()
        };
        request
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiPagesPost {
    pub id: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub filters: Option<Vec<Vec<String>>>,
    pub order_by: Option<Vec<String>>,
    pub tag: Option<String>,
}

impl OnPageApiPagesPost {
    pub fn new(id: String) -> Self {
        OnPageApiPagesPost {
            id,
            ..OnPageApiPagesPost::default()
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiPagesByResourcePost {
    pub id: String,
    pub url: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub filters: Option<Vec<Vec<String>>>,
    pub order_by: Option<Vec<String>>,
    pub tag: Option<String>,
}

impl OnPageApiPagesByResourcePost {
    pub fn new(id: String, url: String) -> Self {
        OnPageApiPagesByResourcePost {
            id,
            url,
            ..OnPageApiPagesByResourcePost::default()
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiRawHtmlPost {
    pub id: String,
    pub url: String,
}

impl OnPageApiRawHtmlPost {
    pub fn new(id: String, url: String) -> Self {
        OnPageApiRawHtmlPost { id, url }
    }
}
