use crate::entity::SerpApiSearchResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Finance Explore advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_explore/task_get/advanced/>.
pub type SerpApiGoogleFinanceExploreTaskAdvanced = SerpApiSearchResult<SerpApiGoogleFinanceItem>;
/// Google Finance Explore raw-HTML result.
pub type SerpApiGoogleFinanceExploreTaskHtml = SerpApiSearchResult<crate::entity::SerpApiHtmlItem>;

/// Google Finance Markets advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_markets/task_get/advanced/>.
pub type SerpApiGoogleFinanceMarketsTaskAdvanced = SerpApiSearchResult<SerpApiGoogleFinanceItem>;
/// Google Finance Markets raw-HTML result.
pub type SerpApiGoogleFinanceMarketsTaskHtml = SerpApiSearchResult<crate::entity::SerpApiHtmlItem>;

/// Google Finance Quote advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_quote/task_get/advanced/>.
pub type SerpApiGoogleFinanceQuoteTaskAdvanced = SerpApiSearchResult<SerpApiGoogleFinanceItem>;

/// Google Finance Ticker Search advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/finance_ticker_search/task_get/advanced/>.
pub type SerpApiGoogleFinanceTickerSearchTaskAdvanced =
    SerpApiSearchResult<SerpApiGoogleFinanceTickerSearchItem>;

/// A Google Finance block, tagged by the DataForSEO `type` field. Deeply nested
/// container payloads are kept as raw JSON to avoid over-modeling; unrecognized
/// types fall back to [`SerpApiGoogleFinanceItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleFinanceItem {
    /// Element of type `hero_groups`.
    #[serde(rename = "google_finance_hero_groups")]
    HeroGroups(Box<SerpApiElementFinanceHeroGroups>),
    /// Element of type `futures_chain`.
    #[serde(rename = "google_finance_futures_chain")]
    FuturesChain(Box<SerpApiElementFinanceHeroGroups>),
    /// Element of type `news`.
    #[serde(rename = "google_finance_news")]
    News(Box<SerpApiElementFinanceNews>),
    /// Element of type `explore_market_trends`.
    #[serde(rename = "google_finance_explore_market_trends")]
    ExploreMarketTrends(Box<SerpApiElementFinanceNews>),
    /// Element of type `market_trends`.
    #[serde(rename = "google_finance_market_trends")]
    MarketTrends(Box<SerpApiElementFinanceMarketTrends>),
    /// Element of type `interested`.
    #[serde(rename = "google_finance_interested")]
    Interested(Box<SerpApiElementFinanceList>),
    /// Element of type `most_followed`.
    #[serde(rename = "google_finance_most_followed")]
    MostFollowed(Box<SerpApiElementFinanceList>),
    /// Element of type `earnings_calendar`.
    #[serde(rename = "google_finance_earnings_calendar")]
    EarningsCalendar(Box<SerpApiElementFinanceList>),
    /// Element of type `people_also_search`.
    #[serde(rename = "google_finance_people_also_search")]
    PeopleAlsoSearch(Box<SerpApiElementFinanceList>),
    /// Element of type `compare_to`.
    #[serde(rename = "google_finance_compare_to")]
    CompareTo(Box<SerpApiElementFinanceList>),
    /// Element of type `quote`.
    #[serde(rename = "google_finance_quote")]
    Quote(Box<SerpApiElementFinanceQuote>),
    /// Element of type `financial`.
    #[serde(rename = "google_finance_financial")]
    Financial(Box<SerpApiElementFinanceFinancial>),
    /// Element of type `details`.
    #[serde(rename = "google_finance_details")]
    Details(Box<SerpApiElementFinanceDetails>),
    /// Element of type `about`.
    #[serde(rename = "google_finance_about")]
    About(Box<SerpApiElementFinanceAbout>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}

/// Finance block carrying a `markets` array (hero groups, futures chain).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceHeroGroups {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Market groups contained in the block.
    pub markets: Option<Vec<Value>>,
}

/// Finance news / market-trends block with a title and an `items` array.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceNews {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Title of the result.
    pub title: Option<String>,
    /// Secondary title of the result.
    pub sub_title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<Value>>,
}

/// Finance market-trends block whose `items` is an object (most_active/gainers/losers).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceMarketTrends {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Parsed elements of the result.
    pub items: Option<Value>,
}

/// Generic finance list block (interested, most followed, compare-to, ...).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceList {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Parsed elements of the result.
    pub items: Option<Vec<Value>>,
}

