# data-for-seo

[![crates.io](https://img.shields.io/crates/v/data-for-seo.svg)](https://crates.io/crates/data-for-seo)
[![docs.rs](https://docs.rs/data-for-seo/badge.svg)](https://docs.rs/data-for-seo)
[![license](https://img.shields.io/crates/l/data-for-seo.svg)](./LICENSE)

`data-for-seo` is a Rust client library for the [DataForSEO API v3](https://docs.dataforseo.com/v3/). It covers every API domain of v3 with typed request/response models, while staying resilient to API changes (unknown fields and item types never break deserialization).

```toml
[dependencies]
data-for-seo = "0.2"
```

## Quick start

```rust
use data_for_seo::{DataForSeoClient, BacklinksApiSummaryPost};

#[tokio::main]
async fn main() -> Result<(), data_for_seo::error::DataForSeoError> {
    let client = DataForSeoClient::new("login", "password");

    let response = client
        .backlinks()
        .summary(vec![BacklinksApiSummaryPost::new("https://example.com/")])
        .await?;

    for result in response.results() {
        println!("backlinks: {:?}", result.backlinks);
    }
    Ok(())
}
```

The client is organized by API domain, mirroring the DataForSEO documentation:

```text
client.serp().google()            client.dataforseo_labs().google()
client.serp().bing()              client.dataforseo_labs().amazon()
client.serp().youtube()           client.dataforseo_labs().apple()
client.keywords_data().google_ads()
client.keywords_data().bing()     client.merchant().amazon()
client.on_page()                  client.merchant().google()
client.backlinks()                client.app_data().google()
client.domain_analytics().technologies()
client.content_analysis()         client.business_data().google()
client.content_generation()       client.ai_optimization().chat_gpt()
```

### Task-based endpoints

Many endpoints follow DataForSEO's task flow (`task_post` → `tasks_ready` → `task_get`):

```rust
use data_for_seo::{DataForSeoClient, SerpApiGoogleOrganicTaskPostRequest};

async fn example(client: &DataForSeoClient) -> Result<(), data_for_seo::error::DataForSeoError> {
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 2392);
    request.keyword = "seo".to_string();

    let posted = client.serp().google().organic_task_post(vec![request]).await?;
    let task_id = &posted.tasks()[0].id;

    // ... poll tasks_ready, then:
    let result = client.serp().google().organic_task_get_advanced(task_id).await?;
    for page in result.results() {
        for item in page.items() {
            println!("{:?}", item);
        }
    }
    Ok(())
}
```

### Response helpers

Every call returns `DataForSeoApiResponse<T>` (= `Result<DataForSeoApiResponseData<T>, DataForSeoError>`). The response data exposes:

- `response.tasks()` — all tasks as a slice (never panics on `null`)
- `response.results()` / `response.into_results()` — iterate every result of every task
- `response.first_result()` — first result across all tasks
- `task.results()`, `task.is_success()`, `task.task_status()` — per-task access

### Credentials

Pass your API login and password directly (`DataForSeoClient::new`), or read them
from the `DATAFORSEO_LOGIN` / `DATAFORSEO_PASSWORD` environment variables:

```rust
use data_for_seo::DataForSeoClient;

let client = DataForSeoClient::from_env()?; // DATAFORSEO_LOGIN / DATAFORSEO_PASSWORD
```

### Sandbox

DataForSEO's [sandbox](https://docs.dataforseo.com/v3/appendix/sandbox/) is free and returns mock data:

```rust
use data_for_seo::DataForSeoClient;
let client = DataForSeoClient::new("login", "password").sandbox();
```

### Error handling

`DataForSeoError` implements `std::error::Error` and distinguishes:

- `Api` — the API returned an error envelope (`status_code`, `status_message`, see [error codes](https://docs.dataforseo.com/v3/appendix/errors/))
- `Http` — a non-JSON HTTP failure
- `System` — transport or deserialization failure

## Supported API domains (all v3 domains)

| Domain | Coverage |
| --- | --- |
| **SERP** | Google (Organic, AI Mode, Maps, Local Finder, News, Events, Images, Jobs, Autocomplete, Dataset Search/Info, Ads Advertisers, Ads Search), Bing, YouTube (Organic, Video Info/Subtitles/Comments), Baidu, Yahoo, Seznam, Naver |
| **Keywords Data** | Google Ads (Search Volume, Keywords For Site/Keywords, Ad Traffic), Bing (Search Volume, Keyword Performance, Search Volume History, Keyword Suggestions For URL, Audience Estimation, ...), Google Trends, DataForSEO Trends, Clickstream Data |
| **DataForSEO Labs** | Google (Ranked Keywords, Keyword Ideas/Suggestions, Related Keywords, Competitors, Domain/Page Intersection, Traffic Estimation, Search Intent, Historical data, ...), Amazon, Apple App Store |
| **Backlinks** | Summary, History, Backlinks, Anchors, Domain Pages, Referring Domains/Networks, Competitors, Intersections, Timeseries, Bulk endpoints, Index |
| **OnPage** | Task Post, Summary, Pages, Resources, Links, Duplicate Tags/Content, Redirect Chains, Non-Indexable, Waterfall, Keyword Density, Microdata, Raw HTML, Screenshot, Content Parsing, Instant Pages, Lighthouse |
| **Domain Analytics** | Technologies (full), Whois |
| **Content Analysis** | Search, Summary, Sentiment Analysis, Rating Distribution, Phrase/Category Trends |
| **Content Generation** | Generate (Text/Meta Tags/Sub Topics), Paraphrase, Check Grammar, Text Summary |
| **Merchant** | Amazon (Products, ASIN, Sellers), Google Shopping (Products, Sellers, Product Spec/Info) |
| **App Data** | Google Play, Apple App Store (App Searches, App List, App Info, App Reviews) |
| **Business Data** | Google (My Business, Hotels, Reviews, Q&A), Trustpilot, Tripadvisor, Social Media, Business Listings |
| **AI Optimization** | AI Keyword Data, LLM Mentions, LLM Responses (ChatGPT, Claude, Gemini, Perplexity), LLM Scraper |

A few endpoints whose response schema is not documented return `serde_json::Value` so the raw data is still accessible.

## Testing

```bash
cargo test                 # offline deserialization tests (no credentials needed)
cargo test -- --ignored    # live API tests; requires .env with ID and PASSWORD
```

## License

MIT
