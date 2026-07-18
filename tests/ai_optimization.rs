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
        AiOptimizationKeywordDataSearchVolume, AiOptimizationLlmMentionsSearch,
        AiOptimizationLlmResponse, AiOptimizationLlmResponseModel, AiOptimizationLlmScraper,
        DataForSeoApiResponseData,
    };

    #[test]
    fn keywords_search_volume() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.001,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010101",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.05 sec.",
                "cost": 0.001,
                "result_count": 1,
                "path": ["v3", "ai_optimization", "ai_keyword_data", "keywords_search_volume", "live"],
                "data": {},
                "result": [{
                    "location_code": 2840,
                    "language_code": "en",
                    "items_count": 1,
                    "items": [{
                        "keyword": "best crm software",
                        "ai_search_volume": 1200,
                        "ai_monthly_searches": [
                            { "year": 2025, "month": 6, "ai_search_volume": 1100 },
                            { "year": 2025, "month": 7, "ai_search_volume": 1200 }
                        ]
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<AiOptimizationKeywordDataSearchVolume> =
            serde_json::from_str(json).expect("keywords_search_volume should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.location_code, Some(2840));
        let items = result.items.as_ref().expect("items");
        assert_eq!(items[0].keyword.as_deref(), Some("best crm software"));
        assert_eq!(items[0].ai_search_volume, Some(1200));
        assert_eq!(
            items[0].ai_monthly_searches.as_ref().unwrap()[1].ai_search_volume,
            Some(1200)
        );
    }

    #[test]
    fn llm_responses_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "3.5 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "02020202-0202-0202-0202-020202020202",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "3.4 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "ai_optimization", "chat_gpt", "llm_responses", "live"],
                "data": {},
                "result": [{
                    "model_name": "gpt-4.1-mini",
                    "input_tokens": 42,
                    "output_tokens": 128,
                    "reasoning_tokens": 0,
                    "web_search": true,
                    "money_spent": 0.015,
                    "datetime": "2025-07-18 00:00:00 +00:00",
                    "items": [{
                        "type": "message",
                        "sections": [{
                            "type": "text",
                            "text": "DataForSEO provides SEO APIs.",
                            "annotations": [
                                { "title": "DataForSEO", "url": "https://dataforseo.com/" }
                            ]
                        }]
                    }],
                    "fan_out_queries": ["seo api", "rank tracker api"]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<AiOptimizationLlmResponse> =
            serde_json::from_str(json).expect("llm_responses live should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.model_name.as_deref(), Some("gpt-4.1-mini"));
        assert_eq!(result.output_tokens, Some(128));
        assert_eq!(result.web_search, Some(true));
        let item = &result.items.as_ref().unwrap()[0];
        assert_eq!(item.item_type.as_deref(), Some("message"));
        let section = &item.sections.as_ref().unwrap()[0];
        assert_eq!(section.section_type.as_deref(), Some("text"));
        assert_eq!(
            section.annotations.as_ref().unwrap()[0].url.as_deref(),
            Some("https://dataforseo.com/")
        );
    }

    #[test]
    fn llm_mentions_search() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "12.0 sec.",
            "cost": 0.05,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "03030303-0303-0303-0303-030303030303",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "11.5 sec.",
                "cost": 0.05,
                "result_count": 1,
                "path": ["v3", "ai_optimization", "llm_mentions", "search", "live"],
                "data": {},
                "result": [{
                    "total_count": 1,
                    "current_offset": 0,
                    "search_after_token": "tok_123",
                    "items_count": 1,
                    "items": [{
                        "platform": "chat_gpt",
                        "model_name": "gpt-4o",
                        "location_code": 2840,
                        "language_code": "en",
                        "question": "What is the best SEO API?",
                        "answer": "DataForSEO is a leading SEO API provider.",
                        "sources": [{
                            "source_name": "dataforseo.com",
                            "title": "DataForSEO",
                            "domain": "dataforseo.com",
                            "url": "https://dataforseo.com/",
                            "position": 1
                        }],
                        "ai_search_volume": 800,
                        "first_response_at": "2025-07-01 00:00:00 +00:00",
                        "last_response_at": "2025-07-18 00:00:00 +00:00"
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<AiOptimizationLlmMentionsSearch> =
            serde_json::from_str(json).expect("llm_mentions search should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.total_count, Some(1));
        assert_eq!(result.search_after_token.as_deref(), Some("tok_123"));
        let mention = &result.items.as_ref().unwrap()[0];
        assert_eq!(mention.platform.as_deref(), Some("chat_gpt"));
        assert_eq!(mention.ai_search_volume, Some(800));
        assert_eq!(
            mention.sources.as_ref().unwrap()[0].domain.as_deref(),
            Some("dataforseo.com")
        );
    }

    #[test]
    fn llm_scraper_live_advanced() {
        let json = r##"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "8.0 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "04040404-0404-0404-0404-040404040404",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "7.5 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "ai_optimization", "chat_gpt", "llm_scraper", "live", "advanced"],
                "data": {},
                "result": [{
                    "keyword": "best seo api",
                    "location_code": 2840,
                    "language_code": "en",
                    "model": "gpt-4o",
                    "check_url": "https://chatgpt.com/",
                    "datetime": "2025-07-18 00:00:00 +00:00",
                    "markdown": "# Best SEO API\nDataForSEO ...",
                    "item_types": ["chat_gpt_text"],
                    "items_count": 1,
                    "items": [{
                        "type": "chat_gpt_text",
                        "rank_group": 1,
                        "rank_absolute": 1,
                        "markdown": "DataForSEO ..."
                    }]
                }]
            }]
        }"##;

        let parsed: DataForSeoApiResponseData<AiOptimizationLlmScraper> =
            serde_json::from_str(json).expect("llm_scraper live advanced should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.keyword.as_deref(), Some("best seo api"));
        assert_eq!(result.model.as_deref(), Some("gpt-4o"));
        assert_eq!(result.item_types.as_ref().unwrap(), &vec!["chat_gpt_text"]);
        assert_eq!(result.items.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn llm_responses_models() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.01 sec.",
            "cost": 0.0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "05050505-0505-0505-0505-050505050505",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.01 sec.",
                "cost": 0.0,
                "result_count": 2,
                "path": ["v3", "ai_optimization", "chat_gpt", "llm_responses", "models"],
                "data": {},
                "result": [
                    { "model_name": "gpt-4.1-mini", "reasoning": false, "web_search_supported": true, "task_post_supported": true },
                    { "model_name": "o3", "reasoning": true, "web_search_supported": true, "task_post_supported": true }
                ]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<AiOptimizationLlmResponseModel> =
            serde_json::from_str(json).expect("models should deserialize");
        let models: Vec<_> = parsed.results().collect();
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].model_name.as_deref(), Some("gpt-4.1-mini"));
        assert_eq!(models[1].reasoning, Some(true));
    }
}

#[cfg(test)]
mod live {
    use crate::client;
    use data_for_seo::{
        AiOptimizationKeywordDataSearchVolumePost, AiOptimizationLlmMentionsSearchPost,
        AiOptimizationLlmMentionsTarget, AiOptimizationLlmResponsesPost,
    };

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn keywords_search_volume() {
        let client = client();
        let request =
            AiOptimizationKeywordDataSearchVolumePost::new(vec!["best crm software".to_string()]);
        let res = client
            .ai_optimization()
            .ai_keyword_data()
            .keywords_search_volume(vec![request])
            .await
            .unwrap();
        println!("{res:?}");
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn chat_gpt_models() {
        let client = client();
        let res = client
            .ai_optimization()
            .chat_gpt()
            .llm_responses()
            .models()
            .await
            .unwrap();
        println!("{res:?}");
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn chat_gpt_live() {
        let client = client();
        let request = AiOptimizationLlmResponsesPost::new("What is DataForSEO?", "gpt-4.1-mini");
        let res = client
            .ai_optimization()
            .chat_gpt()
            .llm_responses()
            .live(vec![request])
            .await
            .unwrap();
        println!("{res:?}");
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn llm_mentions_search() {
        let client = client();
        let request = AiOptimizationLlmMentionsSearchPost::new(vec![
            AiOptimizationLlmMentionsTarget::domain("dataforseo.com"),
        ]);
        let res = client
            .ai_optimization()
            .llm_mentions()
            .search(vec![request])
            .await
            .unwrap();
        println!("{res:?}");
    }
}
