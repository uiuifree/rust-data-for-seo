mod id_list;
mod serp;
mod keywords_data;
mod on_page;

pub use id_list::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use serp::*;
pub use keywords_data::*;
pub use on_page::*;

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
