//! Response and data models, organized by API domain.

mod ai_optimization;
mod app_data;
mod appendix;
mod backlinks;
mod business_data;
mod content_analysis;
mod content_generation;
mod dataforseo_labs;
mod domain_analytics;
mod keywords_data;
mod merchant;
mod on_page;
mod serp;

pub use ai_optimization::*;
pub use app_data::*;
pub use appendix::*;
pub use backlinks::*;
pub use business_data::*;
pub use content_analysis::*;
pub use content_generation::*;
pub use dataforseo_labs::*;
pub use domain_analytics::*;
pub use keywords_data::*;
pub use merchant::*;
pub use on_page::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use serp::*;

/// Top-level response envelope shared by every DataForSEO endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiResponseData<R> {
    /// Current API version.
    pub version: String,
    /// General status code of the request; `20000` means "Ok".
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub status_code: i32,
    /// Informational message accompanying `status_code`.
    pub status_message: String,
    /// Total execution time.
    pub time: String,
    /// Total cost of the request in USD.
    pub cost: f64,
    /// Number of tasks in the `tasks` array.
    pub tasks_count: i32,
    /// Number of tasks that returned an error.
    pub tasks_error: i32,
    /// Echo of the `limit` request parameter, where applicable.
    pub limit: Option<i32>,
    /// Echo of the `offset` request parameter, where applicable.
    pub offset: Option<i32>,
    /// Echo of the `sort` request parameter, where applicable.
    pub sort: Option<String>,
    /// Echo of the `include_metadata` request parameter, where applicable.
    pub include_metadata: Option<bool>,
    /// Tasks executed within this request.
    pub tasks: Option<Vec<DataForSeoApiTask<R>>>,
}

impl<R> DataForSeoApiResponseData<R> {
    /// All tasks in the response (empty slice when the API returned none).
    pub fn tasks(&self) -> &[DataForSeoApiTask<R>] {
        self.tasks.as_deref().unwrap_or_default()
    }

    /// Consumes the response and returns its tasks.
    pub fn into_tasks(self) -> Vec<DataForSeoApiTask<R>> {
        self.tasks.unwrap_or_default()
    }

    /// Iterates over every result of every task.
    pub fn results(&self) -> impl Iterator<Item = &R> {
        self.tasks().iter().flat_map(|task| task.results().iter())
    }

    /// Consumes the response and returns every result of every task.
    pub fn into_results(self) -> Vec<R> {
        self.into_tasks()
            .into_iter()
            .flat_map(|task| task.result.unwrap_or_default())
            .collect()
    }

    /// First result across all tasks, if any.
    pub fn first_result(&self) -> Option<&R> {
        self.results().next()
    }
}

/// A single task inside a [`DataForSeoApiResponseData`] envelope.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiTask<R> {
    /// Unique task identifier in UUID format.
    pub id: String,
    /// Task status code; `20000` means "Ok".
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub status_code: i32,
    /// Informational message accompanying `status_code`.
    pub status_message: String,
    /// Execution time of this task.
    pub time: String,
    /// Cost of this task in USD.
    pub cost: f64,
    /// URL path segments of the executed function (e.g. `["v3", "serp", ...]`).
    pub path: Vec<String>,
    /// Echo of the parameters submitted when the task was created.
    pub data: Value,
    /// Results produced by this task.
    pub result: Option<Vec<R>>,
}

impl<R> DataForSeoApiTask<R> {
    /// Results of this task (empty slice when the API returned none).
    pub fn results(&self) -> &[R] {
        self.result.as_deref().unwrap_or_default()
    }

    /// `true` when the task finished successfully (status code `20000`..`29999`).
    pub fn is_success(&self) -> bool {
        matches!(self.task_status(), TaskStatus::RequestSuccess(_))
    }

    /// Classifies the task's `status_code`.
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub fn task_status(&self) -> TaskStatus {
        match self.status_code {
            20000..=29999 => TaskStatus::RequestSuccess(self.status_code),
            40601 | 40602 => TaskStatus::TaskWaiting(self.status_code),
            40401 => TaskStatus::TaskNotFound(self.status_code),
            40202 => TaskStatus::RateLimit(self.status_code),
            other => TaskStatus::Other(other),
        }
    }
}

/// Classification of a task-level `status_code`.
#[derive(Debug, Clone)]
pub enum TaskStatus {
    /// The task completed successfully (`20000`..`29999`).
    RequestSuccess(i32),
    /// The task was created but is not finished yet (`40601`, `40602`).
    TaskWaiting(i32),
    /// No task with the given id was found (`40401`).
    TaskNotFound(i32),
    /// The rate limit was exceeded (`40202`).
    RateLimit(i32),
    /// Any other status code.
    Other(i32),
}

/// Common SERP task result wrapper: metadata about the search plus its `items`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiTaskResult<T> {
    /// Keyword the search was performed for, UTF-8 decoded.
    pub keyword: String,
    /// Search engine type (e.g. `organic`, `maps`).
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    /// Search engine domain the request was made to (e.g. `google.co.jp`).
    pub se_domain: Option<String>,
    /// Location code used in the task.
    pub location_code: i32,
    /// Language code used in the task.
    pub language_code: Option<String>,
    /// Direct URL of the search query.
    pub check_url: Option<String>,
    /// Date and time when the result was received, UTC.
    pub datetime: Option<String>,
    /// Autocorrection info applied by the search engine, if any.
    pub spell: Option<Value>,
    /// Search refinement chips shown in the SERP, if any.
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    /// Element types present in `items` (e.g. `organic`, `people_also_ask`).
    pub item_types: Option<Vec<String>>,
    /// Total number of results the search engine reported.
    pub se_results_count: Option<i64>,
    /// Number of items returned in `items`.
    pub items_count: Option<i64>,
    /// Elements of the search results page.
    pub items: Option<Vec<T>>,
}

impl<T> SerpApiTaskResult<T> {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[T] {
        self.items.as_deref().unwrap_or_default()
    }
}
