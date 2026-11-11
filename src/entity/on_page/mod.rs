mod content_parsing;
mod duplicate_content;
mod duplicate_tags;
mod errors;
mod id_list;
mod instant_pages;
mod keyword_density;
mod lighthouse;
mod links;
mod microdata;
mod non_indexable;
mod page_screenshot;
mod pages;
mod raw_html;
mod redirect_chains;
mod resource;
mod resources;
mod summary;
mod tasks_ready;
mod waterfall;

pub use content_parsing::*;
pub use duplicate_content::*;
pub use duplicate_tags::*;
pub use errors::*;
pub use id_list::*;
pub use instant_pages::*;
pub use keyword_density::*;
pub use lighthouse::*;
pub use links::*;
pub use microdata::*;
pub use non_indexable::*;
pub use page_screenshot::*;
pub use pages::*;
pub use raw_html::*;
pub use redirect_chains::*;
pub use resource::*;
pub use resources::*;
use serde::{Deserialize, Serialize};
pub use summary::*;
pub use tasks_ready::*;
pub use waterfall::*;

/// Statistics on the progress of an OnPage crawling session.
/// See <https://docs.dataforseo.com/v3/on_page/summary/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiCrawlStatus {
    /// Maximum number of pages set to be crawled.
    pub max_crawl_pages: Option<i64>,
    /// Number of pages still queued to be crawled.
    pub pages_in_queue: Option<i64>,
    /// Number of pages crawled so far.
    pub pages_crawled: Option<i64>,
}

/// Result of `on_page/uncrawlable_resources`.
/// See <https://docs.dataforseo.com/v3/on_page/uncrawlable_resources/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiUncrawlableResources {
    /// Status of the crawling session.
    pub crawl_progress: Option<String>,
    /// Details of the crawling session.
    pub crawl_status: Option<serde_json::Value>,
    /// Current offset.
    pub current_offset: Option<i64>,
    /// Total number of uncrawlable resources found.
    pub total_items_count: Option<i64>,
    /// Number of resources in the `items` array.
    pub items_count: Option<i64>,
    /// Uncrawlable resources.
    pub items: Option<Vec<OnPageDataApiUncrawlableResourceItem>>,
}

/// A resource that could not be crawled.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiUncrawlableResourceItem {
    /// URL of the resource.
    pub url: Option<String>,
    /// Reason the resource is uncrawlable.
    pub reason: Option<String>,
    /// HTTP status code.
    pub status_code: Option<i32>,
    /// UTC time the resource was fetched.
    pub fetch_time: Option<String>,
    /// Metadata of the resource.
    pub meta: Option<OnPageDataApiUncrawlableResourceMeta>,
}

/// Metadata of an uncrawlable resource.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiUncrawlableResourceMeta {
    /// Actual content type.
    pub content_type: Option<String>,
    /// Expected content types.
    pub expected_content_types: Option<Vec<String>>,
}

/// Result of `on_page/lighthouse/audits`.
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/audits/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiLighthouseAudits {
    /// Available audits.
    pub audits: Option<Vec<String>>,
}

/// Result of `on_page/lighthouse/versions`.
/// See <https://docs.dataforseo.com/v3/on_page/lighthouse/versions/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiLighthouseVersions {
    /// Available versions (the field is spelled this way by the API).
    pub availible_versions: Option<Vec<OnPageDataApiLighthouseVersion>>,
}

/// A Lighthouse version.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct OnPageDataApiLighthouseVersion {
    /// Lighthouse version.
    pub version: Option<String>,
    /// `true` when the version is used by default.
    pub default: Option<bool>,
}
