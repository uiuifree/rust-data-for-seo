use crate::entity::{
    OnPageDataApiContentParsing, OnPageDataApiDuplicateContent, OnPageDataApiDuplicateTags,
    OnPageDataApiError, OnPageDataApiIdList, OnPageDataApiInstantPages,
    OnPageDataApiKeywordDensity, OnPageDataApiLighthouseTaskGet, OnPageDataApiLighthouseTasksReady,
    OnPageDataApiLinks, OnPageDataApiMicrodata, OnPageDataApiNonIndexable, OnPageDataApiPage,
    OnPageDataApiPageScreensShot, OnPageDataApiRawHtml, OnPageDataApiRedirectChains,
    OnPageDataApiResources, OnPageDataApiSummary, OnPageDataApiTasksReady, OnPageDataApiWaterfall,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OnPage API — on-page SEO audit and website crawling.
/// See <https://docs.dataforseo.com/v3/on_page/overview/>.
pub struct OnPageApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the OnPage API endpoint group.
    pub fn on_page(&self) -> OnPageApi<'_> {
        OnPageApi { client: self }
    }
}

impl OnPageApi<'_> {
    /// Sets a crawling task for the target website.
    /// See <https://docs.dataforseo.com/v3/on_page/task_post/>.
    pub async fn task_post(&self, data: Vec<OnPageApiTaskPost>) -> DataForSeoApiResponse<Value> {
        self.client.http_post("/v3/on_page/task_post", &data).await
    }

    /// Returns the list of completed crawling tasks.
    /// See <https://docs.dataforseo.com/v3/on_page/tasks_ready/>.
    pub async fn tasks_ready(&self) -> DataForSeoApiResponse<OnPageDataApiTasksReady> {
        self.client.http_get("/v3/on_page/tasks_ready").await
    }

    /// Stops an active crawling session.
    /// See <https://docs.dataforseo.com/v3/on_page/force_stop/>.
    pub async fn force_stop(
        &self,
        data: Vec<OnPageApiForceStopPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client.http_post("/v3/on_page/force_stop", &data).await
    }

    /// Returns a summary of the crawling results for a task.
    /// See <https://docs.dataforseo.com/v3/on_page/summary/>.
    pub async fn summary(&self, id: &str) -> DataForSeoApiResponse<OnPageDataApiSummary> {
        self.client
            .http_get(format!("/v3/on_page/summary/{id}").as_str())
            .await
    }

    /// Returns the crawled pages of a task.
    /// See <https://docs.dataforseo.com/v3/on_page/pages/>.
    pub async fn pages(
        &self,
        data: Vec<OnPageApiPagesPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiPage> {
        self.client.http_post("/v3/on_page/pages", &data).await
    }

    /// Returns the pages on which a given resource appears.
    /// See <https://docs.dataforseo.com/v3/on_page/pages_by_resource/>.
    pub async fn pages_by_resource(
        &self,
        data: Vec<OnPageApiPagesByResourcePost>,
    ) -> DataForSeoApiResponse<OnPageDataApiPage> {
        self.client
            .http_post("/v3/on_page/pages_by_resource", &data)
            .await
    }

    /// Returns the resources (images, scripts, stylesheets, ...) found during crawling.
    /// See <https://docs.dataforseo.com/v3/on_page/resources/>.
    pub async fn resources(
        &self,
        data: Vec<OnPageApiResourcesPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiResources> {
        self.client.http_post("/v3/on_page/resources", &data).await
    }

    /// Returns pages that share duplicate title or description tags.
    /// See <https://docs.dataforseo.com/v3/on_page/duplicate_tags/>.
    pub async fn duplicate_tags(
        &self,
        data: Vec<OnPageApiDuplicateTagsPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiDuplicateTags> {
        self.client
            .http_post("/v3/on_page/duplicate_tags", &data)
            .await
    }

    /// Returns pages with content duplicating a given page.
    /// See <https://docs.dataforseo.com/v3/on_page/duplicate_content/>.
    pub async fn duplicate_content(
        &self,
        data: Vec<OnPageApiDuplicateContentPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiDuplicateContent> {
        self.client
            .http_post("/v3/on_page/duplicate_content", &data)
            .await
    }

    /// Returns the internal and external links discovered during crawling.
    /// See <https://docs.dataforseo.com/v3/on_page/links/>.
    pub async fn links(
        &self,
        data: Vec<OnPageApiLinksPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiLinks> {
        self.client.http_post("/v3/on_page/links", &data).await
    }

    /// Returns redirect chains and loops found during crawling.
    /// See <https://docs.dataforseo.com/v3/on_page/redirect_chains/>.
    pub async fn redirect_chains(
        &self,
        data: Vec<OnPageApiRedirectChainsPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiRedirectChains> {
        self.client
            .http_post("/v3/on_page/redirect_chains", &data)
            .await
    }

    /// Returns pages that are not available for indexing.
    /// See <https://docs.dataforseo.com/v3/on_page/non_indexable/>.
    pub async fn non_indexable(
        &self,
        data: Vec<OnPageApiNonIndexablePost>,
    ) -> DataForSeoApiResponse<OnPageDataApiNonIndexable> {
        self.client
            .http_post("/v3/on_page/non_indexable", &data)
            .await
    }

    /// Returns the loading sequence of resources for a page.
    /// See <https://docs.dataforseo.com/v3/on_page/waterfall/>.
    pub async fn waterfall(
        &self,
        data: Vec<OnPageApiWaterfallPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiWaterfall> {
        self.client.http_post("/v3/on_page/waterfall", &data).await
    }

    /// Returns keyword density metrics for the crawled content.
    /// See <https://docs.dataforseo.com/v3/on_page/keyword_density/>.
    pub async fn keyword_density(
        &self,
        data: Vec<OnPageApiKeywordDensityPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiKeywordDensity> {
        self.client
            .http_post("/v3/on_page/keyword_density", &data)
            .await
    }

    /// Validates the structured markup (microdata) of a page.
    /// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
    pub async fn microdata(
        &self,
        data: Vec<OnPageApiMicrodataPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiMicrodata> {
        self.client.http_post("/v3/on_page/microdata", &data).await
    }

    /// Returns the raw HTML stored for a crawled page.
    /// See <https://docs.dataforseo.com/v3/on_page/raw_html/>.
    pub async fn raw_html(
        &self,
        data: Vec<OnPageApiRawHtmlPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiRawHtml> {
        self.client.http_post("/v3/on_page/raw_html", &data).await
    }

    /// Captures a screenshot of a page.
    /// See <https://docs.dataforseo.com/v3/on_page/page_screenshot/>.
    pub async fn page_screenshot(
        &self,
        data: Vec<OnPageApiPageScreenshotPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiPageScreensShot> {
        self.client
            .http_post("/v3/on_page/page_screenshot", &data)
            .await
    }

    /// Parses the structured content of a crawled page.
    /// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
    pub async fn content_parsing(
        &self,
        data: Vec<OnPageApiContentParsingPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiContentParsing> {
        self.client
            .http_post("/v3/on_page/content_parsing", &data)
            .await
    }

    /// Parses the structured content of a page in real time.
    /// See <https://docs.dataforseo.com/v3/on_page/content_parsing/live/>.
    pub async fn content_parsing_live(
        &self,
        data: Vec<OnPageApiContentParsingLivePost>,
    ) -> DataForSeoApiResponse<OnPageDataApiContentParsing> {
        self.client
            .http_post("/v3/on_page/content_parsing/live", &data)
            .await
    }

    /// Crawls and returns on-page data for a single page in real time.
    /// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
    pub async fn instant_pages(
        &self,
        data: Vec<OnPageApiInstantPagesPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiInstantPages> {
        self.client
            .http_post("/v3/on_page/instant_pages", &data)
            .await
    }

    /// Returns the list of task IDs created within a datetime range.
    /// See <https://docs.dataforseo.com/v3/on_page/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<OnPageApiIdListPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiIdList> {
        self.client.http_post("/v3/on_page/id_list", &data).await
    }

    /// Returns OnPage API errors from the past 7 days (free of charge).
    /// See <https://docs.dataforseo.com/v3/on_page/errors/>.
    pub async fn errors(
        &self,
        data: Vec<OnPageApiErrorsPost>,
    ) -> DataForSeoApiResponse<OnPageDataApiError> {
        self.client.http_post("/v3/on_page/errors", &data).await
    }

    /// Returns the full list of filters available for OnPage endpoints (free of charge).
    /// See <https://docs.dataforseo.com/v3/on_page/filters_and_thresholds/>.
    pub async fn available_filters(&self) -> DataForSeoApiResponse<Value> {
        self.client.http_get("/v3/on_page/available_filters").await
    }

    /// Sets a Lighthouse audit task for a page.
    /// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_post/>.
    pub async fn lighthouse_task_post(
        &self,
        data: Vec<OnPageApiLighthouseTaskPost>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/on_page/lighthouse/task_post", &data)
            .await
    }

    /// Returns the list of completed Lighthouse audit tasks.
    /// See <https://docs.dataforseo.com/v3/on_page/lighthouse/tasks_ready/>.
    pub async fn lighthouse_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<OnPageDataApiLighthouseTasksReady> {
        self.client
            .http_get("/v3/on_page/lighthouse/tasks_ready")
            .await
    }

    /// Returns the result of a completed Lighthouse audit task.
    /// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_get/>.
    pub async fn lighthouse_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<OnPageDataApiLighthouseTaskGet> {
        self.client
            .http_get(format!("/v3/on_page/lighthouse/task_get/json/{id}").as_str())
            .await
    }

    /// Runs a Lighthouse audit for a page in real time.
    /// See <https://docs.dataforseo.com/v3/on_page/lighthouse/live/>.
    pub async fn lighthouse_live(
        &self,
        data: Vec<OnPageApiLighthouseLivePost>,
    ) -> DataForSeoApiResponse<OnPageDataApiLighthouseTaskGet> {
        self.client
            .http_post("/v3/on_page/lighthouse/live/json", &data)
            .await
    }
}

/// Request for [`OnPageApi::task_post`].
/// See <https://docs.dataforseo.com/v3/on_page/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiTaskPost {
    /// Domain to crawl, without protocol or `www` (e.g. "example.com").
    pub target: String,
    /// Maximum number of pages to crawl.
    pub max_crawl_pages: i32,
    /// First URL to crawl; defaults to the domain's home page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_url: Option<String>,
    /// Run sitewide checks even when only a single page is crawled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_sitewide_checks: Option<bool>,
    /// URLs to crawl before the rest of the site (max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_urls: Option<Vec<String>>,
    /// Maximum crawl depth, counted in clicks from the start page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_depth: Option<i32>,
    /// Delay between page requests, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_delay: Option<i32>,
    /// Store the raw HTML of crawled pages for later retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_raw_html: Option<bool>,
    /// Parse page content so it can be retrieved via the content endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_content_parsing: Option<bool>,
    /// Accept cookies while crawling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_cookies: Option<bool>,
    /// Value of the Accept-Language header sent while crawling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// Custom robots.txt content to apply during the crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_robots_txt: Option<String>,
    /// How custom robots.txt combines with the site's: "merge" or "override".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots_txt_merge_mode: Option<String>,
    /// Custom User-Agent header sent while crawling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
    /// Device preset for rendering: "desktop", "mobile", or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_preset: Option<String>,
    /// Viewport width used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Viewport height used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Device pixel ratio used for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f32>,
    /// Crawl following the site's XML sitemap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respect_sitemap: Option<bool>,
    /// URL of a custom sitemap to crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sitemap: Option<String>,
    /// Restrict the crawl to URLs listed in the sitemap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_sitemap_only: Option<bool>,
    /// Load page resources (images, scripts, stylesheets) while crawling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_resources: Option<bool>,
    /// Check whether the www and non-www versions redirect consistently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_www_redirect_check: Option<bool>,
    /// Execute page JavaScript while crawling (incurs extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_javascript: Option<bool>,
    /// Allow XMLHttpRequest calls while rendering the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_xhr: Option<bool>,
    /// Emulate full browser rendering to measure Core Web Vitals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_browser_rendering: Option<bool>,
    /// Hide cookie-consent popups before capturing the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cookie_popup: Option<bool>,
    /// Custom JavaScript to execute on each crawled page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_js: Option<String>,
    /// Validate structured microdata markup on crawled pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_micromarkup: Option<bool>,
    /// Crawl subdomains of the target domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_subdomains: Option<bool>,
    /// Subdomains to crawl (used when subdomains are otherwise excluded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_subdomains: Option<Vec<String>>,
    /// Subdomains to exclude from the crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_subdomains: Option<Vec<String>>,
    /// Check spelling on crawled pages using the Hunspell library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_spell: Option<bool>,
    /// Hunspell language code used for spell checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_spell_language: Option<String>,
    /// Words to ignore during spell checking (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_spell_exceptions: Option<Vec<String>>,
    /// Compute keyword density for crawled pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_keyword_density: Option<bool>,
    /// Custom threshold values for the checks in OnPage responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks_threshold: Option<Value>,
    /// Sitewide checks to skip during the crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sitewide_checks: Option<Vec<String>>,
    /// Per-page checks to skip during the crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_page_checks: Option<Vec<String>>,
    /// Retry using a different proxy pool if the first attempt fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pool: Option<bool>,
    /// Return partial results even if the crawl times out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_despite_timeout: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL notified by GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl OnPageApiTaskPost {
    /// Creates a task request for `target`, crawling up to `max_crawl_pages` pages.
    pub fn new(target: String, max_crawl_pages: i32) -> Self {
        OnPageApiTaskPost {
            target,
            max_crawl_pages,
            ..OnPageApiTaskPost::default()
        }
    }
}

/// Request for [`OnPageApi::pages`].
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiPagesPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Maximum number of pages to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Vec<String>>>,
    /// Sorting rules, each as "field,asc" or "field,desc" (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Pagination token returned by a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_after_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiPagesPost {
    /// Creates a pages request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiPagesPost {
            id,
            ..OnPageApiPagesPost::default()
        }
    }
}

/// Request for [`OnPageApi::pages_by_resource`].
/// See <https://docs.dataforseo.com/v3/on_page/pages_by_resource/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiPagesByResourcePost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Resource URL, obtained from the Resources endpoint.
    pub url: String,
    /// Maximum number of pages to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Vec<String>>>,
    /// Sorting rules, each as "field,asc" or "field,desc" (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiPagesByResourcePost {
    /// Creates a request for pages on which the resource `url` appears, for task `id`.
    pub fn new(id: String, url: String) -> Self {
        OnPageApiPagesByResourcePost {
            id,
            url,
            ..OnPageApiPagesByResourcePost::default()
        }
    }
}

/// Request for [`OnPageApi::resources`].
/// See <https://docs.dataforseo.com/v3/on_page/resources/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiResourcesPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Restrict results to resources found on this page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Maximum number of resources to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules applied to resources (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Filtering rules applied to the pages that reference each resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_pages_filters: Option<Vec<Value>>,
    /// Sorting rules, each as "field,asc" or "field,desc" (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// Pagination token returned by a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_after_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiResourcesPost {
    /// Creates a resources request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiResourcesPost {
            id,
            ..OnPageApiResourcesPost::default()
        }
    }
}

/// Request for [`OnPageApi::duplicate_tags`].
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_tags/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiDuplicateTagsPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// `duplicate_title` or `duplicate_description`.
    #[serde(rename = "type")]
    pub duplicate_type: String,
    /// Restrict results to a specific title or description value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulator: Option<String>,
    /// Maximum number of pages to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiDuplicateTagsPost {
    /// Creates a duplicate-tags request for task `id` and the given `duplicate_type`.
    pub fn new(id: String, duplicate_type: String) -> Self {
        OnPageApiDuplicateTagsPost {
            id,
            duplicate_type,
            ..OnPageApiDuplicateTagsPost::default()
        }
    }
}

/// Request for [`OnPageApi::duplicate_content`].
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_content/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiDuplicateContentPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Page URL to compare other pages against.
    pub url: String,
    /// Minimum content similarity to include, from 0 to 10 (default 6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<i32>,
    /// Maximum number of pages to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiDuplicateContentPost {
    /// Creates a duplicate-content request for task `id`, comparing against `url`.
    pub fn new(id: String, url: String) -> Self {
        OnPageApiDuplicateContentPost {
            id,
            url,
            ..OnPageApiDuplicateContentPost::default()
        }
    }
}

/// Request for [`OnPageApi::links`].
/// See <https://docs.dataforseo.com/v3/on_page/links/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiLinksPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Restrict results to links originating from this page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_from: Option<String>,
    /// Restrict results to links pointing to this page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_to: Option<String>,
    /// Maximum number of links to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// Pagination token returned by a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_after_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiLinksPost {
    /// Creates a links request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiLinksPost {
            id,
            ..OnPageApiLinksPost::default()
        }
    }
}

