use crate::{StrategyError, StrategyResult};

/// Elliott Wave Pattern Strategy
///
/// Generates buy signals for impulse waves (1, 2)
/// Generates sell signals for corrective waves (-1, -2)
///
/// @strategy_id elliott-wave-pattern
/// @strategy_name Elliott Wave Pattern Strategy
/// @category patterns
/// @default_timeframes 1h,4h,1d
#[allow(clippy::too_many_arguments)]
pub fn elliott_wave_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	min_wave_separation: u32,
	lookaround: u32,
	retracement_tolerance: f64,
) -> StrategyResult<Vec<i8>> {
	let data_len = closes.len();
	if opens.len() != data_len || highs.len() != data_len || lows.len() != data_len {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}
	if !(0.3..=1.0).contains(&wave2_retracement) || !(0.2..=0.8).contains(&wave4_retracement) {
		return Err(StrategyError::Validation(
			"Retracement values out of range".into(),
		));
	}
	if !(1.0..=3.0).contains(&wave3_min_extension) {
		return Err(StrategyError::Validation(
			"Wave 3 extension out of range".into(),
		));
	}
	if !(2..=20).contains(&min_wave_separation) || !(1..=5).contains(&lookaround) {
		return Err(StrategyError::Validation(
			"Wave separation or lookaround out of range".into(),
		));
	}
	if !(0.05..=0.3).contains(&retracement_tolerance) {
		return Err(StrategyError::Validation(
			"Retracement tolerance out of range".into(),
		));
	}

	// Calculate Elliott Wave
	let wave_signals = indicators_core::elliott_wave(
		opens,
		highs,
		lows,
		closes,
		Some(wave2_retracement),
		Some(wave4_retracement),
		Some(wave3_min_extension),
		Some(min_wave_separation),
		Some(lookaround),
		Some(retracement_tolerance),
	)?;

	// Convert wave signals to strategy signals
	let mut signals = Vec::with_capacity(data_len);
	for wave in wave_signals {
		let signal = match wave.round() as i32 {
			1 | 2 => 1,    // Buy for impulse waves
			-1 | -2 => -1, // Sell for corrective waves
			_ => 0,
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Elliott Wave strategy metadata for registry
pub fn elliott_wave_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "elliott-wave-pattern",
		"name": "Elliott Wave Pattern Strategy",
		"category": "patterns",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals for impulse waves and sell signals for corrective waves"
	})
}

/// Get Elliott Wave strategy default parameters
pub fn elliott_wave_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"wave2_retracement": 0.618,
			"wave4_retracement": 0.382,
			"wave3_min_extension": 1.618,
			"min_wave_separation": 5,
			"lookaround": 2,
			"retracement_tolerance": 0.1
		},
		"optimization_bounds": [
			{
				"param_name": "wave2_retracement",
				"min": 0.3,
				"max": 1.0,
				"step": 0.05
			},
			{
				"param_name": "wave4_retracement",
				"min": 0.2,
				"max": 0.8,
				"step": 0.05
			},
			{
				"param_name": "wave3_min_extension",
				"min": 1.0,
				"max": 3.0,
				"step": 0.1
			},
			{
				"param_name": "min_wave_separation",
				"min": 2.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "lookaround",
				"min": 1.0,
				"max": 5.0,
				"step": 1.0
			},
			{
				"param_name": "retracement_tolerance",
				"min": 0.05,
				"max": 0.3,
				"step": 0.025
			}
		]
	})
}
