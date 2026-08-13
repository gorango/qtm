use crate::types::configs::VwapBreakoutConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// VWAP Breakout Strategy
///
/// Generates buy signals when price crosses above VWAP + breakout threshold
/// Generates sell signals when price crosses below VWAP
#[strategy(
	id = "vwap-breakout",
	name = "VWAP Breakout",
	category = "volume",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when price crosses above VWAP + breakout threshold, sell signals when price crosses below VWAP",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "breakoutThreshold", "min": 0.005, "max": 0.1, "step": 0.005}
	]"#
)]
pub fn vwap_breakout_strategy(
	closes: &[f64],
	highs: &[f64],
	lows: &[f64],
	volumes: &[f64],
	config: Option<VwapBreakoutConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let breakout_threshold = config.breakout_threshold.unwrap_or(0.01);

	let data_len = closes.len();
	if closes.len() != highs.len() || closes.len() != lows.len() || closes.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}
	if !(5..=50).contains(&period) {
		return Err(StrategyError::Validation(
			"Period must be between 5 and 50".into(),
		));
	}
	if !(0.001..=0.1).contains(&breakout_threshold) {
		return Err(StrategyError::Validation(
			"Breakout threshold must be between 0.001 and 0.1".into(),
		));
	}
	if data_len < (period as usize) + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for VWAP Breakout strategy".into(),
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let vwap_config = indicators_core::VWAPConfig {
		period: Some(period),
		anchored: Some(true),
		session_length: Some(1440), // 24 hours in minutes
		price_source: Some("hlc3".to_string()),
	};
	let vwap_values = indicators_core::vwap(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		&volumes_vec,
		Some(vwap_config),
	);

	let mut signals = Vec::with_capacity(data_len);

	for (i, &vwap_value) in vwap_values.iter().enumerate().take(data_len) {
		let signal = if i < (period as usize) {
			0
		} else {
			let breakout_level = vwap_value * (1.0 + breakout_threshold);

			if crossed_over(&closes_vec, breakout_level, i as u32) {
				1
			} else if crossed_under(&closes_vec, vwap_value, i as u32) {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
