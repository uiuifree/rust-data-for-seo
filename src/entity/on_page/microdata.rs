use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Structured data (microdata/JSON-LD) validation results for a page.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiMicrodata {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Aggregated counts of validation issues by severity.
    pub test_summary: Option<OnPageDataApiTestSummary>,
    /// Number of items returned in this response.
    pub items_count: Option<i32>,
    /// Parsed structured-data items found on the page.
    pub items: Option<Vec<OnPageDataApiItemJsonLd>>,
}

/// Counts of structured-data validation issues grouped by severity.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiTestSummary {
    /// Number of fatal-level issues.
    pub fatal: Option<i32>,
    /// Number of error-level issues.
    pub error: Option<i32>,
    /// Number of warning-level issues.
    pub warning: Option<i32>,
    /// Number of info-level notices.
    pub info: Option<i32>,
}

/// A single structured-data item parsed from the page.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiItemJsonLd {
    /// Structured-data format of the item, e.g. "json_ld" or "microdata".
    #[serde(rename = "type")]
    pub items_type: Option<String>,
    /// Parsed schema and validation details for the item.
    pub inspection_info: Option<OnPageDataSchema>,
}

/// Parsed schema of a structured-data item.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchema {
    /// Schema.org types declared by the item.
    pub types: Option<Vec<String>>,
    /// Properties defined on the item.
    pub fields: Option<Vec<OnPageDataSchemaField>>,
}
/// A single property of a structured-data schema.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchemaField {
    /// Name of the property.
    pub name: Option<String>,
    /// Schema.org types allowed for the property.
    pub types: Option<Vec<String>>,
    /// Value assigned to the property.
    pub value: Option<String>,
    /// Validation result recorded for the property.
    pub test_results: Option<OnPageDataSchemaTestResult>,
    /// Nested properties of the property.
    pub fields: Option<Vec<OnPageDataSchemaField>>,
}
/// Validation outcome for a structured-data property.
/// See <https://docs.dataforseo.com/v3/on_page/microdata/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataSchemaTestResult {
    /// Severity of the result: "fatal", "error", "warning" or "info".
    pub level: Option<String>,
    /// Human-readable description of the validation result.
    pub message: Option<String>,
}
