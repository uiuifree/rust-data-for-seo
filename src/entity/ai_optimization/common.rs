//! Shared models used across the AI Optimization sub-APIs.
//!
//! See <https://docs.dataforseo.com/v3/ai_optimization/overview/>.

use serde::{Deserialize, Serialize};

/// A language available for a location.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLanguage {
    /// Full name of the language (e.g. `"English"`).
    pub language_name: Option<String>,
    /// ISO 639-1 language code (e.g. `"en"`).
    pub language_code: Option<String>,
}

/// A supported location together with the languages available for it.
///
/// Returned by the `locations_and_languages` endpoints of AI Keyword Data and
/// LLM Mentions.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationLocationLanguage {
    /// DataForSEO location code (e.g. `2840` for the United States).
    pub location_code: Option<i32>,
    /// Full name of the location (e.g. `"United States"`).
    pub location_name: Option<String>,
    /// Languages supported for this location.
    pub available_languages: Option<Vec<AiOptimizationLanguage>>,
}

/// A supported location for the LLM Scraper (standard SERP location shape).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationScraperLocation {
    /// DataForSEO location code.
    pub location_code: Option<i32>,
    /// Full name of the location.
    pub location_name: Option<String>,
    /// Location code of the parent location, if any.
    pub location_code_parent: Option<i32>,
    /// ISO 3166-1 alpha-2 country code the location belongs to.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. `"Country"`, `"City"`).
    pub location_type: Option<String>,
}

/// Monthly search-volume data point.
///
/// Covers both AI Keyword Data (`ai_monthly_searches`) and LLM Mentions
/// (`monthly_searches`); only the relevant fields are populated per endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationMonthlySearch {
    /// Year the data point refers to.
    pub year: Option<i32>,
    /// Month the data point refers to (`1`–`12`).
    pub month: Option<i32>,
    /// AI search volume for the month.
    pub ai_search_volume: Option<f64>,
    /// Classic search volume for the month, when provided.
    pub search_volume: Option<i64>,
}

/// A single queued task returned by a `tasks_ready` endpoint.
///
/// See <https://docs.dataforseo.com/v3/ai_optimization/chat_gpt/llm_responses/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AiOptimizationTaskReady {
    /// Task identifier used to collect the result via `task_get`.
    pub id: Option<String>,
    /// Search engine the task belongs to.
    pub se: Option<String>,
    /// Function name of the task.
    pub function: Option<String>,
    /// Relative endpoint to fetch the completed result.
    pub endpoint: Option<String>,
    /// Relative endpoint to fetch the parsed (advanced) result, for scraper tasks.
    pub endpoint_advanced: Option<String>,
    /// Relative endpoint to fetch the raw HTML result, for scraper tasks.
    pub endpoint_html: Option<String>,
    /// UTC timestamp at which the task was queued.
    pub date_posted: Option<String>,
    /// User-defined identifier echoed from the task request.
    pub tag: Option<String>,
}
