use crate::types::configs::BumpAndRunConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Bump and Run Reversal Strategy
///
/// Detects bump and run reversals: a steep lead-in trend, an overshoot "bump"
/// beyond the trend line, and a return back through the extrapolated line
/// that marks the reversal.
///
/// @strategy_id bump_and_run_reversal
/// @strategy_name Bump and Run Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "bump_and_run_reversal",
	name = "Bump and Run Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects bump and run reversals and generates signals when price returns through the trend line",
	opt_params = r#"[
		{"param_name": "leadInBars", "min": 10, "max": 40, "step": 5},
		{"param_name": "minSlope", "min": 0.0, "max": 0.01, "step": 0.0005},
		{"param_name": "bumpThreshold", "min": 0.01, "max": 0.1, "step": 0.01},
		{"param_name": "lookback", "min": 40, "max": 160, "step": 20}
	]"#
)]
pub fn bump_and_run_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<BumpAndRunConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let lead_in_bars = config.lead_in_bars.unwrap_or(20);
	let min_slope = config.min_slope.unwrap_or(0.001);
	let bump_threshold = config.bump_threshold.unwrap_or(0.03);
	let lookback = config.lookback.unwrap_or(80) as usize;

	let data_len = highs.len();
	if data_len < lookback + 5 || lookback <= lead_in_bars as usize + 5 {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::bump_and_run(
		opens,
		highs,
		lows,
		closes,
		Some(lead_in_bars),
		Some(min_slope),
		Some(bump_threshold),
		Some(lookback as u32),
	)?;

	Ok(signals
		.iter()
		.map(|&s| {
			if s > 0.5 {
				1
			} else if s < -0.5 {
				-1
			} else {
				0
			}
		})
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::test_util::*;

	#[test]
	fn insufficient_data_returns_zeros() {
		let (opens, highs, lows, closes) = ohlc_from_series(&[100.0; 10]);
		let result = bump_and_run_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_bullish_bump_and_run() {
		let pivots = [
			(0, 100.0),
			(20, 95.0),
			(35, 78.0),
			(48, 86.0),
			(50, 89.0),
			(60, 94.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(BumpAndRunConfig {
			lead_in_bars: Some(20),
			lookback: Some(50),
			..Default::default()
		});
		let result = bump_and_run_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert_eq!(idx, 50, "signal should fire at the return crossing");
	}

	#[test]
	fn detects_bearish_bump_and_run() {
		let pivots = [
			(0, 100.0),
			(20, 105.0),
			(35, 122.0),
			(48, 114.0),
			(50, 111.0),
			(60, 106.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(BumpAndRunConfig {
			lead_in_bars: Some(20),
			lookback: Some(50),
			..Default::default()
		});
		let result = bump_and_run_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert_eq!(idx, 50, "signal should fire at the return crossing");
	}
}
