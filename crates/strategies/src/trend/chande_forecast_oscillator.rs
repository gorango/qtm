use crate::types::configs::ChandeForecastOscillatorConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};

/// Chande Forecast Oscillator Trend Strategy
///
/// Generates buy signals when CFO crosses under oversold level
/// Generates sell signals when CFO crosses over overbought level
///
/// @strategy_id chandeForecastOscillator
/// @strategy_name Chande Forecast Oscillator Trend
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn chande_forecast_oscillator_strategy(
	closes: &[f64],
	config: Option<ChandeForecastOscillatorConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let overbought = config.overbought.unwrap_or(70.0);
	let oversold = config.oversold.unwrap_or(30.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"CFO period must be between 2 and 100".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Chande Forecast Oscillator strategy".into(),
		));
	}

	// Calculate CFO
	let cfo_result = indicators_core::moving_chande_forecast_oscillator(closes, Some(period))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_under(&cfo_result, oversold, i as u32) {
			1 // Buy signal: CFO crosses under oversold
		} else if crossed_over(&cfo_result, overbought, i as u32) {
			-1 // Sell signal: CFO crosses over overbought
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Chande Forecast Oscillator strategy metadata for registry
pub fn chande_forecast_oscillator_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "chandeForecastOscillator",
		"name": "Chande Forecast Oscillator Trend",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when CFO crosses under oversold level and sell signals when CFO crosses over overbought level"
	})
}

/// Get Chande Forecast Oscillator strategy default parameters
pub fn chande_forecast_oscillator_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"overbought": 70.0,
			"oversold": 30.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 90.0,
				"step": 5.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 40.0,
				"step": 5.0
			}
		]
	})
}