/// Request for [`OnPageApi::redirect_chains`].
/// See <https://docs.dataforseo.com/v3/on_page/redirect_chains/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiRedirectChainsPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Restrict results to redirect chains starting at this URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Maximum number of chains to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules; only `is_redirect_loop` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiRedirectChainsPost {
    /// Creates a redirect-chains request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiRedirectChainsPost {
            id,
            ..OnPageApiRedirectChainsPost::default()
        }
    }
}

/// Request for [`OnPageApi::non_indexable`].
/// See <https://docs.dataforseo.com/v3/on_page/non_indexable/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiNonIndexablePost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Maximum number of pages to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Value>>,
}

impl OnPageApiNonIndexablePost {
    /// Creates a non-indexable request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiNonIndexablePost {
            id,
            ..OnPageApiNonIndexablePost::default()
        }
    }
}

/// Request for [`OnPageApi::waterfall`].
/// See <https://docs.dataforseo.com/v3/on_page/waterfall/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiWaterfallPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Page URL to build the resource loading waterfall for.
    pub url: String,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiWaterfallPost {
    /// Creates a waterfall request for task `id` and page `url`.
    pub fn new(id: String, url: String) -> OnPageApiWaterfallPost {
        OnPageApiWaterfallPost { id, url, tag: None }
    }
}

