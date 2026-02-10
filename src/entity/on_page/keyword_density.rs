use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensity {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub items_count: Option<i64>,
    pub items: Option<Vec<OnPageDataApiKeywordDensityItem>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensityItem {
    pub keyword: Option<String>,
    pub frequency: Option<i32>,
    pub density: Option<f32>,
}
