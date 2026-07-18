use serde::{Deserialize, Serialize};

/// Result of `content_generation/grammar_rules`: the grammar and spelling rules
/// used by the Check Grammar endpoint.
/// See <https://docs.dataforseo.com/v3/content_generation/grammar_rules/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGrammarRules {
    /// Grammar and spelling rule IDs used by the Check Grammar endpoint.
    pub rules: Option<Vec<String>>,
}
