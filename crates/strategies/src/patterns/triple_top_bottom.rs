use crate::types::configs::TripleTopBottomConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Triple Top/Bottom Reversal Strategy
///
/// Detects triple top (three peaks at similar levels followed by a breakdown)
/// and triple bottom (three troughs followed by a breakout) reversal patterns.
///
/// @strategy_id triple_top_bottom_reversal
/// @strategy_name Triple Top/Bottom Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "triple_top_bottom_reversal",
	name = "Triple Top/Bottom Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects triple top and triple bottom reversal patterns",
	opt_params = r#"[
		{"param_name": "tolerance", "min": 0.0, "max": 0.1, "step": 0.005},
		{"param_name": "minSeparation", "min": 3, "max": 20, "step": 1},
		{"param_name": "lookaround", "min": 1, "max": 5, "step": 1}
	]"#
)]
pub fn triple_top_bottom_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TripleTopBottomConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let tolerance = config.tolerance.unwrap_or(0.03);
	let min_separation = config.min_separation.unwrap_or(8);
	let lookaround = config.lookaround.unwrap_or(2);

	let data_len = highs.len();

	let bearish_signals = indicators_core::triple_top(
		opens,
		highs,
		lows,
		closes,
		Some(tolerance),
		Some(min_separation),
		Some(lookaround),
	)?;

	let bullish_signals = indicators_core::triple_bottom(
		opens,
		highs,
		lows,
		closes,
		Some(tolerance),
		Some(min_separation),
		Some(lookaround),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for i in 0..data_len {
		let signal = if bullish_signals[i] > 0.5 {
			1 // Bullish triple bottom breakout
		} else if bearish_signals[i] < -0.5 {
			-1 // Bearish triple top breakdown
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::test_util::*;

	#[test]
	fn insufficient_data_returns_zeros() {
		let (opens, highs, lows, closes) = ohlc_from_series(&[100.0; 10]);
		let result = triple_top_bottom_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_triple_top_breakdown() {
		let pivots = [
			(0, 96.0),
			(10, 100.0),
			(20, 97.0),
			(30, 100.5),
			(40, 96.5),
			(50, 100.0),
			(60, 93.0),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(TripleTopBottomConfig {
			min_separation: Some(5),
			..Default::default()
		});
		let result = triple_top_bottom_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx > 50, "signal should fire after the third peak, got {idx}");
	}

	#[test]
	fn detects_triple_bottom_breakout() {
		let pivots = [
			(0, 104.0),
			(10, 100.0),
			(20, 103.0),
			(30, 99.5),
			(40, 103.5),
			(50, 100.0),
			(60, 107.0),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(TripleTopBottomConfig {
			min_separation: Some(5),
			..Default::default()
		});
		let result = triple_top_bottom_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx > 50, "signal should fire after the third trough, got {idx}");
	}
}
