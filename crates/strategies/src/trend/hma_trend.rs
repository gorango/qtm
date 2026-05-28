use crate::types::configs::HmaTrendConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// HMA Trend Strategy
///
/// Generates signals based on HMA slope direction
/// Buy when HMA is rising, sell when HMA is falling
#[strategy(
	id = "hmaTrend",
	name = "HMA Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when HMA is rising and sell signals when HMA is falling",
	opt_params = r#"[
		{"param_name": "period", "min": 10.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn hma_trend_strategy(
	closes: &[f64],
	config: Option<HmaTrendConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(21);

	// Validate parameters
	if !(2..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"HMA period must be between 2 and 200".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods + 1 {
		// Need extra point for slope calculation
		return Err(StrategyError::InsufficientData(
			"Insufficient data for HMA Trend strategy".into(),
		));
	}

	// Calculate HMA
	let hma_result = indicators_core::hma(closes, Some(period))?;

	// Generate signals based on slope
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if hma_result[i] > hma_result[i - 1] {
			1 // Buy signal: HMA rising
		} else if hma_result[i] < hma_result[i - 1] {
			-1 // Sell signal: HMA falling
		} else {
			0 // Hold: HMA flat
		};
		signals.push(signal);
	}

	Ok(signals)
}
