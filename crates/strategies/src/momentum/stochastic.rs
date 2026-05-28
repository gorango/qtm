use crate::types::configs::StochasticConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Stochastic Oscillator Momentum Strategy
///
/// Generates buy signals when %K crosses above oversold level
/// Generates sell signals when %K crosses below overbought level
#[strategy(
	id = "stochastic",
	name = "Stochastic Oscillator Momentum",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when %K crosses above oversold level and sell signals when %K crosses below overbought level",
	opt_params = r#"[
		{"param_name": "k_period", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "d_period", "min": 2.0, "max": 10.0, "step": 1.0},
		{"param_name": "oversold", "min": 10.0, "max": 30.0, "step": 1.0},
		{"param_name": "overbought", "min": 70.0, "max": 90.0, "step": 1.0}
	]"#
)]
pub fn stochastic_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<StochasticConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let k_period = config.k_period.unwrap_or(14);
	let d_period = config.d_period.unwrap_or(3);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	// Validate parameters
	if !(2..=100).contains(&k_period) {
		return Err(StrategyError::Validation(
			"Stochastic K period must be between 2 and 100".into(),
		));
	}
	if !(2..=50).contains(&d_period) {
		return Err(StrategyError::Validation(
			"Stochastic D period must be between 2 and 50".into(),
		));
	}
	if !(0.0..=100.0).contains(&oversold) || !(0.0..=100.0).contains(&overbought) {
		return Err(StrategyError::Validation(
			"Stochastic thresholds must be between 0 and 100".into(),
		));
	}

	let min_data_length = (k_period + d_period + 1) as usize;
	let data_len = highs.len();
	if data_len < min_data_length || lows.len() < min_data_length || closes.len() < min_data_length
	{
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Stochastic strategy".into(),
		));
	}

	// Calculate Stochastic values
	let stochastic_config = indicators_core::StochConfig {
		k_period: Some(k_period),
		d_period: Some(d_period),
	};
	let stochastic_result =
		indicators_core::stochastic_oscillator(highs, lows, closes, Some(stochastic_config));

	// Generate signals based on %K line
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&stochastic_result.k, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&stochastic_result.k, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
