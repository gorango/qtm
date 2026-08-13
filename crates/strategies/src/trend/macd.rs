use crate::types::configs::MACDConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// MACD Trend Strategy
///
/// Generates buy signals when MACD line crosses above signal line
/// Generates sell signals when MACD line crosses below signal line
#[strategy(
	id = "macd",
	name = "MACD Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when MACD line crosses above signal line and sell signals when MACD line crosses below signal line",
	opt_params = r#"[
		{"param_name": "fastPeriod", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "slowPeriod", "min": 20.0, "max": 50.0, "step": 1.0},
		{"param_name": "signalPeriod", "min": 5.0, "max": 20.0, "step": 1.0}
	]"#
)]
pub fn macd_strategy(closes: &[f64], config: Option<MACDConfig>) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(12);
	let slow_period = config.slow_period.unwrap_or(26);
	let signal_period = config.signal_period.unwrap_or(9);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err(StrategyError::Validation(
			"MACD fast period must be between 2 and 100".into(),
		));
	}
	if !(2..=200).contains(&slow_period) {
		return Err(StrategyError::Validation(
			"MACD slow period must be between 2 and 200".into(),
		));
	}
	if !(2..=50).contains(&signal_period) {
		return Err(StrategyError::Validation(
			"MACD signal period must be between 2 and 50".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = (slow_period + signal_period) as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for MACD strategy".into(),
		));
	}

	// Calculate MACD
	let macd_config = indicators_core::MACDConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
		signal_period: Some(signal_period),
	};
	let macd_result = indicators_core::macd(closes, Some(macd_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