/// Finance quote block with the quote object and chart data points.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceQuote {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Quote data for the financial instrument.
    pub quote: Option<Value>,
    /// Chart data points for the quote.
    pub graph_items: Option<Vec<Value>>,
}

/// Finance financials block with quarterly and annual metrics.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceFinancial {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Quarterly financial metrics.
    pub quarterly_metrics: Option<Vec<Value>>,
    /// Annual financial metrics.
    pub annual_metrics: Option<Vec<Value>>,
}

/// Finance details block (price stats for a quote).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceDetails {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Badges shown on the result.
    pub badges: Option<Vec<Value>>,
    /// Previous closing price.
    pub previous_close: Option<Value>,
    /// Lower bound of the day's price range.
    pub start_day_range: Option<Value>,
    /// Upper bound of the day's price range.
    pub end_day_range: Option<Value>,
    /// Market capitalization.
    pub market_cap: Option<Value>,
    /// Trading volume.
    pub volume: Option<Value>,
    /// Price-to-earnings ratio.
    pub pe_ratio: Option<Value>,
    /// Dividend yield.
    pub dividend_yield: Option<Value>,
    /// Primary exchange the instrument trades on.
    pub primary_exchange: Option<String>,
    /// Currency of the reported metrics.
    pub metrics_currency: Option<String>,
}

/// Finance "about the company" block.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceAbout {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Name as displayed in the SERP.
    pub displayed_name: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Name of the company's chief executive officer.
    pub ceo: Option<String>,
    /// When the company was founded.
    pub founded: Option<String>,
    /// Location of the company's headquarters.
    pub headquarters: Option<String>,
    /// URL of the company's website.
    pub website: Option<String>,
    /// Number of employees.
    pub employees: Option<i64>,
}

/// A Google Finance ticker-search result, tagged by the DataForSEO `type` field.
/// Unrecognized types fall back to [`SerpApiGoogleFinanceTickerSearchItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleFinanceTickerSearchItem {
    /// Element of type `market_index`.
    #[serde(rename = "google_finance_market_index")]
    MarketIndex(Box<SerpApiElementFinanceMarketIndex>),
    /// Element of type `asset_pair`.
    #[serde(rename = "google_finance_asset_pair")]
    AssetPair(Box<SerpApiElementFinanceAssetPair>),
    /// Element of type `market_instrument`.
    #[serde(rename = "google_finance_market_instrument")]
    MarketInstrument(Box<SerpApiElementFinanceMarketInstrument>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}

/// `google_finance_market_index` ticker-search item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceMarketIndex {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Ticker symbol of the instrument.
    pub ticker: Option<String>,
    /// Market identifier of the instrument.
    pub market_identifier: Option<String>,
    /// Current value of the index.
    pub index_value: Option<f64>,
    /// Absolute change in the index value.
    pub index_value_delta: Option<f64>,
    /// Unique identifier of the instrument.
    pub identifier: Option<String>,
    /// Name as displayed in the SERP.
    pub displayed_name: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Direction of the price trend, `up` or `down`.
    pub trend: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Change as a percentage.
    pub percentage_delta: Option<f64>,
}

/// `google_finance_asset_pair` ticker-search item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceAssetPair {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Symbol of the base asset in the pair.
    pub base_symbol: Option<String>,
    /// Symbol of the quote asset in the pair.
    pub quote_symbol: Option<String>,
    /// Display name of the base asset.
    pub base_display_name: Option<String>,
    /// Display name of the quote asset.
    pub quote_display_name: Option<String>,
    /// Price information shown for the result.
    pub price: Option<f64>,
    /// Absolute change in price.
    pub price_delta: Option<f64>,
    /// Unique identifier of the instrument.
    pub identifier: Option<String>,
    /// Name as displayed in the SERP.
    pub displayed_name: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Direction of the price trend, `up` or `down`.
    pub trend: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Change as a percentage.
    pub percentage_delta: Option<f64>,
}

/// `google_finance_market_instrument` ticker-search item.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFinanceMarketInstrument {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Ticker symbol of the instrument.
    pub ticker: Option<String>,
    /// Price information shown for the result.
    pub price: Option<f64>,
    /// Absolute change in price.
    pub price_delta: Option<f64>,
    /// Currency of the price.
    pub price_currency: Option<String>,
    /// Unique identifier of the instrument.
    pub identifier: Option<String>,
    /// Name as displayed in the SERP.
    pub displayed_name: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Direction of the price trend, `up` or `down`.
    pub trend: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Change as a percentage.
    pub percentage_delta: Option<f64>,
}
