# Changelog

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
