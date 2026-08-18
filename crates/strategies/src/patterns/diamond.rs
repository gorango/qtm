use crate::types::configs::DiamondConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Diamond Reversal Strategy
///
/// Detects diamond top/bottom reversal patterns (a broadening range that
/// contracts into a point before breaking out) and generates reversal signals:
/// diamond top breakdown (bearish) and diamond bottom breakout (bullish).
///
/// @strategy_id diamond_reversal
/// @strategy_name Diamond Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "diamond_reversal",
	name = "Diamond Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects diamond top and diamond bottom reversal patterns",
	opt_params = r#"[
		{"param_name": "minPoints", "min": 1, "max": 4, "step": 1},
		{"param_name": "tolerance", "min": 0.0, "max": 0.005, "step": 0.0001},
		{"param_name": "breakoutThreshold", "min": 0.0, "max": 0.02, "step": 0.001},
		{"param_name": "lookback", "min": 60, "max": 240, "step": 20}
	]"#
)]
pub fn diamond_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<DiamondConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(2);
	let tolerance = config.tolerance.unwrap_or(0.0005);
	let breakout_threshold = config.breakout_threshold.unwrap_or(0.0);
	let lookback = config.lookback.unwrap_or(150) as usize;

	let data_len = highs.len();
	if data_len < lookback + 10 {
		return Ok(vec![0; data_len]);
	}

	let bearish_signals = indicators_core::diamond_top(
		opens,
		highs,
		lows,
		closes,
		Some(min_points),
		Some(tolerance),
		Some(breakout_threshold),
		Some(lookback as u32),
	)?;

	let bullish_signals = indicators_core::diamond_bottom(
		opens,
		highs,
		lows,
		closes,
		Some(min_points),
		Some(tolerance),
		Some(breakout_threshold),
		Some(lookback as u32),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for i in 0..data_len {
		let signal = if bullish_signals[i] > 0.5 {
			1 // Bullish diamond bottom breakout
		} else if bearish_signals[i] < -0.5 {
			-1 // Bearish diamond top breakdown
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
		let result = diamond_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_diamond_top_breakdown() {
		let pivots = [
			(0, 100.0),
			(30, 102.0),
			(38, 99.0),
			(46, 106.0),
			(54, 95.0),
			(62, 104.0),
			(70, 97.0),
			(78, 101.0),
			(84, 98.5),
			(86, 100.0),
			(88, 96.0),
			(92, 90.0),
			(100, 88.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(DiamondConfig {
			lookback: Some(60),
			..Default::default()
		});
		let result = diamond_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx > 80, "signal should fire after the diamond completes, got {idx}");
	}

	#[test]
	fn detects_diamond_bottom_breakout() {
		let pivots = [
			(0, 100.0),
			(30, 102.0),
			(38, 99.0),
			(46, 106.0),
			(54, 95.0),
			(62, 104.0),
			(70, 97.0),
			(78, 101.0),
			(84, 98.5),
			(86, 99.0),
			(88, 100.2),
			(90, 104.0),
			(92, 108.0),
			(100, 112.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(DiamondConfig {
			lookback: Some(60),
			..Default::default()
		});
		let result = diamond_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx > 80, "signal should fire after the diamond completes, got {idx}");
	}
}
