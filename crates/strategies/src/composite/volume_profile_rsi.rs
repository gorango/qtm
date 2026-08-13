use crate::types::configs::VolumeProfileRsiConfig;
use crate::{StrategyError, StrategyResult};
use indicators_core::rsi;
use indicators_core::volume_profile;
use strategies_proc_macro::strategy;

#[strategy(
    id = "volume-profile-rsi",
    name = "Volume Profile + RSI",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Volume Profile + RSI",
    opt_params = r#"[{"param_name": "rsiPeriod", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "rsiOversold", "min": 10.0, "max": 40.0, "step": 5.0}, {"param_name": "rsiOverbought", "min": 60.0, "max": 90.0, "step": 5.0}, {"param_name": "volumeProfileBins", "min": 20.0, "max": 100.0, "step": 5.0}]"#
)]
pub fn volume_profile_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VolumeProfileRsiConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let rsi_period = config.rsi_period.unwrap_or(14);
	let rsi_oversold = config.rsi_oversold.unwrap_or(30.0);
	let rsi_overbought = config.rsi_overbought.unwrap_or(70.0);
	let volume_profile_bins = config.volume_profile_bins.unwrap_or(50);

	let min_data_length = (rsi_period + 1).max(2) as usize;

	if closes.len() < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: Volume Profile + RSI requires at least {} data points, got {}",
			min_data_length,
			closes.len()
		)));
	}

	let closes_vec = closes;
	let highs_vec = highs;
	let lows_vec = lows;
	let volumes_vec = volumes;

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = rsi(closes_vec, Some(rsi_config));

	let vp = volume_profile(highs_vec, lows_vec, volumes_vec, Some(volume_profile_bins));

	let mut sorted_volumes = vp.volumes.clone();
	sorted_volumes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
	let median_volume = sorted_volumes[sorted_volumes.len() / 2];

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, closes, and volumes arrays must have the same length".into(),
		));
	}
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_data_length {
			0
		} else {
			let current_close = closes[i];
			let mut bin_index = -1i32;

			for (idx, level) in vp.price_levels.iter().enumerate() {
				let next_level = if idx + 1 < vp.price_levels.len() {
					vp.price_levels[idx + 1]
				} else {
					level + (vp.price_levels[1] - vp.price_levels[0])
				};

				if current_close >= *level && current_close < next_level {
					bin_index = idx as i32;
					break;
				}
			}

			// Handle edge case where price equals the exact maximum price
			if bin_index == -1 {
				bin_index = (vp.price_levels.len() - 1) as i32;
			}

			if bin_index == -1 {
				0
			} else {
				let bin_volume = vp.volumes[bin_index as usize];
				let is_high_volume_area = bin_volume > median_volume;
				let is_low_volume_area = bin_volume < median_volume;
				let rsi_value = rsi_values[i];

				if is_high_volume_area && rsi_value < rsi_oversold {
					1
				} else if is_low_volume_area && rsi_value > rsi_overbought {
					-1
				} else {
					0
				}
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
