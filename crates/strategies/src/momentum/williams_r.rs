use crate::types::configs::WilliamsRConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Williams %R Momentum Strategy
///
/// Generates buy signals when Williams %R crosses above oversold level
/// Generates sell signals when Williams %R crosses below overbought level
#[strategy(
	id = "williamsR",
	name = "Williams %R Momentum Strategy",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when Williams %R crosses above oversold level and sell signals when Williams %R crosses below overbought level",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "oversold", "min": -90.0, "max": -70.0, "step": 1.0},
		{"param_name": "overbought", "min": -30.0, "max": -10.0, "step": 1.0}
	]"#
)]
pub fn williams_r_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<WilliamsRConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(-80.0);
	let overbought = config.overbought.unwrap_or(-20.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Williams %R period must be between 2 and 100".into(),
		));
	}
	if !(-100.0..=0.0).contains(&oversold) || !(-100.0..=0.0).contains(&overbought) {
		return Err(StrategyError::Validation(
			"Williams %R thresholds must be between -100 and 0".into(),
		));
	}
	if oversold >= overbought {
		return Err(StrategyError::Validation(
			"Williams %R oversold must be less than overbought".into(),
		));
	}

	let data_len = highs.len();
	if data_len < (period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Williams %R strategy".into(),
		));
	}
	if lows.len() != data_len || closes.len() != data_len {
		return Err(StrategyError::Validation(
			"All price arrays must have the same length".into(),
		));
	}

	// Calculate Williams %R values
	let williams_config = indicators_core::WilliamsRConfig {
		period: Some(period),
	};
	let williams_values = indicators_core::williams_r(highs, lows, closes, Some(williams_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&williams_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&williams_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
