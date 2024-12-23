use serde::{Deserialize, Serialize};
use crate::entity::OnPageDataApiCrawlStatus;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPageScreensShot {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub items_count: Option<i32>,
    pub items: Option<OnPageDataApiPageScreensShotItem>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPageScreensShotItem {
    pub image: Option<String>,
}

