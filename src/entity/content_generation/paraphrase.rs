use serde::{Deserialize, Serialize};

/// Result of `content_generation/paraphrase/live`: a rewritten version of the text.
/// See <https://docs.dataforseo.com/v3/content_generation/paraphrase/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiParaphrase {
    /// Number of input tokens in the POST request.
    pub input_tokens: Option<i64>,
    /// Number of output tokens in the response.
    pub output_tokens: Option<i64>,
    /// Number of new tokens in the response.
    pub new_tokens: Option<i64>,
    /// Paraphrased version of the given text.
    pub generated_text: Option<String>,
}
