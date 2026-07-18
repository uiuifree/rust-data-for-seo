use serde::{Deserialize, Serialize};

/// Result of `content_generation/check_grammar/live`: grammar and spelling errors.
/// See <https://docs.dataforseo.com/v3/content_generation/check_grammar/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiCheckGrammar {
    /// Number of input tokens in the POST request.
    pub input_tokens: Option<i64>,
    /// Number of output tokens in the response.
    pub output_tokens: Option<i64>,
    /// Number of new tokens in the response.
    pub new_tokens: Option<i64>,
    /// The text supplied in the POST request.
    pub initial_text: Option<String>,
    /// Language code supplied in the POST request.
    pub language_code: Option<String>,
    /// Number of errors returned in `items`.
    pub items_count: Option<i64>,
    /// Grammar and spelling errors found in the text.
    pub items: Option<Vec<ContentGenerationCheckGrammarItem>>,
}

/// Result item of `content_generation/check_grammar/languages`: a supported
/// grammar-check language.
/// See <https://docs.dataforseo.com/v3/content_generation/check_grammar/languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiCheckGrammarLanguage {
    /// Full language name.
    pub language_name: Option<String>,
    /// ISO 639-1 language code.
    pub language_code: Option<String>,
}

/// A single grammar or spelling error found in the text.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationCheckGrammarItem {
    /// Type of element.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// Message describing the grammar or spelling error.
    pub message: Option<String>,
    /// Description of the grammar or spelling error.
    pub description: Option<String>,
    /// Suggested corrections.
    pub suggestions: Option<Vec<String>>,
    /// Character offset of the error within the text.
    pub offset: Option<i64>,
    /// Character length of the erroneous span.
    pub length: Option<i64>,
    /// ID of the grammar or spelling rule that flagged the error.
    pub rule_id: Option<String>,
    /// Description of the rule.
    pub rule_description: Option<String>,
    /// Issue type flagged by the rule.
    pub rule_issue_type: Option<String>,
    /// ID of the rule category.
    pub rule_category_id: Option<String>,
    /// Name of the rule category.
    pub rule_category_name: Option<String>,
}
