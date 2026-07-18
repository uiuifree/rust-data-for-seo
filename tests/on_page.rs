use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

/// Live integration tests. Ignored by default; require valid DataForSEO credentials.
#[cfg(test)]
mod on_page_live {
    use crate::client;
    use data_for_seo::{
        OnPageApiKeywordDensityPost, OnPageApiPagesByResourcePost, OnPageApiPagesPost,
        OnPageApiRawHtmlPost, OnPageApiTaskPost, OnPageApiWaterfallPost,
    };

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn post() {
        let client = client();
        let mut request = OnPageApiTaskPost::new("example.com".to_string(), 1);
        request.start_url = Some("https://example.com/".to_string());
        request.validate_micromarkup = Some(true);
        request.store_raw_html = Some(true);
        let response = client.on_page().task_post(vec![request]).await.unwrap();
        for task in response.tasks() {
            println!("{task:?}");
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn page_by_resource() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let request = OnPageApiPagesByResourcePost::new(id, url);
        let response = client
            .on_page()
            .pages_by_resource(vec![request])
            .await
            .unwrap();
        for result in response.results() {
            println!("{result:?}");
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn pages() {
        let client = client();
        let id = "xxx".to_string();

        let request = OnPageApiPagesPost::new(id);
        let response = client.on_page().pages(vec![request]).await.unwrap();
        for result in response.results() {
            println!("{result:?}");
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn waterfall() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let request = OnPageApiWaterfallPost::new(id, url);
        let response = client.on_page().waterfall(vec![request]).await.unwrap();
        for result in response.results() {
            println!("{result:?}");
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn keyword_density() {
        let client = client();
        let id = "xxx".to_string();
        let mut request = OnPageApiKeywordDensityPost::new(id, 1);
        request.order_by = Some(vec!["frequency,desc".to_string()]);
        request.filters = Some(vec![vec![
            "frequency".to_string(),
            ">=".to_string(),
            "2".to_string(),
        ]]);
        let response = client
            .on_page()
            .keyword_density(vec![request])
            .await
            .unwrap();
        for result in response.results() {
            for item in result.items.iter().flatten() {
                println!(
                    "{} {} {}",
                    item.keyword.clone().unwrap_or_default(),
                    item.density.unwrap_or_default(),
                    item.frequency.unwrap_or_default()
                );
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires DataForSEO credentials"]
    async fn raw_html() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let request = OnPageApiRawHtmlPost::new(id, url);
        let response = client.on_page().raw_html(vec![request]).await.unwrap();
        for result in response.results() {
            println!("{result:?}");
        }
    }
}

/// Deserialization tests against documented sample response structures.
/// These run without credentials.
#[cfg(test)]
mod on_page_deserialize {
    use data_for_seo::entity::{
        DataForSeoApiResponseData, OnPageDataApiKeywordDensity, OnPageDataApiLinks,
        OnPageDataApiNonIndexable, OnPageDataApiSummary,
    };

    /// <https://docs.dataforseo.com/v3/on_page/summary/>
    #[test]
    fn deserialize_summary() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [
                {
                    "id": "01011234-1234-0123-0000-abcdef012345",
                    "status_code": 20000,
                    "status_message": "Ok.",
                    "time": "0.05 sec.",
                    "cost": 0,
                    "result_count": 1,
                    "path": ["v3", "on_page", "summary"],
                    "data": {"api": "on_page", "function": "summary"},
                    "result": [
                        {
                            "crawl_progress": "finished",
                            "crawl_status": {
                                "max_crawl_pages": 100,
                                "pages_in_queue": 0,
                                "pages_crawled": 10
                            },
                            "crawl_stop_reason": null,
                            "domain_info": {
                                "name": "example.com",
                                "cms": null,
                                "ip": "93.184.216.34",
                                "server": "ECS",
                                "crawl_start": "2021-01-01 00:00:00 +00:00",
                                "crawl_end": "2021-01-01 00:05:00 +00:00",
                                "ssl_info": {
                                    "valid_certificate": true,
                                    "certificate_issuer": "DigiCert Inc"
                                },
                                "checks": {"sitemap": true, "robots_txt": true},
                                "total_pages": 10,
                                "page_not_found_status_code": 404,
                                "canonicalization_status_code": 200,
                                "main_domain": "example.com"
                            },
                            "page_metrics": {
                                "links_external": 5,
                                "links_internal": 20,
                                "duplicate_title": 0,
                                "duplicate_description": 0,
                                "duplicate_content": 0,
                                "broken_links": 1,
                                "broken_resources": 0,
                                "links_relation_conflict": 0,
                                "redirect_loop": 0,
                                "onpage_score": 92.5,
                                "non_indexable": 2,
                                "checks": {"canonical": 5, "no_title": 0}
                            }
                        }
                    ]
                }
            ]
        }"#;

        let parsed: DataForSeoApiResponseData<OnPageDataApiSummary> =
            serde_json::from_str(json).expect("summary response should deserialize");
        let result = parsed.first_result().expect("one summary result");
        assert_eq!(result.crawl_progress.as_deref(), Some("finished"));
        let domain_info = result.domain_info.as_ref().unwrap();
        assert_eq!(domain_info.name.as_deref(), Some("example.com"));
        assert_eq!(domain_info.total_pages, Some(10));
        let page_metrics = result.page_metrics.as_ref().unwrap();
        assert_eq!(page_metrics.onpage_score, Some(92.5));
        assert_eq!(page_metrics.broken_links, Some(1));
    }

    /// <https://docs.dataforseo.com/v3/on_page/keyword_density/>
    #[test]
    fn deserialize_keyword_density() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [
                {
                    "id": "01011234-1234-0123-0000-abcdef012345",
                    "status_code": 20000,
                    "status_message": "Ok.",
                    "time": "0.05 sec.",
                    "cost": 0,
                    "result_count": 1,
                    "path": ["v3", "on_page", "keyword_density"],
                    "data": {"api": "on_page", "function": "keyword_density"},
                    "result": [
                        {
                            "crawl_progress": "finished",
                            "crawl_status": {
                                "max_crawl_pages": 100,
                                "pages_in_queue": 0,
                                "pages_crawled": 10
                            },
                            "items_count": 2,
                            "items": [
                                {"keyword": "seo", "frequency": 10, "density": 0.05},
                                {"keyword": "tool", "frequency": 5, "density": 0.02}
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let parsed: DataForSeoApiResponseData<OnPageDataApiKeywordDensity> =
            serde_json::from_str(json).expect("keyword_density response should deserialize");
        let result = parsed.first_result().expect("one keyword_density result");
        assert_eq!(result.items_count, Some(2));
        let items = result.items.as_ref().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].keyword.as_deref(), Some("seo"));
        assert_eq!(items[0].frequency, Some(10));
        assert_eq!(items[1].density, Some(0.02));
    }

    /// <https://docs.dataforseo.com/v3/on_page/links/>
    #[test]
    fn deserialize_links() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [
                {
                    "id": "01011234-1234-0123-0000-abcdef012345",
                    "status_code": 20000,
                    "status_message": "Ok.",
                    "time": "0.05 sec.",
                    "cost": 0,
                    "result_count": 1,
                    "path": ["v3", "on_page", "links"],
                    "data": {"api": "on_page", "function": "links"},
                    "result": [
                        {
                            "crawl_progress": "finished",
                            "crawl_status": {
                                "max_crawl_pages": 100,
                                "pages_in_queue": 0,
                                "pages_crawled": 10
                            },
                            "total_items_count": 2,
                            "items_count": 1,
                            "items": [
                                {
                                    "type": "anchor",
                                    "domain_from": "example.com",
                                    "domain_to": "example.com",
                                    "page_from": "https://example.com/",
                                    "page_to": "https://example.com/about",
                                    "link_from": "https://example.com/",
                                    "link_to": "https://example.com/about",
                                    "dofollow": true,
                                    "page_from_scheme": "HTTPS",
                                    "page_to_scheme": "HTTPS",
                                    "direction": "internal",
                                    "is_broken": false,
                                    "is_link_relation_conflict": false,
                                    "page_to_status_code": 200,
                                    "text": "About",
                                    "image_alt": null,
                                    "image_src": null,
                                    "link_attribute": null
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let parsed: DataForSeoApiResponseData<OnPageDataApiLinks> =
            serde_json::from_str(json).expect("links response should deserialize");
        let result = parsed.first_result().expect("one links result");
        assert_eq!(result.total_items_count, Some(2));
        let items = result.items.as_ref().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].link_type.as_deref(), Some("anchor"));
        assert_eq!(items[0].dofollow, Some(true));
        assert_eq!(items[0].page_to_status_code, Some(200));
    }

    /// <https://docs.dataforseo.com/v3/on_page/non_indexable/>
    #[test]
    fn deserialize_non_indexable() {
        let json = r#"{
            "version": "0.1.20240101",
            "status_code": 20000,
            "status_message": "Ok.",
            "time": "0.1 sec.",
            "cost": 0,
            "tasks_count": 1,
            "tasks_error": 0,
            "tasks": [
                {
                    "id": "01011234-1234-0123-0000-abcdef012345",
                    "status_code": 20000,
                    "status_message": "Ok.",
                    "time": "0.05 sec.",
                    "cost": 0,
                    "result_count": 1,
                    "path": ["v3", "on_page", "non_indexable"],
                    "data": {"api": "on_page", "function": "non_indexable"},
                    "result": [
                        {
                            "crawl_progress": "finished",
                            "crawl_status": {
                                "max_crawl_pages": 100,
                                "pages_in_queue": 0,
                                "pages_crawled": 10
                            },
                            "total_items_count": 1,
                            "items_count": 1,
                            "items": [
                                {
                                    "reason": "canonical_another_page",
                                    "url": "https://example.com/duplicate"
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let parsed: DataForSeoApiResponseData<OnPageDataApiNonIndexable> =
            serde_json::from_str(json).expect("non_indexable response should deserialize");
        let result = parsed.first_result().expect("one non_indexable result");
        assert_eq!(result.total_items_count, Some(1));
        let items = result.items.as_ref().unwrap();
        assert_eq!(items[0].reason.as_deref(), Some("canonical_another_page"));
        assert_eq!(
            items[0].url.as_deref(),
            Some("https://example.com/duplicate")
        );
    }
}