/// Request for [`OnPageApi::keyword_density`].
/// See <https://docs.dataforseo.com/v3/on_page/keyword_density/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiKeywordDensityPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Number of words per keyword (1-5).
    pub keyword_length: i32,
    /// Restrict results to this page URL; omit for whole-site results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Maximum number of keywords to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Result filtering rules (max 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Vec<String>>>,
    /// Sorting rules, each as "field,asc" or "field,desc" (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiKeywordDensityPost {
    /// Creates a keyword-density request for task `id` and the given `keyword_length`.
    pub fn new(id: String, keyword_length: i32) -> OnPageApiKeywordDensityPost {
        OnPageApiKeywordDensityPost {
            id,
            keyword_length,
            ..OnPageApiKeywordDensityPost::default()
        }
    }
}

/// Request for [`OnPageApi::microdata`].
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiMicrodataPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Page URL to validate structured markup for.
    pub url: String,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiMicrodataPost {
    /// Creates a microdata request for task `id` and page `url`.
    pub fn new(id: String, url: String) -> OnPageApiMicrodataPost {
        OnPageApiMicrodataPost { id, url, tag: None }
    }
}

/// Request for [`OnPageApi::raw_html`].
/// See <https://docs.dataforseo.com/v3/on_page/raw_html/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiRawHtmlPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Page URL whose stored raw HTML to return.
    pub url: String,
}

