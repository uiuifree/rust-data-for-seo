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
        DataForSeoApiResponseData, DataForSeoLabsAmazonProductKeywordIntersectionItem,
        DataForSeoLabsAppCompetitorItem, DataForSeoLabsCompetitorItem,
        DataForSeoLabsDomainMetricsItem, DataForSeoLabsHistoricalTrafficEstimationItem,
        DataForSeoLabsIdListItem, DataForSeoLabsKeywordData, DataForSeoLabsKeywordDifficultyItem,
        DataForSeoLabsRankedKeywordItem, DataForSeoLabsResult, DataForSeoLabsSearchIntentItem,
        DataForSeoLabsSerpCompetitorItem,
    };

    #[test]
    fn ranked_keywords() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.5 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010101",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.4 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "ranked_keywords", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "target": "example.com",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1200,
                    "items_count": 1,
                    "metrics": {
                        "organic": { "pos_1": 3, "count": 42, "etv": 1234.5, "is_up": 2 }
                    },
                    "items": [{
                        "se_type": "google",
                        "ranked_serp_element": {
                            "se_type": "google",
                            "serp_item": {
                                "se_type": "google",
                                "type": "organic",
                                "rank_group": 1,
                                "rank_absolute": 1,
                                "domain": "example.com",
                                "title": "Example",
                                "url": "https://example.com/",
                                "etv": 500.0,
                                "rank_changes": {
                                    "previous_rank_absolute": 2,
                                    "is_new": false,
                                    "is_up": true,
                                    "is_down": false
                                },
                                "backlinks_info": {
                                    "referring_domains": 10,
                                    "backlinks": 55,
                                    "dofollow": 40
                                },
                                "rank_info": { "page_rank": 120, "main_domain_rank": 300 }
                            },
                            "check_url": "https://www.google.com/search?q=example",
                            "last_updated_time": "2025-01-01 00:00:00 +00:00"
                        },
                        "keyword_data": {
                            "se_type": "google",
                            "keyword": "example",
                            "location_code": 2840,
                            "language_code": "en",
                            "keyword_info": {
                                "se_type": "google",
                                "competition": 0.5,
                                "competition_level": "MEDIUM",
                                "cpc": 1.2,
                                "search_volume": 8100,
                                "monthly_searches": [
                                    { "year": 2024, "month": 12, "search_volume": 8100 }
                                ],
                                "search_volume_trend": { "monthly": 0, "quarterly": -5, "yearly": 10 }
                            },
                            "keyword_properties": {
                                "se_type": "google",
                                "keyword_difficulty": 42,
                                "detected_language": "en",
                                "is_another_language": false
                            }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsRankedKeywordItem>,
        > = serde_json::from_str(json).expect("ranked_keywords should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.target.as_deref(), Some("example.com"));
        assert_eq!(result.total_count, Some(1200));
        assert_eq!(
            result
                .metrics
                .as_ref()
                .unwrap()
                .organic
                .as_ref()
                .unwrap()
                .count,
            Some(42)
        );
        let item = &result.items()[0];
        let serp = item
            .ranked_serp_element
            .as_ref()
            .unwrap()
            .serp_item
            .as_ref()
            .unwrap();
        assert_eq!(serp.item_type.as_deref(), Some("organic"));
        assert_eq!(serp.rank_absolute, Some(1));
        assert_eq!(serp.rank_changes.as_ref().unwrap().is_up, Some(true));
        assert_eq!(serp.rank_info.as_ref().unwrap().page_rank, Some(120));
        let kw = item.keyword_data.as_ref().unwrap();
        assert_eq!(kw.keyword.as_deref(), Some("example"));
        assert_eq!(kw.keyword_info.as_ref().unwrap().search_volume, Some(8100));
        assert_eq!(
            kw.keyword_properties.as_ref().unwrap().keyword_difficulty,
            Some(42)
        );
    }

    #[test]
    fn keyword_ideas() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.01,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010102",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.2 sec.",
                "cost": 0.01,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "keyword_ideas", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 500,
                    "items_count": 1,
                    "offset": 0,
                    "offset_token": "abc",
                    "items": [{
                        "se_type": "google",
                        "keyword": "seo tools",
                        "location_code": 2840,
                        "language_code": "en",
                        "keyword_info": {
                            "se_type": "google",
                            "competition": 0.7,
                            "cpc": 5.5,
                            "search_volume": 40500,
                            "categories": [10005, 13418]
                        },
                        "keyword_properties": {
                            "se_type": "google",
                            "keyword_difficulty": 78,
                            "words_count": 2
                        },
                        "serp_info": {
                            "se_type": "google",
                            "serp_item_types": ["organic", "paid"],
                            "se_results_count": 123000000
                        },
                        "avg_backlinks_info": {
                            "se_type": "google",
                            "backlinks": 250.5,
                            "referring_domains": 42.0
                        },
                        "search_intent_info": {
                            "se_type": "google",
                            "main_intent": "commercial",
                            "foreign_intent": ["informational"]
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<DataForSeoLabsResult<DataForSeoLabsKeywordData>> =
            serde_json::from_str(json).expect("keyword_ideas should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.offset_token.as_deref(), Some("abc"));
        let kw = &result.items()[0];
        assert_eq!(kw.keyword.as_deref(), Some("seo tools"));
        assert_eq!(kw.keyword_info.as_ref().unwrap().search_volume, Some(40500));
        assert_eq!(
            kw.keyword_info
                .as_ref()
                .unwrap()
                .categories
                .as_ref()
                .unwrap()[0],
            10005
        );
        assert_eq!(kw.keyword_properties.as_ref().unwrap().words_count, Some(2));
        assert_eq!(
            kw.serp_info.as_ref().unwrap().se_results_count,
            Some(123000000)
        );
        assert_eq!(
            kw.search_intent_info
                .as_ref()
                .unwrap()
                .main_intent
                .as_deref(),
            Some("commercial")
        );
    }

    #[test]
    fn bulk_keyword_difficulty() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.01,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010103",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.05 sec.",
                "cost": 0.01,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "bulk_keyword_difficulty", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 2,
                    "items_count": 2,
                    "items": [
                        { "se_type": "google", "keyword": "seo", "keyword_difficulty": 91 },
                        { "se_type": "google", "keyword": "local seo", "keyword_difficulty": 55 }
                    ]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsKeywordDifficultyItem>,
        > = serde_json::from_str(json).expect("bulk_keyword_difficulty should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.items().len(), 2);
        assert_eq!(result.items()[0].keyword.as_deref(), Some("seo"));
        assert_eq!(result.items()[0].keyword_difficulty, Some(91));
        assert_eq!(result.items()[1].keyword_difficulty, Some(55));
    }

    #[test]
    fn search_intent() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.01,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010104",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.05 sec.",
                "cost": 0.01,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "search_intent", "live"],
                "data": {},
                "result": [{
                    "language_code": "en",
                    "items_count": 1,
                    "items": [{
                        "keyword": "buy running shoes",
                        "keyword_intent": { "label": "transactional", "probability": 0.98 },
                        "secondary_keyword_intents": [
                            { "label": "commercial", "probability": 0.42 }
                        ]
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsSearchIntentItem>,
        > = serde_json::from_str(json).expect("search_intent should deserialize");
        let item = &parsed.first_result().expect("one result").items()[0];
        assert_eq!(item.keyword.as_deref(), Some("buy running shoes"));
        assert_eq!(
            item.keyword_intent.as_ref().unwrap().label.as_deref(),
            Some("transactional")
        );
        assert_eq!(
            item.secondary_keyword_intents.as_ref().unwrap()[0].probability,
            Some(0.42)
        );
    }

    #[test]
    fn competitors_domain() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.5 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010105",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.4 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "competitors_domain", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "target": "example.com",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "google",
                        "domain": "competitor.com",
                        "avg_position": 12.5,
                        "sum_position": 2500,
                        "intersections": 200,
                        "metrics": {
                            "organic": { "pos_1": 5, "count": 200, "etv": 9999.9 }
                        },
                        "full_domain_metrics": {
                            "organic": { "count": 500 }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<DataForSeoLabsResult<DataForSeoLabsCompetitorItem>> =
            serde_json::from_str(json).expect("competitors_domain should deserialize");
        let item = &parsed.first_result().expect("one result").items()[0];
        assert_eq!(item.domain.as_deref(), Some("competitor.com"));
        assert_eq!(item.intersections, Some(200));
        assert_eq!(
            item.metrics.as_ref().unwrap().organic.as_ref().unwrap().etv,
            Some(9999.9)
        );
        assert_eq!(
            item.full_domain_metrics
                .as_ref()
                .unwrap()
                .organic
                .as_ref()
                .unwrap()
                .count,
            Some(500)
        );
    }

    #[test]
    fn domain_rank_overview() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.3 sec.",
            "cost": 0.01,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010106",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.2 sec.",
                "cost": 0.01,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "domain_rank_overview", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "google",
                        "location_code": 2840,
                        "language_code": "en",
                        "metrics": {
                            "organic": {
                                "pos_1": 10,
                                "pos_2_3": 20,
                                "pos_4_10": 100,
                                "etv": 55000.0,
                                "count": 1500,
                                "estimated_paid_traffic_cost": 12000.5,
                                "is_new": 30,
                                "is_up": 40,
                                "is_down": 15,
                                "is_lost": 5
                            },
                            "paid": { "count": 12 }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsDomainMetricsItem>,
        > = serde_json::from_str(json).expect("domain_rank_overview should deserialize");
        let item = &parsed.first_result().expect("one result").items()[0];
        let organic = item.metrics.as_ref().unwrap().organic.as_ref().unwrap();
        assert_eq!(organic.pos_4_10, Some(100));
        assert_eq!(organic.etv, Some(55000.0));
        assert_eq!(organic.is_new, Some(30));
        assert_eq!(
            item.metrics.as_ref().unwrap().paid.as_ref().unwrap().count,
            Some(12)
        );
    }

    #[test]
    fn serp_competitors() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.4 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010107",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.3 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "serp_competitors", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "seed_keywords": ["seo", "sem"],
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "google",
                        "domain": "competitor.com",
                        "avg_position": 4.5,
                        "median_position": 3.0,
                        "rating": 88,
                        "etv": 12345.6,
                        "keywords_count": 2,
                        "visibility": 0.75,
                        "relevant_serp_items": 2,
                        "keywords_positions": { "seo": [1, 2], "sem": [3] }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsSerpCompetitorItem>,
        > = serde_json::from_str(json).expect("serp_competitors should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(
            result.seed_keywords.as_ref().unwrap(),
            &vec!["seo".to_string(), "sem".to_string()]
        );
        let item = &result.items()[0];
        assert_eq!(item.domain.as_deref(), Some("competitor.com"));
        assert_eq!(item.rating, Some(88));
        assert_eq!(
            item.keywords_positions.as_ref().unwrap().get("seo"),
            Some(&vec![1, 2])
        );
    }

    #[test]
    fn id_list() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010108",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.05 sec.",
                "cost": 0.0,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "id_list"],
                "data": {},
                "result": [{
                    "id": "01230123-0123-0123-0123-012301230123",
                    "datetime_posted": "2025-01-01 00:00:00 +00:00",
                    "datetime_done": "2025-01-01 00:00:05 +00:00",
                    "url": "https://api.dataforseo.com/v3/dataforseo_labs/google/ranked_keywords/live",
                    "status": "ok",
                    "cost": 0.02,
                    "metadata": { "target": "example.com" }
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<DataForSeoLabsIdListItem> =
            serde_json::from_str(json).expect("id_list should deserialize");
        let item = parsed.first_result().expect("one result");
        assert_eq!(item.status.as_deref(), Some("ok"));
        assert_eq!(item.cost, Some(0.02));
        assert_eq!(
            item.metadata.as_ref().unwrap().get("target").unwrap(),
            "example.com"
        );
    }

    #[test]
    fn app_competitors() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.4 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010109",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.3 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "app_competitors", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "app_id": "com.example.app",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "google",
                        "app_id": "com.competitor.app",
                        "avg_position": 6.5,
                        "sum_position": 130,
                        "intersections": 20,
                        "competitor_metrics": {
                            "google_play_organic": {
                                "pos_1": 3,
                                "pos_2_3": 5,
                                "pos_4_10": 12,
                                "pos_11_100": 40,
                                "count": 60,
                                "search_volume": 150000
                            }
                        },
                        "full_metrics": {
                            "google_play_organic": { "count": 200, "search_volume": 500000 }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsAppCompetitorItem>,
        > = serde_json::from_str(json).expect("app_competitors should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.app_id.as_deref(), Some("com.example.app"));
        let item = &result.items()[0];
        assert_eq!(item.app_id.as_deref(), Some("com.competitor.app"));
        assert_eq!(item.intersections, Some(20));
        let cm = item.competitor_metrics.as_ref().unwrap();
        assert_eq!(
            cm.get("google_play_organic").unwrap().search_volume,
            Some(150000)
        );
        assert_eq!(
            item.full_metrics
                .as_ref()
                .unwrap()
                .get("google_play_organic")
                .unwrap()
                .count,
            Some(200)
        );
    }

    #[test]
    fn historical_bulk_traffic_estimation() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.4 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-01010101010a",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.3 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "google", "historical_bulk_traffic_estimation", "live"],
                "data": {},
                "result": [{
                    "se_type": "google",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "google",
                        "target": "example.com",
                        "metrics": {
                            "organic": [
                                { "year": 2024, "month": 11, "etv": 1000.5, "count": 500 },
                                { "year": 2024, "month": 12, "etv": 1200.0, "count": 520 }
                            ],
                            "paid": [
                                { "year": 2024, "month": 12, "etv": 50.0, "count": 3 }
                            ]
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsHistoricalTrafficEstimationItem>,
        > = serde_json::from_str(json)
            .expect("historical_bulk_traffic_estimation should deserialize");
        let item = &parsed.first_result().expect("one result").items()[0];
        assert_eq!(item.target.as_deref(), Some("example.com"));
        let organic = item.metrics.as_ref().unwrap().organic.as_ref().unwrap();
        assert_eq!(organic.len(), 2);
        assert_eq!(organic[1].month, Some(12));
        assert_eq!(organic[1].etv, Some(1200.0));
        assert_eq!(
            item.metrics.as_ref().unwrap().paid.as_ref().unwrap()[0].count,
            Some(3)
        );
    }

    #[test]
    fn amazon_product_keyword_intersections() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.4 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-01010101010b",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.3 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "dataforseo_labs", "amazon", "product_keyword_intersections", "live"],
                "data": {},
                "result": [{
                    "se_type": "amazon",
                    "location_code": 2840,
                    "language_code": "en",
                    "total_count": 1,
                    "items_count": 1,
                    "items": [{
                        "se_type": "amazon",
                        "keyword_data": {
                            "se_type": "amazon",
                            "keyword": "wireless earbuds",
                            "location_code": 2840,
                            "language_code": "en",
                            "keyword_info": {
                                "se_type": "amazon",
                                "search_volume": 74000
                            }
                        },
                        "intersection_result": {
                            "1": {
                                "type": "amazon_serp",
                                "rank_absolute": 3,
                                "asin": "B0AAAA1111",
                                "title": "Product One",
                                "price_from": 29.99,
                                "currency": "USD"
                            },
                            "2": {
                                "type": "amazon_serp",
                                "rank_absolute": 7,
                                "asin": "B0BBBB2222",
                                "title": "Product Two"
                            }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<
            DataForSeoLabsResult<DataForSeoLabsAmazonProductKeywordIntersectionItem>,
        > = serde_json::from_str(json)
            .expect("amazon product_keyword_intersections should deserialize");
        let item = &parsed.first_result().expect("one result").items()[0];
        assert_eq!(
            item.keyword_data.as_ref().unwrap().keyword.as_deref(),
            Some("wireless earbuds")
        );
        let inter = item.intersection_result.as_ref().unwrap();
        assert_eq!(inter.get("1").unwrap().asin.as_deref(), Some("B0AAAA1111"));
        assert_eq!(inter.get("1").unwrap().price_from, Some(29.99));
        assert_eq!(inter.get("2").unwrap().rank_absolute, Some(7));
    }
}

/// Live API smoke tests. Run with credentials via `cargo test --test
/// dataforseo_labs -- --ignored`.
#[cfg(test)]
mod live {
    use super::client;
    use data_for_seo::DataForSeoLabsRankedKeywordsRequest;

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn ranked_keywords_live() {
        let response = client()
            .dataforseo_labs()
            .google()
            .ranked_keywords(vec![{
                let mut req = DataForSeoLabsRankedKeywordsRequest::new("dataforseo.com");
                req.location_code = Some(2840);
                req.language_code = Some("en".to_string());
                req.limit = Some(5);
                req
            }])
            .await
            .expect("ranked_keywords live call");
        assert_eq!(response.status_code, 20000);
        for result in response.results() {
            for item in result.items() {
                println!(
                    "{:?}",
                    item.keyword_data.as_ref().and_then(|k| k.keyword.clone())
                );
            }
        }
    }
}
