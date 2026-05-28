use crate::types::configs::FlagsPennantsConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Flags and Pennants Continuation Strategy
///
/// Detects flag and pennant continuation patterns
/// Flag: Consolidation with parallel lines
/// Pennant: Consolidation with converging lines
///
/// @strategy_id flags-pennants-continuation
/// @strategy_name Flags and Pennants Continuation Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "flags-pennants-continuation",
	name = "Flags and Pennants Continuation Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects flag and pennant continuation patterns",
	opt_params = r#"[
		{"param_name": "poleLength", "min": 5, "max": 50, "step": 5},
		{"param_name": "consolidationBars", "min": 5, "max": 50, "step": 5},
		{"param_name": "breakoutThreshold", "min": 0.0, "max": 0.1, "step": 0.005},
		{"param_name": "additionalBuffer", "min": 1, "max": 20, "step": 1}
	]"#
)]
pub fn flags_pennants_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<FlagsPennantsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let pole_length = config.pole_length.unwrap_or(10) as usize;
	let consolidation_bars = config.consolidation_bars.unwrap_or(10) as usize;
	let breakout_threshold = config.breakout_threshold.unwrap_or(0.02);
	let additional_buffer = config.additional_buffer.unwrap_or(5) as usize;

	let data_len = highs.len();

	if data_len < pole_length + consolidation_bars + additional_buffer {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::flags_pennants(
		opens,
		highs,
		lows,
		closes,
		Some(pole_length as u32),
		Some(consolidation_bars as u32),
		Some(breakout_threshold),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for &s in signals.iter().take(data_len) {
		let signal = if s > 0.5 {
			1 // Bullish breakout
		} else if s < -0.5 {
			-1 // Bearish breakout
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}