impl OnPageApiRawHtmlPost {
    /// Creates a raw-HTML request for task `id` and page `url`.
    pub fn new(id: String, url: String) -> Self {
        OnPageApiRawHtmlPost { id, url }
    }
}

/// Request for [`OnPageApi::page_screenshot`].
/// See <https://docs.dataforseo.com/v3/on_page/page_screenshot/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiPageScreenshotPost {
    /// Page URL to capture.
    pub url: String,
    /// Value of the Accept-Language header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// Custom User-Agent header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
    /// Device preset for rendering: "desktop", "mobile", or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_preset: Option<String>,
    /// Viewport width used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Viewport height used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Device pixel ratio used for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f32>,
    /// Capture the entire scrollable page rather than just the viewport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page_screenshot: Option<bool>,
    /// Hide cookie-consent popups before capturing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cookie_popup: Option<bool>,
    /// Retry using a different proxy pool if the first attempt fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pool: Option<bool>,
    /// Proxy location to load the page from: "us" or "de".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pool_for_scan: Option<String>,
}

impl OnPageApiPageScreenshotPost {
    /// Creates a screenshot request for the given page `url`.
    pub fn new(url: String) -> Self {
        OnPageApiPageScreenshotPost {
            url,
            ..OnPageApiPageScreenshotPost::default()
        }
    }
}

