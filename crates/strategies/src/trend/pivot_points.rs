use crate::types::configs::PivotPointsConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Pivot Points Trend Strategy
///
/// Generates signals based on price vs pivot levels
/// Buy when price breaks above pivot, sell when price breaks below pivot
#[strategy(
	id = "pivotPoints",
	name = "Pivot Points Trend",
	category = "trend",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Generates signals based on price position relative to pivot levels",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn pivot_points_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<PivotPointsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let _period_high = config.period_high.unwrap_or(20); // Not used in current implementation
	let _period_low = config.period_low.unwrap_or(20); // Not used in current implementation

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Pivot Points period must be between 2 and 100".into(),
		));
	}
	let data_len = highs.len();
	if data_len < period as usize {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Pivot Points strategy".into(),
		));
	}

	// Generate signals based on pivot levels
	// For simplicity, calculate pivot as (H+L+C)/3 for each bar
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < (period as usize - 1) {
			signals.push(0); // Not enough data
			continue;
		}

		// Calculate pivot using current bar (simplified)
		let pivot = (highs[i] + lows[i] + closes[i]) / 3.0;

		let signal = if closes[i] > pivot {
			1 // Buy signal: price above pivot
		} else if closes[i] < pivot {
			-1 // Sell signal: price below pivot
		} else {
			0 // Hold: price at pivot
		};
		signals.push(signal);
	}

	Ok(signals)
}
