mod id_list;
mod keywords_data;
mod on_page;
mod serp;

pub use id_list::*;
pub use keywords_data::*;
pub use on_page::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use serp::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiResponseData<R> {
    pub version: String,
    pub status_code: i32,
    pub status_message: String,
    pub time: String,
    pub cost: f32,
    pub tasks_count: i32,
    pub tasks_error: i32,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort: Option<String>,
    pub include_metadata: Option<bool>,
    pub tasks: Vec<DataForSeoApiTask<R>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiTask<R> {
    pub id: String,
    pub status_code: i32,
    pub status_message: String,
    pub time: String,
    pub cost: f32,
    pub path: Vec<String>,
    pub data: Value,
    pub result: Option<Vec<R>>,
}

#[derive(Debug,Clone)]
pub enum TaskStatus {
    RequestSuccess(i32),
    TaskWaiting(i32),
    TaskNotFound(i32),
    RateLimit(i32),
    Other(i32),
}
impl<R> DataForSeoApiTask<R> {
    pub fn task_status(&self) -> TaskStatus {
        if 20000 <= self.status_code && self.status_code < 30000 {
            TaskStatus::RequestSuccess(self.status_code)
        } else if vec![40601, 40602].contains(&self.status_code) {
            TaskStatus::TaskWaiting(self.status_code)
        } else if vec![40401,].contains(&self.status_code) {
            TaskStatus::TaskNotFound(self.status_code)
        } else if vec![40202].contains(&self.status_code) {
            TaskStatus::RateLimit(self.status_code)
        } else {
            TaskStatus::Other(self.status_code)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiTaskResult<T> {
    pub keyword: String,
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    pub se_domain: Option<String>,
    pub location_code: i32,
    pub language_code: Option<String>,
    pub check_url: Option<String>,
    pub datetime: Option<String>,
    pub spell: Option<Value>,
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    pub item_types: Option<Vec<String>>,
    pub se_results_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<T>>,
}
