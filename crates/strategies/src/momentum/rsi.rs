use crate::types::configs::RSIConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use indicators_core::validate_range;
use strategies_proc_macro::strategy;

/// RSI Momentum Strategy
///
/// Generates buy signals when RSI crosses above oversold level
/// Generates sell signals when RSI crosses below overbought level
#[strategy(
	id = "rsi",
	name = "RSI Momentum Strategy",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when RSI crosses above oversold level and sell signals when RSI crosses below overbought level",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "oversold", "min": 10.0, "max": 40.0, "step": 5.0},
		{"param_name": "overbought", "min": 60.0, "max": 90.0, "step": 5.0}
	]"#
)]
pub fn rsi_strategy(closes: &[f64], config: Option<RSIConfig>) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	// Validate parameters
	validate_range!(period, 2, 100, "RSI period", |msg| {
		StrategyError::Validation(msg)
	});
	validate_range!(oversold, 0.0, 100.0, "RSI oversold threshold", |msg| {
		StrategyError::Validation(msg)
	});
	validate_range!(overbought, 0.0, 100.0, "RSI overbought threshold", |msg| {
		StrategyError::Validation(msg)
	});
	let data_len = closes.len();
	if data_len < (period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for RSI strategy".into(),
		));
	}

	// Calculate RSI values
	let rsi_config = indicators_core::RSIConfig {
		period: Some(period),
	};
	let rsi_values = indicators_core::rsi(closes, Some(rsi_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&rsi_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&rsi_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
