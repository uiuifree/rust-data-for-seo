use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

/// Live integration tests. They hit the DataForSEO API and are ignored by default;
/// run with `cargo test --test backlinks -- --ignored` and a `.env` holding `ID`/`PASSWORD`.
#[cfg(test)]
mod live {
    use crate::client;
    use data_for_seo::{
        BacklinksApiAnchorPost, BacklinksApiBacklinksPost, BacklinksApiSummaryPost,
    };

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn index() {
        let client = client();
        let res = client.backlinks().index().await.unwrap();
        for result in res.results() {
            println!("total_backlinks: {:?}", result.total_backlinks);
            println!("total_pages: {:?}", result.total_pages);
            println!("total_domains: {:?}", result.total_domains);
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn summary() {
        let client = client();
        let request = BacklinksApiSummaryPost::new("https://example.com/");
        let res = client.backlinks().summary(vec![request]).await.unwrap();
        for result in res.results() {
            println!("res: {result:?}");
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn backlinks() {
        let client = client();
        let mut request = BacklinksApiBacklinksPost::new("https://example.com/");
        request.order_by = Some(vec!["rank,desc".to_string()]);
        request.limit = Some(100);
        let res = client.backlinks().backlinks(vec![request]).await.unwrap();
        for result in res.results() {
            println!("mode: {:?}", result.mode);
            println!("items_count: {:?}", result.items_count);
            for item in result.items.iter().flatten() {
                println!("res: {item:?}");
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn anchor() {
        let client = client();
        let request = BacklinksApiAnchorPost::new("https://example.com/");
        let res = client.backlinks().anchor(vec![request]).await.unwrap();
        for result in res.results() {
            for item in result.items.iter().flatten() {
                println!("res: {item:?}");
            }
        }
    }
}

/// Deserialization tests based on the documented response structure.
/// These do not require credentials and always run.
#[cfg(test)]
mod deserialize {
    use data_for_seo::entity::{
        BacklinksApiBacklinks, BacklinksApiBulkSpamScore, BacklinksApiDomainIntersection,
        BacklinksApiSummary, DataForSeoApiResponseData,
    };

    #[test]
    fn summary_response() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "task-1",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.1 sec.",
                "cost": 0.02,
                "result_count": 1,
                "path": ["v3", "backlinks", "summary", "live"],
                "data": {"api": "backlinks"},
                "result": [{
                    "target": "example.com",
                    "first_seen": "2019-01-01 00:00:00 +00:00",
                    "rank": 250,
                    "backlinks": 1024,
                    "backlinks_spam_score": 12,
                    "referring_domains": 42,
                    "info": {"server": "nginx", "cms": "WordPress", "is_ip": false, "target_spam_score": 5},
                    "referring_links_types": {"anchor": 900, "image": 124}
                }]
            }]
        }"#;

        let response: DataForSeoApiResponseData<BacklinksApiSummary> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.status_code, 20000);
        let result = response.first_result().unwrap();
        assert_eq!(result.target.as_deref(), Some("example.com"));
        assert_eq!(result.backlinks, Some(1024));
        assert_eq!(
            result.info.as_ref().and_then(|i| i.cms.as_deref()),
            Some("WordPress")
        );
        assert_eq!(
            result
                .referring_links_types
                .as_ref()
                .and_then(|m| m.get("anchor").copied()),
            Some(900)
        );
    }

    #[test]
    fn backlinks_response_indirect_link_path() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "task-1",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.1 sec.",
                "cost": 0.02,
                "path": ["v3", "backlinks", "backlinks", "live"],
                "data": {},
                "result": [{
                    "target": "example.com",
                    "mode": "as_is",
                    "total_count": 2,
                    "items_count": 1,
                    "search_after_token": "abc",
                    "items": [{
                        "type": "backlink",
                        "domain_from": "ref.com",
                        "url_from": "https://ref.com/a",
                        "url_to": "https://example.com/",
                        "rank": 100,
                        "dofollow": true,
                        "anchor": "click here",
                        "is_indirect_link": true,
                        "indirect_link_path": [
                            {"type": "redirect", "status_code": 301, "url": "https://ref.com/r"},
                            {"type": "link", "status_code": 200, "url": "https://example.com/"}
                        ]
                    }]
                }]
            }]
        }"#;

        let response: DataForSeoApiResponseData<BacklinksApiBacklinks> =
            serde_json::from_str(json).unwrap();
        let result = response.first_result().unwrap();
        assert_eq!(result.search_after_token.as_deref(), Some("abc"));
        let items = result.items.as_ref().unwrap();
        let item = &items[0];
        assert_eq!(item.type_of_element.as_deref(), Some("backlink"));
        let path = item.indirect_link_path.as_ref().unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].status_code, Some(301));
        assert_eq!(path[0].type_of_element.as_deref(), Some("redirect"));
    }

    #[test]
    fn bulk_spam_score_response() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "task-1",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.1 sec.",
                "cost": 0.02,
                "path": ["v3", "backlinks", "bulk_spam_score", "live"],
                "data": {},
                "result": [{
                    "items_count": 2,
                    "items": [
                        {"type": "backlinks_bulk_spam_score", "target": "example.com", "spam_score": 17},
                        {"type": "backlinks_bulk_spam_score", "target": "foo.com", "spam_score": 3}
                    ]
                }]
            }]
        }"#;

        let response: DataForSeoApiResponseData<BacklinksApiBulkSpamScore> =
            serde_json::from_str(json).unwrap();
        let result = response.first_result().unwrap();
        let items = result.items.as_ref().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].target.as_deref(), Some("foo.com"));
        assert_eq!(items[1].spam_score, Some(3));
    }

    #[test]
    fn domain_intersection_response_keyed_by_target() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0.02,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [{
                "id": "task-1",
                "status_code": 20000,
                "status_message": "Ok.",
                "time": "0.1 sec.",
                "cost": 0.02,
                "path": ["v3", "backlinks", "domain_intersection", "live"],
                "data": {},
                "result": [{
                    "targets": {"1": "first.com", "2": "second.com"},
                    "total_count": 5,
                    "items_count": 1,
                    "items": [{
                        "domain_intersection": {
                            "1": {"type": "backlinks_domain_intersection", "target": "ref.com", "rank": 88, "backlinks": 10},
                            "2": {"type": "backlinks_domain_intersection", "target": "ref.com", "rank": 90, "backlinks": 12}
                        },
                        "summary": {"intersections_count": 2}
                    }]
                }]
            }]
        }"#;

        let response: DataForSeoApiResponseData<BacklinksApiDomainIntersection> =
            serde_json::from_str(json).unwrap();
        let result = response.first_result().unwrap();
        let item = &result.items.as_ref().unwrap()[0];
        let intersection = item.domain_intersection.as_ref().unwrap();
        let first = intersection.get("1").unwrap();
        assert_eq!(first.target.as_deref(), Some("ref.com"));
        assert_eq!(first.rank, Some(88));
        assert_eq!(
            item.summary.as_ref().and_then(|s| s.intersections_count),
            Some(2)
        );
    }
}
