use indicators_core::{classify_market_trend as cmt_core, Bar, TrendAnalysis};
use napi_derive::napi;

/// Classify Market Trend
#[napi]
pub fn classify_market_trend(
	market_data: Vec<Bar>,
	trailing_period_length: Option<u32>,
) -> TrendAnalysis {
	cmt_core(market_data, trailing_period_length)
}
