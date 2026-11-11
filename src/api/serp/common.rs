//! SERP API endpoints shared by every search engine.

use serde::{Deserialize, Serialize};

/// Request body for `serp/screenshot`.
/// See <https://docs.dataforseo.com/v3/serp/screenshot/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiScreenshotRequest {
    /// Identifier of the task to capture.
    pub task_id: String,
    /// Browser preset (`desktop`, `mobile` or `tablet`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_preset: Option<String>,
    /// Browser screen width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Browser screen height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Browser scale factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_scale_factor: Option<f64>,
    /// Number of SERP pages to capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

impl SerpApiScreenshotRequest {
    /// Creates a request for the given task.
    pub fn new<T: Into<String>>(task_id: T) -> Self {
        SerpApiScreenshotRequest {
            task_id: task_id.into(),
            ..Default::default()
        }
    }
}

/// Request body for `serp/ai_summary`.
/// See <https://docs.dataforseo.com/v3/serp/ai_summary/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiAiSummaryRequest {
    /// Identifier of the task to summarize.
    pub task_id: String,
    /// Additional instruction for the summarizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Include extra SERP features in the summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_extra: Option<bool>,
    /// Fetch the content of the pages in the SERP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_content: Option<bool>,
    /// Include source links in the summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_links: Option<bool>,
}

impl SerpApiAiSummaryRequest {
    /// Creates a request for the given task.
    pub fn new<T: Into<String>>(task_id: T) -> Self {
        SerpApiAiSummaryRequest {
            task_id: task_id.into(),
            ..Default::default()
        }
    }
}
