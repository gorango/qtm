use crate::types::configs::SuperTrendConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Super Trend Trend Strategy
///
/// Generates signals on trend direction changes
/// Buy when trend changes to up, sell when trend changes to down
#[strategy(
	id = "super_trend",
	name = "Super Trend Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when trend changes to up and sell signals when trend changes to down",
	opt_params = r#"[
		{"param_name": "period", "min": 2.0, "max": 20.0, "step": 1.0},
		{"param_name": "multiplier", "min": 1.0, "max": 5.0, "step": 0.5}
	]"#
)]
pub fn super_trend_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<SuperTrendConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(3);
	let multiplier = config.multiplier.unwrap_or(3.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"SuperTrend period must be between 2 and 100".into(),
		));
	}
	let data_len = highs.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for SuperTrend strategy".into(),
		));
	}

	// Calculate SuperTrend
	let supertrend_result =
		indicators_core::super_trend(highs, lows, closes, Some(period), Some(multiplier))?;

	// Generate signals on direction changes
	let mut signals = Vec::with_capacity(data_len);
	let mut prev_direction = 0;

	for i in 0..data_len {
		let current_direction = supertrend_result.direction[i];
		let signal = if i < min_periods {
			0 // Not enough data
		} else if prev_direction == -1 && current_direction == 1 {
			1 // Buy signal: trend changed to up
		} else if prev_direction == 1 && current_direction == -1 {
			-1 // Sell signal: trend changed to down
		} else {
			0 // Hold: no change
		};
		signals.push(signal);
		prev_direction = current_direction;
	}

	Ok(signals)
}
