use crate::types::configs::IchimokuCloudConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Ichimoku Cloud Trend Strategy
///
/// Generates buy signals when price crosses above cloud top
/// Generates sell signals when price crosses below cloud bottom
#[strategy(
	id = "ichimoku",
	name = "Ichimoku Cloud Trend",
	category = "momentum",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Generates buy signals when price crosses above cloud top and sell signals when price crosses below cloud bottom",
	opt_params = r#"[
		{"param_name": "short", "min": 5.0, "max": 15.0, "step": 1.0},
		{"param_name": "medium", "min": 20.0, "max": 35.0, "step": 1.0},
		{"param_name": "long", "min": 40.0, "max": 70.0, "step": 2.0},
		{"param_name": "close", "min": 20.0, "max": 35.0, "step": 1.0}
	]"#
)]
pub fn ichimoku_strategy(
	closes: &[f64],
	highs: &[f64],
	lows: &[f64],
	config: Option<IchimokuCloudConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let short = config.short.unwrap_or(9);
	let medium = config.medium.unwrap_or(26);
	let long = config.long.unwrap_or(52);
	let close_period = config.close.unwrap_or(26);

	// Validate parameters
	let max_period = short.max(medium).max(long).max(close_period);
	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}
	if data_len < max_period as usize + 1 {
		return Err(StrategyError::Validation(format!(
			"Insufficient data for Ichimoku strategy: requires at least {} points, got {}",
			max_period + 1,
			data_len
		)));
	}

	// Calculate Ichimoku Cloud
	let ichimoku_config = indicators_core::IchimokuCloudConfig {
		short: Some(short),
		medium: Some(medium),
		long: Some(long),
		close: Some(close_period),
	};
	let ichimoku_result =
		indicators_core::ichimoku_cloud(highs, lows, closes, Some(ichimoku_config));

	// Calculate cloud top and bottom
	let mut cloud_top = Vec::with_capacity(data_len);
	let mut cloud_bottom = Vec::with_capacity(data_len);
	for i in 0..data_len {
		let ssa = ichimoku_result.ssa.get(i).copied().unwrap_or(0.0);
		let ssb = ichimoku_result.ssb.get(i).copied().unwrap_or(0.0);
		cloud_top.push(ssa.max(ssb));
		cloud_bottom.push(ssa.min(ssb));
	}

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < max_period as usize {
			0 // Not enough data
		} else if crossed_over_series(closes, &cloud_top, i as u32) {
			1 // Buy signal: price crosses above cloud top
		} else if crossed_under_series(closes, &cloud_bottom, i as u32) {
			-1 // Sell signal: price crosses below cloud bottom
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
