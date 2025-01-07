mod keyword_density;
mod microdata;
mod page_screenshot;
mod pages;
mod raw_html;
mod resource;
mod waterfall;

pub use keyword_density::*;
pub use microdata::*;
pub use page_screenshot::*;
pub use pages::*;
pub use raw_html::*;
pub use resource::*;
use serde::{Deserialize, Serialize};
pub use waterfall::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiCrawlStatus {
    pub max_crawl_pages: Option<i32>,
    pub pages_in_queue: Option<i32>,
    pub pages_crawled: Option<i32>,
}
