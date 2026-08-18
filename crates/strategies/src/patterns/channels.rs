use crate::types::configs::ChannelsConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Channels Breakout Strategy
///
/// Detects parallel price channels carried by alternating swings and generates
/// breakout signals when price pushes through a channel boundary: upside
/// breakout from a rising channel (bullish) or downside breakdown from a
/// falling channel (bearish).
///
/// @strategy_id channels_breakout
/// @strategy_name Channels Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "channels_breakout",
	name = "Channels Breakout Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects parallel price channels and generates breakout/breakdown signals",
	opt_params = r#"[
		{"param_name": "minPoints", "min": 2, "max": 6, "step": 1},
		{"param_name": "minSlope", "min": 0.0, "max": 0.005, "step": 0.0001},
		{"param_name": "parallelismTolerance", "min": 0.0, "max": 1.0, "step": 0.1},
		{"param_name": "lookback", "min": 60, "max": 240, "step": 20}
	]"#
)]
pub fn channels_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<ChannelsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(3);
	let min_slope = config.min_slope.unwrap_or(0.0005);
	let parallelism_tolerance = config.parallelism_tolerance.unwrap_or(0.5);
	let lookback = config.lookback.unwrap_or(120) as usize;

	let data_len = highs.len();
	if data_len < lookback + 5 {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::channels(
		opens,
		highs,
		lows,
		closes,
		Some(min_points),
		Some(min_slope),
		Some(parallelism_tolerance),
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
		let result = channels_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_rising_channel_breakout() {
		let pivots = [
			(0, 84.0),
			(10, 88.0),
			(20, 84.9),
			(30, 91.0),
			(40, 87.8),
			(50, 94.0),
			(58, 92.0),
			(61, 94.9),
			(62, 96.2),
			(64, 98.5),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(ChannelsConfig {
			lookback: Some(60),
			..Default::default()
		});
		let result = channels_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx >= 58, "signal should fire on the breakout, got {idx}");
	}

	#[test]
	fn detects_falling_channel_breakdown() {
		let pivots = [
			(0, 116.0),
			(10, 112.0),
			(20, 115.1),
			(30, 108.9),
			(40, 112.2),
			(50, 106.0),
			(58, 108.0),
			(61, 105.1),
			(62, 103.8),
			(64, 101.5),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(ChannelsConfig {
			lookback: Some(60),
			..Default::default()
		});
		let result = channels_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx >= 58, "signal should fire on the breakdown, got {idx}");
	}
}
