use serde::{Deserialize, Serialize};

/// Result of `content_generation/generate_sub_topics/live`: subtopics for a topic.
/// See <https://docs.dataforseo.com/v3/content_generation/generate_sub_topics/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateSubTopics {
    /// Number of input tokens.
    pub input_tokens: Option<i64>,
    /// Number of output tokens.
    pub output_tokens: Option<i64>,
    /// Number of newly generated tokens.
    pub new_tokens: Option<i64>,
    /// Resulting subtopics.
    pub sub_topics: Option<Vec<String>>,
}
