mod microdata;
mod raw_html;
mod resource;
mod keyword_density;
mod page_screenshot;
mod waterfall;
mod pages;

pub use microdata::*;
pub use raw_html::*;
pub use resource::*;
pub use pages::*;
pub use keyword_density::*;
pub use page_screenshot::*;
pub use waterfall::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiCrawlStatus {
    pub max_crawl_pages: Option<i32>,
    pub pages_in_queue: Option<i32>,
    pub pages_crawled: Option<i32>,
}
