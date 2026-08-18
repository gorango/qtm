use crate::types::configs::BroadeningConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Broadening Formation Breakout Strategy
///
/// Detects broadening (expanding range) formations and generates breakout
/// signals: successive higher highs and lower lows, confirmed when price
/// breaks through one of the diverging boundary lines.
///
/// @strategy_id broadening_breakout
/// @strategy_name Broadening Formation Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "broadening_breakout",
	name = "Broadening Formation Breakout Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects broadening (expanding range) formations and generates breakout signals",
	opt_params = r#"[
		{"param_name": "minPoints", "min": 2, "max": 6, "step": 1},
		{"param_name": "tolerance", "min": 0.0, "max": 0.01, "step": 0.0001},
		{"param_name": "lookback", "min": 60, "max": 240, "step": 20}
	]"#
)]
pub fn broadening_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<BroadeningConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(3);
	let tolerance = config.tolerance.unwrap_or(0.0005);
	let lookback = config.lookback.unwrap_or(120) as usize;

	let data_len = highs.len();
	if data_len < lookback + 5 {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::broadening(
		opens,
		highs,
		lows,
		closes,
		Some(min_points),
		Some(tolerance),
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
		let result = broadening_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 10]);
	}

	#[test]
	fn detects_broadening_breakdown() {
		let pivots = [
			(0, 102.0),
			(8, 100.0),
			(10, 103.0),
			(18, 97.0),
			(25, 106.0),
			(33, 94.0),
			(40, 109.0),
			(42, 102.0),
			(44, 90.0),
			(50, 88.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(BroadeningConfig {
			lookback: Some(40),
			..Default::default()
		});
		let result = broadening_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).unwrap();
		assert_eq!(result[idx], -1);
		assert!(idx >= 40, "signal should fire on the breakdown, got {idx}");
	}

	#[test]
	fn detects_broadening_breakout() {
		let pivots = [
			(0, 102.0),
			(8, 100.0),
			(10, 103.0),
			(18, 97.0),
			(25, 106.0),
			(33, 94.0),
			(40, 109.0),
			(42, 93.5),
			(44, 112.0),
			(50, 115.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let config = Some(BroadeningConfig {
			lookback: Some(40),
			..Default::default()
		});
		let result = broadening_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).unwrap();
		assert_eq!(result[idx], 1);
		assert!(idx >= 40, "signal should fire on the breakout, got {idx}");
	}
}
