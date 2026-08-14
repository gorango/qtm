use crate::types::configs::CupAndHandleConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Cup and Handle Breakout Strategy
///
/// Detects the cup and handle continuation pattern, a bullish formation
/// consisting of a rounded bottom (cup) followed by a small pullback (handle)
///
/// @strategy_id cup_and_handle_breakout
/// @strategy_name Cup and Handle Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "cup_and_handle_breakout",
	name = "Cup and Handle Breakout Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects bullish continuation cup and handle pattern",
	opt_params = r#"[
		{"param_name": "cupDepth", "min": 0.05, "max": 0.3, "step": 0.01},
		{"param_name": "handleRetracement", "min": 0.1, "max": 0.5, "step": 0.05},
		{"param_name": "minDuration", "min": 10, "max": 50, "step": 5}
	]"#
)]
pub fn cup_and_handle_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<CupAndHandleConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let cup_depth = config.cup_depth.unwrap_or(0.15);
	let handle_retracement = config.handle_retracement.unwrap_or(0.3);
	let min_duration = config.min_duration.unwrap_or(20) as usize;

	let data_len = highs.len();

	if data_len < min_duration {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::cup_and_handle(
		opens,
		highs,
		lows,
		closes,
		Some(cup_depth),
		Some(handle_retracement),
		Some(min_duration as u32),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for &s in signals.iter().take(data_len) {
		let signal = if s > 0.5 {
			1 // Bullish breakout
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}
