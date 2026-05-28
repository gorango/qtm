use crate::types::configs::VolumePriceTrendConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Volume Price Trend Strategy
///
/// Generates buy signals when VPT crosses above positive threshold with volume confirmation
/// Generates sell signals when VPT crosses below negative threshold with volume confirmation
#[strategy(
	id = "volume-price-trend",
	name = "Volume Price Trend",
	category = "volume",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when VPT crosses above positive threshold with volume confirmation, sell signals when VPT crosses below negative threshold with volume confirmation",
	opt_params = r#"[
		{"param_name": "vpt_threshold", "min": 0.01, "max": 0.5, "step": 0.01}
	]"#
)]
pub fn volume_price_trend_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<VolumePriceTrendConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_criteria_met = config.min_criteria_met.unwrap_or(1);
	let vpt_threshold = config.vpt_threshold.unwrap_or(0.1);

	if closes.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"Closes and volumes must have equal length".into(),
		));
	}
	if !(1..=2).contains(&min_criteria_met) {
		return Err(StrategyError::Validation(
			"minCriteriaMet must be between 1 and 2".into(),
		));
	}
	if !(0.0..=1.0).contains(&vpt_threshold) {
		return Err(StrategyError::Validation(
			"VPT threshold must be between 0 and 1".into(),
		));
	}
	let data_len = closes.len();
	if data_len < 50 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for VPT strategy (need at least 50 bars)".into(),
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();
	let vpt_values = indicators_core::volume_price_trend(&closes_vec, &volumes_vec);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < 50 {
			0
		} else {
			let mut criteria_met = 0u32;

			if crossed_over(&vpt_values, vpt_threshold, i as u32) {
				criteria_met += 1;
			}
			if crossed_under(&vpt_values, -vpt_threshold, i as u32) {
				criteria_met += 1;
			}

			let current_volume = volumes[i];
			let avg_volume: f64 = volumes_vec[(i - 49)..i].iter().sum::<f64>() / 49.0;
			if current_volume > avg_volume {
				criteria_met += 1;
			}

			if criteria_met >= min_criteria_met {
				if crossed_over(&vpt_values, vpt_threshold, i as u32) {
					1
				} else if crossed_under(&vpt_values, -vpt_threshold, i as u32) {
					-1
				} else {
					0
				}
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
