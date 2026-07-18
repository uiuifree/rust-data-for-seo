//! Entities for Google Questions and Answers (`task_get` / `live`).
//! See <https://docs.dataforseo.com/v3/business_data/google/questions_and_answers/task_get/>.

use serde::{Deserialize, Serialize};

/// Result of a Questions and Answers request: metadata plus questions.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiQuestionsAndAnswersResult {
    /// Business name or identifier the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"questions_and_answers"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Search engine domain the data was collected from (e.g. `"google.com"`).
    pub se_domain: Option<String>,
    /// Location code the task was run for.
    pub location_code: Option<i64>,
    /// Language code the task was run for.
    pub language_code: Option<String>,
    /// URL of the Google results page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Google-defined customer identifier (CID) for the business.
    pub cid: Option<String>,
    /// Unique Google feature identifier for the business.
    pub feature_id: Option<String>,
    /// Distinct element types present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Number of questions in `items`.
    pub items_count: Option<i64>,
    /// Parsed questions, each with its answers.
    pub items: Option<Vec<BusinessDataApiQuestionItem>>,
}

impl BusinessDataApiQuestionsAndAnswersResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiQuestionItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single question, together with up to five answers in its `items`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiQuestionItem {
    /// Element type, e.g. `"google_business_question_item"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Unique identifier of the question.
    pub question_id: Option<String>,
    /// Direct URL to the question.
    pub url: Option<String>,
    /// URL of the asker's profile image.
    pub profile_image_url: Option<String>,
    /// URL of the asker's Google profile.
    pub profile_url: Option<String>,
    /// Display name of the asker.
    pub profile_name: Option<String>,
    /// Current text of the question.
    pub question_text: Option<String>,
    /// Original question text before any edit.
    pub original_question_text: Option<String>,
    /// Human-readable time since the question was posted.
    pub time_ago: Option<String>,
    /// UTC timestamp the question was posted.
    pub timestamp: Option<String>,
    /// Answers to this question (up to five).
    pub items: Option<Vec<BusinessDataApiAnswerItem>>,
}

/// A single answer nested inside a [`BusinessDataApiQuestionItem`].
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiAnswerItem {
    /// Element type, e.g. `"google_business_answer_element"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Unique identifier of the answer.
    pub answer_id: Option<String>,
    /// URL of the responder's profile image.
    pub profile_image_url: Option<String>,
    /// URL of the responder's Google profile.
    pub profile_url: Option<String>,
    /// Display name of the responder.
    pub profile_name: Option<String>,
    /// Current text of the answer.
    pub answer_text: Option<String>,
    /// Original answer text before any edit.
    pub original_answer_text: Option<String>,
    /// Human-readable time since the answer was posted.
    pub time_ago: Option<String>,
    /// UTC timestamp the answer was posted.
    pub timestamp: Option<String>,
    /// Number of upvotes the answer received.
    pub votes_count: Option<i64>,
}
