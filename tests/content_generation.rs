use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

/// Deserialization tests run against documented sample responses and need no
/// credentials. The `#[ignore]`d tests hit the live DataForSEO API.
#[cfg(test)]
mod deserialize {
    use data_for_seo::entity::{
        ContentGenerationApiCheckGrammar, ContentGenerationApiGenerateMetaTags,
        ContentGenerationApiGenerateText, ContentGenerationApiParaphrase,
        ContentGenerationApiTextSummary, DataForSeoApiResponseData,
    };

    #[test]
    fn generate_text_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "2.0 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "11111111-1111-1111-1111-111111111111",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "1.9 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "content_generation", "generate_text", "live"],
                "data": {},
                "result": [{
                    "input_tokens": 12,
                    "output_tokens": 55,
                    "new_tokens": 55,
                    "generated_text": "Apple is a technology company.",
                    "supplement_token": "supp_123"
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentGenerationApiGenerateText> =
            serde_json::from_str(json).expect("generate_text should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.output_tokens, Some(55));
        assert_eq!(
            result.generated_text.as_deref(),
            Some("Apple is a technology company.")
        );
        assert_eq!(result.supplement_token.as_deref(), Some("supp_123"));
    }

    #[test]
    fn generate_meta_tags_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.5 sec.",
            "cost": 0.001,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "22222222-2222-2222-2222-222222222222",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "1.4 sec.",
                "cost": 0.001,
                "result_count": 1,
                "path": ["v3", "content_generation", "generate_meta_tags", "live"],
                "data": {},
                "result": [{
                    "input_tokens": 147,
                    "output_tokens": 65,
                    "new_tokens": 65,
                    "title": "GMBCRUSH is a Google My Business SEO Audit Tool",
                    "description": "GMB Crush is a Google My Business SEO Audit Tool."
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentGenerationApiGenerateMetaTags> =
            serde_json::from_str(json).expect("generate_meta_tags should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.input_tokens, Some(147));
        assert_eq!(
            result.title.as_deref(),
            Some("GMBCRUSH is a Google My Business SEO Audit Tool")
        );
        assert!(result.description.is_some());
    }

    #[test]
    fn check_grammar_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.0 sec.",
            "cost": 0.0001,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "33333333-3333-3333-3333-333333333333",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.9 sec.",
                "cost": 0.0001,
                "result_count": 1,
                "path": ["v3", "content_generation", "check_grammar", "live"],
                "data": {},
                "result": [{
                    "input_tokens": 8,
                    "output_tokens": 0,
                    "new_tokens": 0,
                    "initial_text": "This are a test.",
                    "language_code": "en-US",
                    "items_count": 1,
                    "items": [{
                        "type": "content_generation_grammar",
                        "message": "The verb does not agree with the subject.",
                        "description": "Subject-verb agreement error",
                        "suggestions": ["is"],
                        "offset": 5,
                        "length": 3,
                        "rule_id": "SUBJECT_VERB_AGREEMENT",
                        "rule_description": "Subject and verb agreement",
                        "rule_issue_type": "grammar",
                        "rule_category_id": "GRAMMAR",
                        "rule_category_name": "Grammar"
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentGenerationApiCheckGrammar> =
            serde_json::from_str(json).expect("check_grammar should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.language_code.as_deref(), Some("en-US"));
        assert_eq!(result.items_count, Some(1));
        let item = &result.items.as_ref().unwrap()[0];
        assert_eq!(item.offset, Some(5));
        assert_eq!(item.suggestions.as_ref().unwrap(), &["is"]);
        assert_eq!(item.rule_category_name.as_deref(), Some("Grammar"));
    }

    #[test]
    fn text_summary_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.5 sec.",
            "cost": 0.0001,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "44444444-4444-4444-4444-444444444444",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.4 sec.",
                "cost": 0.0001,
                "result_count": 1,
                "path": ["v3", "content_generation", "text_summary", "live"],
                "data": {},
                "result": [{
                    "sentences": 3,
                    "paragraphs": 1,
                    "words": 42,
                    "characters_without_spaces": 210,
                    "characters_with_spaces": 251,
                    "words_per_sentence": 14.0,
                    "characters_per_word": 5.0,
                    "vocabulary_density": 0.8,
                    "keyword_density": {"seo": 3, "content": 2},
                    "automated_readability_index": 8.5,
                    "coleman_liau_index": 9.1,
                    "flesch_kincaid_grade_level": 7.4,
                    "smog_readability_index": 8.0,
                    "spelling_errors": 0,
                    "grammar_errors": 1
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentGenerationApiTextSummary> =
            serde_json::from_str(json).expect("text_summary should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.words, Some(42));
        assert_eq!(result.words_per_sentence, Some(14.0));
        assert_eq!(result.flesch_kincaid_grade_level, Some(7.4));
        assert_eq!(
            result.keyword_density.as_ref().unwrap().get("seo"),
            Some(&3)
        );
        assert_eq!(result.grammar_errors, Some(1));
    }

    #[test]
    fn paraphrase_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.2 sec.",
            "cost": 0.0002,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "55555555-5555-5555-5555-555555555555",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "1.1 sec.",
                "cost": 0.0002,
                "result_count": 1,
                "path": ["v3", "content_generation", "paraphrase", "live"],
                "data": {},
                "result": [{
                    "input_tokens": 20,
                    "output_tokens": 22,
                    "new_tokens": 22,
                    "generated_text": "A rephrased version of the sentence."
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentGenerationApiParaphrase> =
            serde_json::from_str(json).expect("paraphrase should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.new_tokens, Some(22));
        assert_eq!(
            result.generated_text.as_deref(),
            Some("A rephrased version of the sentence.")
        );
    }
}

/// Live integration tests. They hit the DataForSEO API and are ignored by default;
/// run with `cargo test --test content_generation -- --ignored` and a `.env`
/// holding `ID`/`PASSWORD`.
#[cfg(test)]
mod live {
    use crate::client;
    use data_for_seo::{ContentGenerationApiGenerateTextPost, ContentGenerationApiTextSummaryPost};

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn generate_text() {
        let client = client();
        let request = ContentGenerationApiGenerateTextPost::new("apple", 50);
        let res = client
            .content_generation()
            .generate_text(vec![request])
            .await
            .unwrap();
        for result in res.results() {
            println!("generated_text: {:?}", result.generated_text);
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn text_summary() {
        let client = client();
        let mut request = ContentGenerationApiTextSummaryPost::new(
            "DataForSEO provides comprehensive SEO data through a single API.",
        );
        request.language_code = Some("en".to_string());
        let res = client
            .content_generation()
            .text_summary(vec![request])
            .await
            .unwrap();
        for result in res.results() {
            println!("words: {:?}", result.words);
            println!("sentences: {:?}", result.sentences);
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn text_summary_languages() {
        let client = client();
        let res = client
            .content_generation()
            .text_summary_languages()
            .await
            .unwrap();
        for result in res.results() {
            println!("{:?} {:?}", result.language_name, result.language_code);
        }
    }
}
