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
        ContentAnalysisApiCategory, ContentAnalysisApiRatingDistribution, ContentAnalysisApiSearch,
        ContentAnalysisApiSentimentAnalysis, ContentAnalysisApiSummary, DataForSeoApiResponseData,
    };

    #[test]
    fn search_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.5 sec.",
            "cost": 0.05,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "01010101-0101-0101-0101-010101010101",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "1.4 sec.",
                "cost": 0.05,
                "result_count": 1,
                "path": ["v3", "content_analysis", "search", "live"],
                "data": {},
                "result": [{
                    "offset_token": "token_abc",
                    "total_count": 120,
                    "items_count": 1,
                    "items": [{
                        "type": "content_analysis_search",
                        "url": "https://example.com/review",
                        "domain": "example.com",
                        "main_domain": "example.com",
                        "url_rank": 320,
                        "spam_score": 5,
                        "domain_rank": 410,
                        "fetch_time": "2024-01-01 00:00:00 +00:00",
                        "country": "US",
                        "language": "en",
                        "score": 12.5,
                        "page_category": [10001, 10002],
                        "page_types": ["blogs"],
                        "ratings": [{
                            "name": "aggregateRating",
                            "rating_value": "4.5",
                            "rating_count": "20",
                            "max_rating_value": "5",
                            "relative_rating": "0.9"
                        }],
                        "social_metrics": [{"type": "facebook", "like_count": 42}],
                        "content_info": {
                            "content_type": "page_content",
                            "title": "Great product",
                            "main_title": "Reviews",
                            "snippet": "This is a great product.",
                            "snippet_length": 24,
                            "language": "en",
                            "sentiment_connotations": {"happiness": 0.8, "love": 0.5},
                            "connotation_types": {"positive": 0.9, "negative": 0.1, "neutral": 0.0},
                            "text_category": [10001],
                            "content_quality_score": 7,
                            "semantic_location": "article",
                            "rating": {
                                "name": "aggregateRating",
                                "rating_value": "4.5",
                                "rating_count": "20",
                                "max_rating_value": "5",
                                "relative_rating": "0.9"
                            }
                        }
                    }]
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentAnalysisApiSearch> =
            serde_json::from_str(json).expect("search should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.total_count, Some(120));
        assert_eq!(result.offset_token.as_deref(), Some("token_abc"));
        let item = &result.items.as_ref().unwrap()[0];
        assert_eq!(item.domain.as_deref(), Some("example.com"));
        assert_eq!(item.spam_score, Some(5));
        assert_eq!(item.score, Some(12.5));
        assert_eq!(item.page_types.as_ref().unwrap(), &["blogs"]);
        let rating = &item.ratings.as_ref().unwrap()[0];
        assert_eq!(rating.rating_value.as_deref(), Some("4.5"));
        let content = item.content_info.as_ref().unwrap();
        assert_eq!(content.title.as_deref(), Some("Great product"));
        assert_eq!(
            content
                .sentiment_connotations
                .as_ref()
                .unwrap()
                .get("happiness"),
            Some(&0.8)
        );
        assert_eq!(
            content.rating.as_ref().unwrap().rating_count.as_deref(),
            Some("20")
        );
    }

    #[test]
    fn summary_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.0 sec.",
            "cost": 0.05,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "02020202-0202-0202-0202-020202020202",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.9 sec.",
                "cost": 0.05,
                "result_count": 1,
                "path": ["v3", "content_analysis", "summary", "live"],
                "data": {},
                "result": [{
                    "type": "content_analysis_summary",
                    "total_count": 500,
                    "rank": 640,
                    "top_domains": [{"domain": "example.com", "count": 12}],
                    "sentiment_connotations": {"anger": 3, "happiness": 10},
                    "connotation_types": {"positive": 40, "negative": 5, "neutral": 20},
                    "text_categories": [{"category": [10001, 10002], "count": 8}],
                    "page_categories": [{"category": [20001], "count": 4}],
                    "page_types": {"blogs": 30, "news": 12},
                    "countries": {"US": 40, "GB": 5},
                    "languages": {"en": 60}
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentAnalysisApiSummary> =
            serde_json::from_str(json).expect("summary should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(
            result.element_type.as_deref(),
            Some("content_analysis_summary")
        );
        assert_eq!(result.total_count, Some(500));
        assert_eq!(result.top_domains.as_ref().unwrap()[0].count, Some(12));
        assert_eq!(result.page_types.as_ref().unwrap().get("blogs"), Some(&30));
        assert_eq!(
            result.text_categories.as_ref().unwrap()[0]
                .category
                .as_ref()
                .unwrap(),
            &[10001, 10002]
        );
        assert_eq!(result.countries.as_ref().unwrap().get("US"), Some(&40));
    }

    #[test]
    fn sentiment_analysis_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.0 sec.",
            "cost": 0.05,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "03030303-0303-0303-0303-030303030303",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.9 sec.",
                "cost": 0.05,
                "result_count": 1,
                "path": ["v3", "content_analysis", "sentiment_analysis", "live"],
                "data": {},
                "result": [{
                    "type": "content_analysis_sentiment_analysis",
                    "positive_connotation_distribution": {
                        "positive": {"type": "content_analysis_summary", "total_count": 300, "rank": 400},
                        "negative": {"type": "content_analysis_summary", "total_count": 50, "rank": 90},
                        "neutral": {"type": "content_analysis_summary", "total_count": 150, "rank": 200}
                    },
                    "sentiment_connotation_distribution": {
                        "anger": {"type": "content_analysis_summary", "total_count": 10, "rank": 20},
                        "happiness": {"type": "content_analysis_summary", "total_count": 120, "rank": 180},
                        "love": {"type": "content_analysis_summary", "total_count": 40, "rank": 70},
                        "sadness": {"type": "content_analysis_summary", "total_count": 15, "rank": 25},
                        "share": {"type": "content_analysis_summary", "total_count": 8, "rank": 12},
                        "fun": {"type": "content_analysis_summary", "total_count": 30, "rank": 55}
                    }
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentAnalysisApiSentimentAnalysis> =
            serde_json::from_str(json).expect("sentiment_analysis should deserialize");
        let result = parsed.first_result().expect("one result");
        let positive = result
            .positive_connotation_distribution
            .as_ref()
            .unwrap()
            .positive
            .as_ref()
            .unwrap();
        assert_eq!(positive.total_count, Some(300));
        let happiness = result
            .sentiment_connotation_distribution
            .as_ref()
            .unwrap()
            .happiness
            .as_ref()
            .unwrap();
        assert_eq!(happiness.rank, Some(180));
    }

    #[test]
    fn rating_distribution_live() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "1.0 sec.",
            "cost": 0.05,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "04040404-0404-0404-0404-040404040404",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.9 sec.",
                "cost": 0.05,
                "result_count": 1,
                "path": ["v3", "content_analysis", "rating_distribution", "live"],
                "data": {},
                "result": [{
                    "type": "content_analysis_rating_distribution",
                    "min": 4.0,
                    "max": 5.0,
                    "metrics": {
                        "type": "content_analysis_summary",
                        "total_count": 75,
                        "rank": 120,
                        "page_types": {"ecommerce": 75}
                    }
                }]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentAnalysisApiRatingDistribution> =
            serde_json::from_str(json).expect("rating_distribution should deserialize");
        let result = parsed.first_result().expect("one result");
        assert_eq!(result.min, Some(4.0));
        assert_eq!(result.max, Some(5.0));
        let metrics = result.metrics.as_ref().unwrap();
        assert_eq!(metrics.total_count, Some(75));
        assert_eq!(
            metrics.page_types.as_ref().unwrap().get("ecommerce"),
            Some(&75)
        );
    }

    #[test]
    fn categories() {
        let json = r#"{
            "version": "0.1.20250101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "05050505-0505-0505-0505-050505050505",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.05 sec.",
                "cost": 0.0,
                "result_count": 2,
                "path": ["v3", "content_analysis", "categories"],
                "data": {},
                "result": [
                    {"category_code": 10001, "category_name": "Arts & Entertainment", "category_code_parent": null},
                    {"category_code": 10002, "category_name": "Celebrities & Entertainment News", "category_code_parent": 10001}
                ]
            }]
        }"#;

        let parsed: DataForSeoApiResponseData<ContentAnalysisApiCategory> =
            serde_json::from_str(json).expect("categories should deserialize");
        let results: Vec<_> = parsed.results().collect();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].category_code, Some(10001));
        assert_eq!(results[1].category_code_parent, Some(10001));
    }
}

/// Live integration tests. They hit the DataForSEO API and are ignored by default;
/// run with `cargo test --test content_analysis -- --ignored` and a `.env`
/// holding `ID`/`PASSWORD`.
#[cfg(test)]
mod live {
    use crate::client;
    use data_for_seo::ContentAnalysisApiSummaryPost;

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn summary() {
        let client = client();
        let request = ContentAnalysisApiSummaryPost::new("iphone");
        let res = client
            .content_analysis()
            .summary(vec![request])
            .await
            .unwrap();
        for result in res.results() {
            println!("total_count: {:?}", result.total_count);
            println!("rank: {:?}", result.rank);
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn categories() {
        let client = client();
        let res = client.content_analysis().categories().await.unwrap();
        for result in res.results() {
            println!("{:?} {:?}", result.category_code, result.category_name);
        }
    }
}
