use crate::types::configs::PivotPointsConfig;

/// Pivot Points Trend Strategy
///
/// Generates signals based on price vs pivot levels
/// Buy when price breaks above pivot, sell when price breaks below pivot
///
/// @strategy_id pivotPoints
/// @strategy_name Pivot Points Trend
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn pivot_points_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<PivotPointsConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let _period_high = config.period_high.unwrap_or(20); // Not used in current implementation
	let _period_low = config.period_low.unwrap_or(20); // Not used in current implementation

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("Pivot Points period must be between 2 and 100".to_string());
	}
	let data_len = highs.len();
	if data_len < period as usize {
		return Err("Insufficient data for Pivot Points strategy".to_string());
	}

	// Generate signals based on pivot levels
	// For simplicity, calculate pivot as (H+L+C)/3 for each bar
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < (period as usize - 1) {
			signals.push(0); // Not enough data
			continue;
		}

		// Calculate pivot using current bar (simplified)
		let pivot = (highs[i] + lows[i] + closes[i]) / 3.0;

		let signal = if closes[i] > pivot {
			1 // Buy signal: price above pivot
		} else if closes[i] < pivot {
			-1 // Sell signal: price below pivot
		} else {
			0 // Hold: price at pivot
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Pivot Points strategy metadata for registry
pub fn pivot_points_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "pivotPoints",
		"name": "Pivot Points Trend",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates signals based on price position relative to pivot levels"
	})
}

/// Get Pivot Points strategy default parameters
pub fn pivot_points_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"period_high": 20,
			"period_low": 20
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
