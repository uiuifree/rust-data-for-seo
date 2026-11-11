# Changelog

## 0.5.0 (2026-11-11)

### Breaking changes

- Removed the Google Events SERP endpoints, retired by DataForSEO on 2026-09-15.
- Removed the Pinterest endpoint and `BusinessDataApi::social_media`. DataForSEO retired the Social Media API on 2026-09-16.
- Removed `jobs_live_advanced` and `maps_task_get_html`, which are not part of the API.

- Removed `SerpApi::locations`, `SerpApi::locations_country`, `KeywordsDataApi::locations`, `KeywordsDataApi::locations_country` and the top-level Keywords Data `tasks_fixed`, which return HTTP 404.
- Corrected response types found to differ from live API responses: OnPage `certificate_version` and topic `level`; Backlinks Domain Pages `h1`-`h3`, `technologies`, `page_summary`; LLM Mentions `key` and the `_lite` results; Content Analysis nullable maps and ratings; Technologies list; `id_list` `status`; SERP `local_pack`, `hotels_pack`, `answer_box`, `organic`, `paid`, `faq` fields; Dataset `size`.
- Clickstream endpoints now return `KeywordsDataApiClickstreamResult`, matching the `items` array of the API.
- `DataForSeoLabsAmazonProductRankOverviewRequest` now takes `asins`.

### Added

- SERP: `screenshot`, `ai_summary`, `id_list`, `errors`, the Standard method and HTML results of Google AI Mode, Live and HTML variants of Finance, Images, Local Finder, News, Organic, Bing and Yahoo, Regular results of Baidu, Bing, Naver, Seznam and Yahoo, Ads Search and Ads Advertisers `locations` and Live, and the remaining `tasks_fixed`, `languages` and per-country `locations` endpoints.
- DataForSEO Labs: `historical_search_volume`, `available_history`, `available_filters`, `errors`, `categories_for_keywords_languages`.
- AI Optimization: `search_mentions`, `available_filters`, per-country LLM Scraper locations.
- OnPage: `uncrawlable_resources`, Lighthouse `audits`, `versions`, `languages`.
- App Data: App Listings search and categories, `tasks_ready`, Google App Reviews HTML, per-country locations.
- Appendix: `errors`, `status`, `webhook_resend`.
- Keywords Data and Merchant: `id_list`, `errors`; Merchant and Business Data: `tasks_ready`; per-country locations; Bing Keyword Performance locations and languages.
- Domain Analytics and Business Listings: `available_filters`, Technologies `languages` and `locations`.

### Fixed

- Google Ads `tasks_ready` and `tasks_fixed` now use the `google_ads` path.
- `organic_tasks_ready` for Google, Bing, Yahoo, Baidu, Naver and Seznam now only lists organic tasks.

## 0.4.0 (2026-11-07)

### Breaking changes

- Upgraded `reqwest` from 0.12 to 0.13. `DataForSeoClient::with_http_client` now takes a `reqwest` 0.13 `Client`. reqwest 0.13 uses rustls with the platform certificate verifier as its default TLS backend.
- Declared `rust-version = "1.85"` (the minimum required by edition 2024, `reqwest` 0.13, and `tokio`).

### Changed

- Upgraded `base64` to 0.23.
- Replaced the unmaintained dev-dependency `dotenv` (RUSTSEC-2021-0141) with `dotenvy`.

## 0.3.1 (2026-11-07)

### Security

- `DataForSeoClient`'s `Debug` output no longer contains the Basic-auth token. The derived implementation printed the Base64-encoded `login:password`, so logging the client with `{:?}` exposed the API credentials. The token is now shown as `<redacted>`.

## 0.3.0 (2026-07-18)

### Breaking changes

- Unified numeric types across all domains: counts, sizes, and ranks are `i64` (overflow-safe — backlink counts and search volumes exceed the `i32` range in practice), money, ratios, and scores are `f64`, `location_code` and `limit`/`offset` are `i32`. All `f32`/`u32`/`u64` fields are gone, including the response envelope's `cost` (now `f64`) and `limit`/`offset` (now `Option<i32>`).

### Fixed

- `SerpApiTaskReadyResult`, `KeywordsDataApiTaskReadyResult`, and a few SERP element structs had private fields, making Tasks Ready results unreadable. All response fields are now public.

### Added

- Runnable examples for the main workflows: `serp_live`, `serp_task_flow` (post → poll → fetch), `keywords_search_volume`, `backlinks_overview`, `labs_keyword_research`, `onpage_audit`.

## 0.2.0 (2026-07-18)

Complete redesign targeting full DataForSEO API v3 coverage.

### Breaking changes

- `DataForSeoError` variants renamed: `ApiError`/`HttpError`/`SystemError` → `Api`/`Http`/`System`. The error payload now matches DataForSEO's actual error envelope (`status_code`, `status_message`) and implements `std::error::Error`.
- `DataForSeoApiResponseData::tasks` is `Option<Vec<...>>`; use the new `tasks()` / `results()` / `first_result()` helpers instead of unwrapping fields.
- All entity types are re-exported at the crate root (`data_for_seo::SerpApiGoogleOrganicItem`); `data_for_seo::entity::...` paths continue to work.
- `SerpApiGoogleOrganicItem` variants are boxed.

### Added

- Full coverage of all v3 API domains: SERP (Google incl. AI Mode/Maps/Local Finder/Finance/Search By Image, Bing, YouTube, Baidu, Yahoo, Seznam, Naver), Keywords Data (Google Ads, Bing, Google Trends, DataForSEO Trends, Clickstream), DataForSEO Labs (Google, Amazon, Apple), Backlinks, OnPage, Domain Analytics, Content Analysis, Content Generation, Merchant, App Data, Business Data, AI Optimization, and Appendix user data — 490+ endpoint methods in total.
- `DataForSeoClient::sandbox()` and `with_base_url()` for the free sandbox environment; `with_http_client()` for custom timeouts/proxies.
- The client now reuses a single `reqwest::Client` connection pool.
- Offline deserialization tests for every domain (no credentials required); live tests are `#[ignore]`d.
- Documentation for every public item (`#![warn(missing_docs)]` enforced).

### Fixed

- OnPage: Lighthouse `task_get` URL path order, `instant_pages`/`page_screenshot` response shapes, `check_spell` type.
- Backlinks: `indirect_link_path` structure, `PageIntersection` field types, missing fields in `anchors`/`history` items.
- Error responses are now parsed from DataForSEO's real envelope instead of an unrelated schema.

## 0.1.x

Initial releases covering parts of SERP, Keywords Data, OnPage, and Backlinks.
