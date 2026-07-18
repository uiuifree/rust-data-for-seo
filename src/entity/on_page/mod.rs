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
