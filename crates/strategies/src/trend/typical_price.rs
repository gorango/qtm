use crate::types::configs::TypicalPriceConfig;

/// Typical Price Trend Strategy
///
/// Generates buy signals when close > typical price
/// Generates sell signals when close < typical price
///
/// @strategy_id typicalPrice
/// @strategy_name Typical Price Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn typical_price_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TypicalPriceConfig>,
) -> Result<Vec<i8>, String> {
	let _config = config.unwrap_or_default();

	let data_len = closes.len();
	if data_len == 0 {
		return Err("Input arrays cannot be empty".to_string());
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

/// Get Typical Price strategy metadata for registry
pub fn typical_price_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "typicalPrice",
		"name": "Typical Price Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when close is above typical price and sell signals when close is below typical price"
	})
}

/// Get Typical Price strategy default parameters
pub fn typical_price_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {},
		"optimization_bounds": []
	})
}
