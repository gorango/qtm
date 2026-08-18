use crate::types::configs::RoundingConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Rounding Reversal Strategy
///
/// Detects rounding bottom (saucer) and rounding top (dome) reversal patterns
/// and generates reversal signals once the curved base completes: upside
/// breakout after a rounding bottom (bullish) and downside breakdown after a
/// rounding top (bearish).
///
/// @strategy_id rounding_reversal
/// @strategy_name Rounding Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "rounding_reversal",
	name = "Rounding Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects rounding bottom (saucer) and rounding top (dome) reversal patterns",
	opt_params = r#"[
		{"param_name": "curvatureTolerance", "min": 0.0, "max": 0.05, "step": 0.005},
		{"param_name": "lookback", "min": 40, "max": 200, "step": 20}
	]"#
)]
pub fn rounding_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RoundingConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let curvature_tolerance = config.curvature_tolerance.unwrap_or(0.01);
	let lookback = config.lookback.unwrap_or(120) as usize;

	let data_len = highs.len();
	if data_len < lookback + 2 {
		return Ok(vec![0; data_len]);
	}

	let bullish_signals = indicators_core::rounding_bottom(
		opens,
		highs,
		lows,
		closes,
		Some(curvature_tolerance),
		Some(lookback as u32),
	)?;

	let bearish_signals = indicators_core::rounding_top(
		opens,
		highs,
		lows,
		closes,
		Some(curvature_tolerance),
		Some(lookback as u32),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for i in 0..data_len {
		let signal = if bullish_signals[i] > 0.5 {
			1 // Bullish rounding bottom breakout
		} else if bearish_signals[i] < -0.5 {
			-1 // Bearish rounding top breakdown
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
		let result = rounding_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_rounding_bottom_breakout() {
		let pivots = [
			(0, 100.0),
			(15, 100.0),
			(35, 96.0),
			(55, 92.0),
			(75, 96.0),
			(85, 100.0),
			(95, 106.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(RoundingConfig {
			lookback: Some(70),
			..Default::default()
		});
		let result = rounding_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx >= 80, "signal should fire after the saucer completes, got {idx}");
	}

	#[test]
	fn detects_rounding_top_breakdown() {
		let pivots = [
			(0, 100.0),
			(15, 100.0),
			(35, 104.0),
			(55, 108.0),
			(75, 104.0),
			(85, 100.0),
			(95, 95.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(RoundingConfig {
			lookback: Some(70),
			..Default::default()
		});
		let result = rounding_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx >= 80, "signal should fire after the rounding top completes, got {idx}");
	}
}
