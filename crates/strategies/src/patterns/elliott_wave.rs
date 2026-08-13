use crate::types::configs::ElliottWaveConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Elliott Wave Pattern Strategy
///
/// Generates buy signals for impulse waves (1, 2)
/// Generates sell signals for corrective waves (-1, -2)
#[strategy(
	id = "elliott-wave-pattern",
	name = "Elliott Wave Pattern Strategy",
	category = "patterns",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Generates buy signals for impulse waves and sell signals for corrective waves",
	opt_params = r#"[
		{"param_name": "wave2Retracement", "min": 0.3, "max": 1.0, "step": 0.05},
		{"param_name": "wave4Retracement", "min": 0.2, "max": 0.8, "step": 0.05},
		{"param_name": "wave3MinExtension", "min": 1.0, "max": 3.0, "step": 0.1},
		{"param_name": "minWaveSeparation", "min": 2.0, "max": 20.0, "step": 1.0},
		{"param_name": "lookaround", "min": 1.0, "max": 5.0, "step": 1.0},
		{"param_name": "retracementTolerance", "min": 0.05, "max": 0.3, "step": 0.025}
	]"#
)]
pub fn elliott_wave_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<ElliottWaveConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let wave2_retracement = config.wave2_retracement.unwrap_or(0.618);
	let wave4_retracement = config.wave4_retracement.unwrap_or(0.382);
	let wave3_min_extension = config.wave3_min_extension.unwrap_or(1.618);
	let min_wave_separation = config.min_wave_separation.unwrap_or(5);
	let lookaround = config.lookaround.unwrap_or(2);
	let retracement_tolerance = config.retracement_tolerance.unwrap_or(0.1);

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
