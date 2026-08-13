use crate::types::configs::AwesomeOscillatorConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Awesome Oscillator Momentum Strategy
///
/// Generates buy signals when AO crosses above zero
/// Generates sell signals when AO crosses below zero
#[strategy(
	id = "awesomeOscillator",
	name = "Awesome Oscillator Momentum Strategy",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when AO crosses above zero and sell signals when AO crosses below zero",
	opt_params = r#"[
		{"param_name": "fastPeriod", "min": 3.0, "max": 10.0, "step": 1.0},
		{"param_name": "slowPeriod", "min": 20.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn awesome_oscillator_strategy(
	highs: &[f64],
	lows: &[f64],
	config: Option<AwesomeOscillatorConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(5);
	let slow_period = config.slow_period.unwrap_or(34);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err(StrategyError::Validation(
			"Awesome Oscillator fast period must be between 2 and 100".into(),
		));
	}
	if !(5..=200).contains(&slow_period) {
		return Err(StrategyError::Validation(
			"Awesome Oscillator slow period must be between 5 and 200".into(),
		));
	}
	if fast_period >= slow_period {
		return Err(StrategyError::Validation(
			"Awesome Oscillator fast period must be less than slow period".into(),
		));
	}

	let data_len = highs.len();
	if data_len < (slow_period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Awesome Oscillator strategy".into(),
		));
	}
	if lows.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs and lows arrays must have the same length".into(),
		));
	}

	// Calculate Awesome Oscillator values
	let ao_config = indicators_core::AwesomeOscillatorConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
	};
	let ao_values = indicators_core::awesome_oscillator(highs, lows, Some(ao_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);
	let zero = 0.0;

	for i in 0..data_len {
		let signal = if crossed_over(&ao_values, zero, i as u32) {
			1 // Buy signal
		} else if crossed_under(&ao_values, zero, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
