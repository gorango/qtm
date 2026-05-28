use crate::types::configs::WmaMomentumConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// WMA Momentum Trend Strategy
///
/// Generates buy signals when WMA is increasing
/// Generates sell signals when WMA is decreasing
#[strategy(
	id = "wmaMomentum",
	name = "WMA Momentum Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when WMA is increasing and sell signals when WMA is decreasing",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn wma_momentum_strategy(
	closes: &[f64],
	config: Option<WmaMomentumConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"WMA period must be between 2 and 100".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for WMA Momentum strategy".into(),
		));
	}

	// Calculate WMA
	let wma_result = indicators_core::wma(closes, Some(period))?;

	// Generate signals based on WMA momentum (increasing/decreasing)
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods || i == 0 {
			0 // Not enough data
		} else if wma_result[i] > wma_result[i - 1] {
			1 // Buy signal: WMA increasing
		} else if wma_result[i] < wma_result[i - 1] {
			-1 // Sell signal: WMA decreasing
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
