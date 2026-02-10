use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiMicrodata {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub test_summary: Option<OnPageDataApiTestSummary>,
    pub items_count: Option<i64>,
    pub items: Option<Vec<OnPageDataApiItemJsonLd>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiTestSummary {
    pub fatal: Option<i32>,
    pub error: Option<i32>,
    pub warning: Option<i32>,
    pub info: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiItemJsonLd {
    #[serde(rename = "type")]
    pub items_type: Option<String>,
    pub inspection_info: Option<OnPageDataSchema>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchema {
    pub types: Option<Vec<String>>,
    pub fields: Option<Vec<OnPageDataSchemaField>>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchemaField {
    pub name: Option<String>,
    pub types: Option<Vec<String>>,
    pub value: Option<String>,
    pub test_results: Option<OnPageDataSchemaTestResult>,
    pub fields: Option<Vec<OnPageDataSchemaField>>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchemaTestResult {
    pub level: Option<String>,
    pub message: Option<String>,
}
