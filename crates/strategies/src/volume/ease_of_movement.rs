use crate::types::configs::EaseOfMovementConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Ease of Movement Strategy
///
/// Generates buy signals when EOM crosses above zero
/// Generates sell signals when EOM crosses below zero
#[strategy(
	id = "ease-of-movement",
	name = "Ease of Movement",
	category = "volume",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when EOM crosses above zero, sell signals when EOM crosses below zero",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn ease_of_movement_strategy(
	highs: &[f64],
	lows: &[f64],
	volumes: &[f64],
	config: Option<EaseOfMovementConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	let data_len = highs.len();
	if highs.len() != lows.len() || highs.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}
	if !(5..=50).contains(&period) {
		return Err(StrategyError::Validation(
			"Period must be between 5 and 50".into(),
		));
	}
	if data_len < (period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Ease of Movement strategy".into(),
		));
	}

	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let eom_values = indicators_core::ease_of_movement(&highs_vec, &lows_vec, &volumes_vec, period);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period as usize) {
			0
		} else if crossed_over(&eom_values, 0.0, i as u32) {
			1
		} else if crossed_under(&eom_values, 0.0, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