/// Request for [`OnPageApi::content_parsing`].
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiContentParsingPost {
    /// Task identifier returned by the Task POST endpoint.
    pub id: String,
    /// Page URL whose parsed content to return.
    pub url: String,
    /// Return the page content rendered as Markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_view: Option<bool>,
}

impl OnPageApiContentParsingPost {
    /// Creates a content-parsing request for task `id` and page `url`.
    pub fn new(id: String, url: String) -> Self {
        OnPageApiContentParsingPost {
            id,
            url,
            ..OnPageApiContentParsingPost::default()
        }
    }
}

/// Request for [`OnPageApi::content_parsing_live`].
/// See <https://docs.dataforseo.com/v3/on_page/content_parsing/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiContentParsingLivePost {
    /// Page URL to fetch and parse.
    pub url: String,
    /// Custom User-Agent header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
    /// Device preset for rendering: "desktop", "mobile", or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_preset: Option<String>,
    /// Viewport width used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Viewport height used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Device pixel ratio used for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f32>,
    /// Store the raw HTML for later retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_raw_html: Option<bool>,
    /// Hide cookie-consent popups before loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cookie_popup: Option<bool>,
    /// Value of the Accept-Language header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// Execute page JavaScript (incurs extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_javascript: Option<bool>,
    /// Emulate full browser rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_browser_rendering: Option<bool>,
    /// Allow XMLHttpRequest calls while rendering the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_xhr: Option<bool>,
    /// Retry using a different proxy pool if the first attempt fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pool: Option<bool>,
    /// Proxy location to load the page from: "us" or "de".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pool_for_scan: Option<String>,
    /// Return the page content rendered as Markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_view: Option<bool>,
}

