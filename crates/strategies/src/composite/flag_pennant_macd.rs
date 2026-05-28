use crate::types::configs::FlagsPennantsConfig;
use crate::types::configs::MACDConfig;

/// Flag Pennant Macd
///
/// Buy on flag/pennant breakout with MACD bullish crossover.
pub fn flag_pennant_macd_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	fp_config: Option<FlagsPennantsConfig>,
	macd_config: Option<MACDConfig>,
) -> Result<Vec<i8>, String> {
	let fp_cfg = fp_config.unwrap_or_default();
	let macd_cfg = macd_config.unwrap_or_default();

	let pole_length = fp_cfg.pole_length.unwrap_or(10);
	let consolidation_bars = fp_cfg.consolidation_bars.unwrap_or(10);
	let breakout_threshold = fp_cfg.breakout_threshold.unwrap_or(0.02);

	let fast_period = macd_cfg.fast_period.unwrap_or(12);
	let slow_period = macd_cfg.slow_period.unwrap_or(26);
	let signal_period = macd_cfg.signal_period.unwrap_or(9);

	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let opens_vec: Vec<f64> = closes_vec.clone();
	let flag_pennant_signals = indicators_core::flags_pennants(
		&opens_vec,
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(pole_length),
		Some(consolidation_bars),
		Some(breakout_threshold),
	)?;

	let macd_cfg_ind = indicators_core::MACDConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
		signal_period: Some(signal_period),
	};
	let macd_result = indicators_core::macd(&closes_vec, Some(macd_cfg_ind))?;

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len {
		return Err("Highs, lows, and closes arrays must have the same length".to_string());
	}
	let min_data_length = (slow_period + signal_period) as usize;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_data_length {
			0
		} else {
			let flag_pennant = &flag_pennant_signals;
			if flag_pennant[i] == 1.0 && macd_result.histogram[i] > 0.0 {
				1
			} else if flag_pennant[i] == -1.0 && macd_result.histogram[i] < 0.0 {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn flag_pennant_macd_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "flag-pennant-macd-continuation",
		"name": "Flag/Pennant + MACD Continuation",
		"category": "composite",
		"description": "Flag/Pennant + MACD continuation",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn flag_pennant_macd_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"pole_length": 10,
			"consolidation_bars": 10,
			"breakout_threshold": 0.02,
			"fast_period": 12,
			"slow_period": 26,
			"signal_period": 9
		},
		"optimization_bounds": [
			{
				"param_name": "pole_length",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "fast_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "slow_period",
				"min": 20.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "signal_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			}
		]
	})
}
