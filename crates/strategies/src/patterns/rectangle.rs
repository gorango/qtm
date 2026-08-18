use crate::types::configs::RectangleConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Rectangle Breakout Strategy
///
/// Detects horizontal-range (rectangle) continuation patterns preceded by a
/// trend and generates breakout signals: upside breakout after an uptrend
/// (bullish) or downside breakdown after a downtrend (bearish).
///
/// @strategy_id rectangle_breakout
/// @strategy_name Rectangle Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "rectangle_breakout",
	name = "Rectangle Breakout Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects rectangle continuation patterns and generates breakout/breakdown signals",
	opt_params = r#"[
		{"param_name": "minPoints", "min": 2, "max": 6, "step": 1},
		{"param_name": "slopeTolerance", "min": 0.0, "max": 0.005, "step": 0.0001},
		{"param_name": "minSpread", "min": 0.0, "max": 0.05, "step": 0.005},
		{"param_name": "lookback", "min": 60, "max": 240, "step": 20},
		{"param_name": "trendBars", "min": 10, "max": 50, "step": 5},
		{"param_name": "minTrend", "min": 0.01, "max": 0.1, "step": 0.01}
	]"#
)]
pub fn rectangle_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RectangleConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(3);
	let slope_tolerance = config.slope_tolerance.unwrap_or(0.0002);
	let min_spread = config.min_spread.unwrap_or(0.01);
	let lookback = config.lookback.unwrap_or(120) as usize;
	let trend_bars = config.trend_bars.unwrap_or(30) as usize;
	let min_trend = config.min_trend.unwrap_or(0.03);

	let data_len = highs.len();
	if data_len < lookback + trend_bars + 5 {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::rectangle(
		opens,
		highs,
		lows,
		closes,
		Some(min_points),
		Some(slope_tolerance),
		Some(min_spread),
		Some(lookback as u32),
		Some(trend_bars as u32),
		Some(min_trend),
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
		let result = rectangle_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_bullish_rectangle_breakout() {
		let pivots = [
			(0, 80.0),
			(15, 100.0),
			(28, 98.0),
			(36, 102.0),
			(44, 98.1),
			(52, 101.8),
			(60, 98.2),
			(68, 102.0),
			(76, 98.3),
			(80, 105.0),
			(90, 109.0),
		];
		let closes = series_from_pivots(&pivots, 100);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(RectangleConfig {
			min_points: Some(3),
			lookback: Some(60),
			trend_bars: Some(15),
			..Default::default()
		});
		let result = rectangle_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx >= 75, "signal should fire on the breakout, got {idx}");
	}

	#[test]
	fn detects_bearish_rectangle_breakdown() {
		let pivots = [
			(0, 120.0),
			(15, 100.0),
			(28, 102.0),
			(36, 98.0),
			(44, 101.9),
			(52, 98.2),
			(60, 101.8),
			(68, 98.0),
			(76, 101.7),
			(80, 95.0),
			(90, 91.0),
		];
		let closes = series_from_pivots(&pivots, 100);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(RectangleConfig {
			min_points: Some(3),
			lookback: Some(60),
			trend_bars: Some(15),
			..Default::default()
		});
		let result = rectangle_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx >= 75, "signal should fire on the breakdown, got {idx}");
	}
}
