use crate::types::configs::CciConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// CCI Momentum Strategy
///
/// Generates buy signals when CCI crosses below oversold level
/// Generates sell signals when CCI crosses above overbought level
#[strategy(
	id = "cci",
	name = "CCI Momentum Strategy",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when CCI crosses below oversold level and sell signals when CCI crosses above overbought level",
	opt_params = r#"[
		{"param_name": "period", "min": 10.0, "max": 30.0, "step": 1.0},
		{"param_name": "oversold", "min": -150.0, "max": -80.0, "step": 10.0},
		{"param_name": "overbought", "min": 80.0, "max": 150.0, "step": 10.0}
	]"#
)]
pub fn cci_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<CciConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let oversold = config.oversold.unwrap_or(-100.0);
	let overbought = config.overbought.unwrap_or(100.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"CCI period must be between 2 and 100".into(),
		));
	}
	if oversold >= overbought {
		return Err(StrategyError::Validation(
			"CCI oversold must be less than overbought".into(),
		));
	}

	let data_len = highs.len();
	if data_len < (period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for CCI strategy".into(),
		));
	}
	if lows.len() != data_len || closes.len() != data_len {
		return Err(StrategyError::Validation(
			"All price arrays must have the same length".into(),
		));
	}

	// Calculate CCI values
	let cci_config = indicators_core::CCIConfig {
		period: Some(period),
	};
	let cci_values = indicators_core::cci(highs, lows, closes, Some(cci_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_under(&cci_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_over(&cci_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
