use crate::types::configs::AbsolutePriceOscillatorConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use serde_json;
use strategies_proc_macro::strategy;

/// Absolute Price Oscillator Trend Strategy
///
/// Generates buy signals when APO crosses above zero line
/// Generates sell signals when APO crosses below zero line
#[strategy(
	id = "absolutePriceOscillator",
	name = "Absolute Price Oscillator Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when APO crosses above zero and sell signals when APO crosses below zero",
	opt_params = r#"[
		{"param_name": "fastPeriod", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "slowPeriod", "min": 15.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn absolute_price_oscillator_strategy(
	closes: &[f64],
	config: Option<AbsolutePriceOscillatorConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(10);
	let slow_period = config.slow_period.unwrap_or(20);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err(StrategyError::Validation(
			"APO fast period must be between 2 and 100".into(),
		));
	}
	if !(2..=200).contains(&slow_period) {
		return Err(StrategyError::Validation(
			"APO slow period must be between 2 and 200".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = slow_period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Absolute Price Oscillator strategy".into(),
		));
	}

	// Calculate APO
	let apo_result =
		indicators_core::absolute_price_oscillator(closes, Some(fast_period), Some(slow_period))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);
	let zero_line = vec![0.0; apo_result.len()];

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&apo_result, &zero_line, i as u32) {
			1 // Buy signal: APO crosses above zero
		} else if crossed_under_series(&apo_result, &zero_line, i as u32) {
			-1 // Sell signal: APO crosses below zero
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
