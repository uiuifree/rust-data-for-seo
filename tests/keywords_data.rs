use data_for_seo::DataForSeoClient;
use data_for_seo::entity::{
    DataForSeoApiResponseData, KeywordsDataApiBingKeyword, KeywordsDataApiBingSearchVolumeHistory,
    KeywordsDataApiClickstreamGlobalSearchVolume, KeywordsDataApiDataforseoTrendsItem,
    KeywordsDataApiDataforseoTrendsResult, KeywordsDataApiGoogleAdsSearchVolumeTask,
    KeywordsDataApiGoogleTrendsExplore, KeywordsDataApiGoogleTrendsItem,
};
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();
    DataForSeoClient::new(id, pass)
}

// --- Deserialization tests (offline, based on documented sample responses) ---

#[test]
fn deserialize_google_ads_search_volume() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "result_count": 1,
          "path": ["v3", "keywords_data", "google_ads", "search_volume", "live"],
          "data": {},
          "result": [
            {
              "keyword": "seo",
              "location_code": 2840,
              "language_code": "en",
              "search_partners": false,
              "competition": "LOW",
              "competition_index": 12,
              "search_volume": 90500,
              "low_top_of_page_bid": 1.2,
              "high_top_of_page_bid": 4.5,
              "cpc": 3.1,
              "monthly_searches": [
                {"year": 2024, "month": 1, "search_volume": 90500}
              ]
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiGoogleAdsSearchVolumeTask> =
        serde_json::from_str(json).expect("valid search volume response");
    let result = parsed.first_result().expect("one result");
    assert_eq!(result.keyword.as_deref(), Some("seo"));
    assert_eq!(result.search_volume, Some(90500));
    assert_eq!(
        result
            .monthly_searches
            .as_ref()
            .map(|m| m.len())
            .unwrap_or_default(),
        1
    );
}

#[test]
fn deserialize_bing_search_volume() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "path": ["v3", "keywords_data", "bing", "search_volume", "live"],
          "data": {},
          "result": [
            {
              "keyword": "seo",
              "location_code": 2840,
              "language_code": "en",
              "search_partners": false,
              "device": "all",
              "competition": 0.5,
              "cpc": 2.4,
              "search_volume": 74000,
              "categories": null,
              "monthly_searches": [
                {"year": 2024, "month": 2, "search_volume": 74000}
              ]
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiBingKeyword> =
        serde_json::from_str(json).expect("valid bing search volume response");
    let result = parsed.first_result().expect("one result");
    assert_eq!(result.keyword.as_deref(), Some("seo"));
    assert_eq!(result.competition, Some(0.5));
    assert_eq!(result.search_volume, Some(74000));
}

#[test]
fn deserialize_google_trends_explore() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "path": ["v3", "keywords_data", "google_trends", "explore", "live"],
          "data": {},
          "result": [
            {
              "keywords": ["seo"],
              "type": "google_trends",
              "location_code": 2840,
              "language_code": "en",
              "check_url": "https://trends.google.com/trends/explore",
              "datetime": "2024-01-01 00:00:00 +00:00",
              "items_count": 3,
              "items": [
                {
                  "position": 1,
                  "type": "google_trends_graph",
                  "title": "Interest over time",
                  "keywords": ["seo"],
                  "data": [
                    {"date_from": "2023-01-01", "date_to": "2023-01-07", "timestamp": 1672531200, "missing_data": false, "values": [72]}
                  ],
                  "averages": [70]
                },
                {
                  "position": 2,
                  "type": "google_trends_queries_list",
                  "title": "Related queries",
                  "keywords": ["seo"],
                  "data": {
                    "top": [{"query": "seo tools", "value": "100"}],
                    "rising": [{"query": "ai seo", "value": "Breakout"}]
                  }
                },
                {
                  "position": 3,
                  "type": "some_future_element",
                  "title": "Unknown block"
                }
              ]
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiGoogleTrendsExplore> =
        serde_json::from_str(json).expect("valid google trends explore response");
    let result = parsed.first_result().expect("one result");
    let items = result.items.as_ref().expect("items present");
    assert_eq!(items.len(), 3);

    match &items[0] {
        KeywordsDataApiGoogleTrendsItem::GoogleTrendsGraph(graph) => {
            let point = &graph.data.as_ref().unwrap()[0];
            assert_eq!(point.values.as_ref().unwrap()[0], 72);
        }
        other => panic!("expected graph, got {other:?}"),
    }
    match &items[1] {
        KeywordsDataApiGoogleTrendsItem::GoogleTrendsQueriesList(list) => {
            assert_eq!(
                list.data.as_ref().unwrap().top.as_ref().unwrap()[0]
                    .query
                    .as_deref(),
                Some("seo tools")
            );
        }
        other => panic!("expected queries list, got {other:?}"),
    }
    // Unknown element types must fall back gracefully rather than fail parsing.
    assert!(matches!(
        &items[2],
        KeywordsDataApiGoogleTrendsItem::Unknown
    ));
}

#[test]
fn deserialize_dataforseo_trends_demography() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "path": ["v3", "keywords_data", "dataforseo_trends", "demography", "live"],
          "data": {},
          "result": [
            {
              "keywords": ["rugby"],
              "type": "dataforseo_trends",
              "location_code": 2826,
              "language_code": null,
              "datetime": "2024-01-01 00:00:00 +00:00",
              "items_count": 1,
              "items": [
                {
                  "position": 1,
                  "type": "demography",
                  "keywords": ["rugby"],
                  "demography": {
                    "age": [
                      {"keyword": "rugby", "values": [{"type": "18-24", "value": 73}, {"type": "25-34", "value": 87}]}
                    ],
                    "gender": [
                      {"keyword": "rugby", "values": [{"type": "female", "value": 79}, {"type": "male", "value": 100}]}
                    ]
                  },
                  "demography_comparison": {"gender": {"female": [20, 80], "male": [20, 80]}}
                }
              ]
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiDataforseoTrendsResult> =
        serde_json::from_str(json).expect("valid dataforseo trends demography response");
    let result = parsed.first_result().expect("one result");
    let items = result.items.as_ref().expect("items present");
    match &items[0] {
        KeywordsDataApiDataforseoTrendsItem::Demography(demo) => {
            let age = &demo.demography.as_ref().unwrap().age.as_ref().unwrap()[0];
            assert_eq!(age.keyword.as_deref(), Some("rugby"));
            assert_eq!(
                age.values.as_ref().unwrap()[0].segment_type.as_deref(),
                Some("18-24")
            );
            assert!(demo.demography_comparison.is_some());
        }
        other => panic!("expected demography, got {other:?}"),
    }
}

#[test]
fn deserialize_clickstream_global_search_volume() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "path": ["v3", "keywords_data", "clickstream_data", "global_search_volume", "live"],
          "data": {},
          "result": [
            {
              "keyword": "youtube",
              "search_volume": 976222640,
              "country_distribution": [
                {"country_iso_code": "US", "search_volume": 145770000, "percentage": 14.93}
              ]
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiClickstreamGlobalSearchVolume> =
        serde_json::from_str(json).expect("valid clickstream global search volume response");
    let result = parsed.first_result().expect("one result");
    assert_eq!(result.keyword.as_deref(), Some("youtube"));
    assert_eq!(result.search_volume, Some(976222640));
    let dist = result.country_distribution.as_ref().unwrap();
    assert_eq!(dist[0].country_iso_code.as_deref(), Some("US"));
    assert_eq!(dist[0].percentage, Some(14.93));
}

#[test]
fn deserialize_bing_search_volume_history() {
    let json = r#"{
      "version": "0.1.20240101",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "0.1 sec.",
      "cost": 0.05,
      "tasks_count": 1,
      "tasks_error": 0,
      "tasks": [
        {
          "id": "task-1",
          "status_code": 20000,
          "status_message": "Ok.",
          "time": "0.1 sec.",
          "cost": 0.05,
          "path": ["v3", "keywords_data", "bing", "search_volume_history", "live"],
          "data": {},
          "result": [
            {
              "keyword": "seo",
              "location_code": 2840,
              "language_code": "en",
              "device": ["desktop", "mobile"],
              "period": "monthly",
              "searches": {
                "desktop": [
                  {"year": 2024, "month": 1, "day": 1, "search_volume": 61000}
                ],
                "mobile": [
                  {"year": 2024, "month": 1, "day": 1, "search_volume": 42000}
                ],
                "tablet": null,
                "non_smartphones": null
              }
            }
          ]
        }
      ]
    }"#;

    let parsed: DataForSeoApiResponseData<KeywordsDataApiBingSearchVolumeHistory> =
        serde_json::from_str(json).expect("valid bing search volume history response");
    let result = parsed.first_result().expect("one result");
    assert_eq!(result.keyword.as_deref(), Some("seo"));
    assert_eq!(result.period.as_deref(), Some("monthly"));
    let searches = result.searches.as_ref().expect("searches present");
    let desktop = searches.desktop.as_ref().expect("desktop series");
    assert_eq!(desktop[0].search_volume, Some(61000));
    assert_eq!(desktop[0].year, Some(2024));
    assert!(searches.tablet.is_none());
}

// --- Live integration tests (hit the paid DataForSEO API) ---

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn live_google_ads_search_volume() {
    use data_for_seo::KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest;
    let client = client();
    let mut request =
        KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest::new("en".to_string(), 2840);
    request.keywords = vec!["seo".to_string()];
    let response = client
        .keywords_data()
        .google_ads()
        .search_volume_live(vec![request])
        .await
        .unwrap();
    for result in response.results() {
        println!("{result:?}");
    }
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn live_google_trends_explore() {
    use data_for_seo::KeywordsDataApiGoogleTrendsExploreTaskPostRequest;
    let client = client();
    let mut request =
        KeywordsDataApiGoogleTrendsExploreTaskPostRequest::new(vec!["seo".to_string()]);
    request.location_code = Some(2840);
    let response = client
        .keywords_data()
        .google_trends()
        .explore_live(vec![request])
        .await
        .unwrap();
    for result in response.results() {
        println!("{result:?}");
    }
}

#[tokio::test]
#[ignore = "requires DataForSEO credentials"]
async fn live_google_ads_status() {
    let client = client();
    let response = client.keywords_data().google_ads().status().await.unwrap();
    for result in response.results() {
        println!("{result:?}");
    }
}
