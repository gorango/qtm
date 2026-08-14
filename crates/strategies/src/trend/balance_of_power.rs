use crate::types::configs::BalanceOfPowerConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Balance of Power Trend Strategy
///
/// Generates buy signals when BOP crosses over zero
/// Generates sell signals when BOP crosses under zero
#[strategy(
	id = "balance_of_power",
	name = "Balance of Power Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when Balance of Power crosses over zero and sell signals when Balance of Power crosses under zero"
)]
pub fn balance_of_power_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<BalanceOfPowerConfig>,
) -> StrategyResult<Vec<i8>> {
	let _config = config.unwrap_or_default(); // Period not used in current implementation

	let data_len = opens.len();
	if data_len == 0 {
		return Err(StrategyError::Validation(
			"Input arrays cannot be empty".into(),
		));
	}

	// Calculate Balance of Power
	let bop_result = indicators_core::balance_of_power(opens, highs, lows, closes)?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&bop_result, 0.0, i as u32) {
			1 // Buy signal: BOP crosses over 0
		} else if crossed_under(&bop_result, 0.0, i as u32) {
			-1 // Sell signal: BOP crosses under 0
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