impl OnPageApiContentParsingLivePost {
    /// Creates a live content-parsing request for the given page `url`.
    pub fn new(url: String) -> Self {
        OnPageApiContentParsingLivePost {
            url,
            ..OnPageApiContentParsingLivePost::default()
        }
    }
}

/// Request for [`OnPageApi::instant_pages`].
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiInstantPagesPost {
    /// Page URL to crawl.
    pub url: String,
    /// Custom User-Agent header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
    /// Device preset for rendering: "desktop", "mobile", or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_preset: Option<String>,
    /// Viewport width used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Viewport height used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Device pixel ratio used for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f32>,
    /// Store the raw HTML for later retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_raw_html: Option<bool>,
    /// Value of the Accept-Language header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// Load page resources (images, scripts, stylesheets) while crawling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_resources: Option<bool>,
    /// Execute page JavaScript (incurs extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_javascript: Option<bool>,
    /// Emulate full browser rendering to measure Core Web Vitals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_browser_rendering: Option<bool>,
    /// Hide cookie-consent popups before loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cookie_popup: Option<bool>,
    /// Return partial results even if the request times out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_despite_timeout: Option<bool>,
    /// Allow XMLHttpRequest calls while rendering the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_xhr: Option<bool>,
    /// Custom JavaScript to execute on the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_js: Option<String>,
    /// Validate structured microdata markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_micromarkup: Option<bool>,
    /// Check spelling on the page using the Hunspell library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_spell: Option<bool>,
    /// Custom threshold values for the checks in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks_threshold: Option<Value>,
    /// Retry using a different proxy pool if the first attempt fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pool: Option<bool>,
    /// Proxy location to load the page from: "us" or "de".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pool_for_scan: Option<String>,
}

