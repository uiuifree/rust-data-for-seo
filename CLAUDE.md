# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`data-for-seo` is a Rust client library (crate) for the [DataForSEO API](https://docs.dataforseo.com/v3/) v3, covering all v3 API domains. Published on crates.io as `data-for-seo`. Uses Rust edition 2024.

## Build & Test Commands

```bash
cargo build                          # Build the library
cargo test                           # Offline deserialization tests (no credentials needed)
cargo test -- --ignored              # Live API tests (requires .env with ID and PASSWORD)
cargo test --test serp               # Run a specific test file
cargo clippy --all-targets           # Must stay warning-free
cargo publish --dry-run              # Verify package before publishing
```

Live tests are marked `#[ignore = "requires DataForSEO credentials"]` and hit the real API using `ID`/`PASSWORD` from `.env`. Deserialization tests embed sample JSON and run offline — every new endpoint should get one.

## Architecture

### Two-layer structure: `api/` and `entity/`

- **`api/`** — HTTP endpoint methods. Each API domain has a sub-module with request structs and async methods calling the REST API. Endpoints are addressed by path (`"/v3/..."`); the base URL lives in the client.
- **`entity/`** — Response/data models. Mirrors the `api/` structure. All models derive `Debug`/`Default`/`Serialize`/`Deserialize`/`Clone`, and response fields are `Option<T>` for null-resilience. Polymorphic `items` arrays use enums with an `Unknown(Value)` fallback so unknown types never fail deserialization.

### Client pattern

`DataForSeoClient` (in `lib.rs`) holds a shared `reqwest::Client`, the Basic-auth token, and a configurable base URL (`with_base_url`, `sandbox()`). Domain accessors return borrowing API structs; engine-specific sub-APIs use two-level chaining:

```
client.serp().google() / .bing() / .youtube() / .baidu() / .yahoo() / .seznam() / .naver()
client.keywords_data().google_ads() / .bing() / .google_trends() / .dataforseo_trends() / .clickstream_data()
client.dataforseo_labs().google() / .amazon() / .apple()
client.on_page()          client.backlinks()
client.domain_analytics().technologies() / .whois()
client.content_analysis() client.content_generation()
client.merchant().amazon() / .google()
client.app_data().google() / .apple()
client.business_data().google() / .trustpilot() / .tripadvisor() / .social_media() / .business_listings()
client.ai_optimization().ai_keyword_data() / .llm_mentions() / .chat_gpt() / .claude() / .gemini() / .perplexity()
```

### Generic response wrapper

All calls return `DataForSeoApiResponse<R>` = `Result<DataForSeoApiResponseData<R>, DataForSeoError>`. Use the helpers instead of unwrapping the raw `Option` fields: `response.tasks()`, `response.results()`, `response.into_results()`, `response.first_result()`, `task.results()`, `task.is_success()`.

### Error handling

`DataForSeoError` (in `error.rs`) implements `std::error::Error` with three variants: `Api` (DataForSEO error envelope with `status_code`/`status_message`), `Http` (non-JSON HTTP failure), `System` (transport/serde failure).

### Naming conventions

- API structs: `{Domain}Api{Endpoint}` (e.g., `BacklinksApiSummaryPost`, `OnPageApiTaskPost`)
- Request structs take required fields via `new()`; optional fields are `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
- When adding SERP element types: create a file in `entity/serp/element/`, add `mod` + `pub use` in `element/mod.rs`

## Conventions for changes

- Never invent response fields — model exactly what the docs (or the official DataForSEO clients) specify; use `serde_json::Value` when the schema is undocumented
- Keep `cargo clippy --all-targets` warning-free
- `cargo fmt` before finishing
- Each new endpoint needs a doc comment with its docs URL and an offline deserialization test
