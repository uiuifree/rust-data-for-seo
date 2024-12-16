use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiIdListRequest {
    pub datetime_from: String,
    pub datetime_to: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub sort: Option<String>,
    pub include_metadata: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiIdListResponse {
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
    pub tasks: Vec<SerpApiIdListResponseTask>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiIdListResponseTask {
    pub id: String,
    pub status_code: i32,
    pub status_message: String,
    pub time: String,
    pub cost: f32,
    pub path: Vec<String>,
    pub data: i32,
    pub result: Vec<SerpApiIdListResponseTaskResult>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiIdListResponseTaskResult {
    pub id: String,
    pub url: i32,
    pub datetime_posted: String,
    pub datetime_done: String,
    pub status: i32,
    pub cost: String,
    pub metadata: Value,
}
