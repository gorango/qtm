use crate::types::configs::TypicalPriceConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Typical Price Trend Strategy
///
/// Generates buy signals when close > typical price
/// Generates sell signals when close < typical price
#[strategy(
	id = "typical_price",
	name = "Typical Price Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when close is above typical price and sell signals when close is below typical price",
	opt_params = r#"[]"#
)]
pub fn typical_price_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TypicalPriceConfig>,
) -> StrategyResult<Vec<i8>> {
	let _config = config.unwrap_or_default();

	let data_len = closes.len();
	if data_len == 0 {
		return Err(StrategyError::Validation(
			"Input arrays cannot be empty".into(),
		));
	}

	// Calculate Typical Price
	let typical_result = indicators_core::typical_price(highs, lows, closes)?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if closes[i] > typical_result[i] {
			1 // Buy signal: close > typical price
		} else if closes[i] < typical_result[i] {
			-1 // Sell signal: close < typical price
		} else {
			0 // Hold: close == typical price
		};
		signals.push(signal);
	}

	Ok(signals)
}