impl OnPageApiInstantPagesPost {
    /// Creates an instant-pages request for the given page `url`.
    pub fn new(url: String) -> Self {
        OnPageApiInstantPagesPost {
            url,
            ..OnPageApiInstantPagesPost::default()
        }
    }
}

/// Request for [`OnPageApi::id_list`].
/// See <https://docs.dataforseo.com/v3/on_page/id_list/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiIdListPost {
    /// Start of the time range, as "yyyy-mm-dd hh-mm-ss +00:00".
    pub datetime_from: String,
    /// End of the time range, as "yyyy-mm-dd hh-mm-ss +00:00".
    pub datetime_to: String,
    /// Maximum number of task IDs to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Sort order by post date: "asc" or "desc".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Include each task's metadata in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
}

impl OnPageApiIdListPost {
    /// Creates an id-list request for tasks created between `datetime_from` and `datetime_to`.
    pub fn new(datetime_from: String, datetime_to: String) -> Self {
        OnPageApiIdListPost {
            datetime_from,
            datetime_to,
            ..OnPageApiIdListPost::default()
        }
    }
}

/// Request for [`OnPageApi::errors`].
/// See <https://docs.dataforseo.com/v3/on_page/errors/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiErrorsPost {
    /// Maximum number of error records to return (default 1000, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Offset into the results array (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Restrict results to a specific API function name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_function: Option<String>,
    /// Start of the time range (within the last 7 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_from: Option<String>,
    /// End of the time range (within the last 7 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_to: Option<String>,
}

/// Request for [`OnPageApi::force_stop`].
/// See <https://docs.dataforseo.com/v3/on_page/force_stop/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiForceStopPost {
    /// Identifier of the crawling task to stop.
    pub id: String,
}

impl OnPageApiForceStopPost {
    /// Creates a force-stop request for the given task `id`.
    pub fn new(id: String) -> Self {
        OnPageApiForceStopPost { id }
    }
}

/// Request for [`OnPageApi::lighthouse_task_post`].
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiLighthouseTaskPost {
    /// Page URL to audit with Lighthouse.
    pub url: String,
    /// Run the audit emulating a mobile device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_mobile: Option<bool>,
    /// Lighthouse categories to run (e.g. "performance", "seo").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Specific Lighthouse audits to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audits: Option<Vec<String>>,
    /// Lighthouse version to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Full language name for the report (e.g. "English").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code for the report (e.g. "en").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL notified by GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
    /// URL the results are POSTed to when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
}

impl OnPageApiLighthouseTaskPost {
    /// Creates a Lighthouse task request for the given page `url`.
    pub fn new(url: String) -> Self {
        OnPageApiLighthouseTaskPost {
            url,
            ..OnPageApiLighthouseTaskPost::default()
        }
    }
}

/// Request for [`OnPageApi::lighthouse_live`].
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageApiLighthouseLivePost {
    /// Page URL to audit with Lighthouse.
    pub url: String,
    /// Run the audit emulating a mobile device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_mobile: Option<bool>,
    /// Lighthouse categories to run (e.g. "performance", "seo").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Specific Lighthouse audits to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audits: Option<Vec<String>>,
    /// Lighthouse version to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Full language name for the report (e.g. "English").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code for the report (e.g. "en").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Custom User-Agent header sent while loading the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
    /// Viewport width used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Viewport height used for rendering, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Device pixel ratio used for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f32>,
    /// Network throttling method: "provided", "devtools", or "simulate".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_network_throttling_method: Option<String>,
    /// CPU slowdown multiplier applied during the audit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_cpu_throttling_multiplier: Option<f32>,
    /// Network throttling preset applied during the audit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_network_throttling: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OnPageApiLighthouseLivePost {
    /// Creates a live Lighthouse request for the given page `url`.
    pub fn new(url: String) -> Self {
        OnPageApiLighthouseLivePost {
            url,
            ..OnPageApiLighthouseLivePost::default()
        }
    }
}
