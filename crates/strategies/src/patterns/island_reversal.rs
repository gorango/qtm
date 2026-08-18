use crate::types::configs::IslandReversalConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Island Reversal Strategy
///
/// Detects island reversals: a gap, a cluster of bars trading away from the
/// prior range, then a gap back through it — leaving the cluster isolated on
/// both sides. Signals fire on the covering gap: bullish (gap down then gap
/// up) and bearish (gap up then gap down).
///
/// @strategy_id island_reversal
/// @strategy_name Island Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "island_reversal",
	name = "Island Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects island reversals and generates signals on the covering gap",
	opt_params = r#"[
		{"param_name": "minIslandBars", "min": 1, "max": 5, "step": 1},
		{"param_name": "maxIslandBars", "min": 5, "max": 30, "step": 5}
	]"#
)]
pub fn island_reversal_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<IslandReversalConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_island_bars = config.min_island_bars.unwrap_or(2);
	let max_island_bars = config.max_island_bars.unwrap_or(15);

	let data_len = highs.len();
	if data_len < 3 {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::island_reversal(
		opens,
		highs,
		lows,
		closes,
		Some(min_island_bars),
		Some(max_island_bars),
	)?;

	Ok(signals
		.iter()
		.map(|&s| if s > 0.5 { 1 } else if s < -0.5 { -1 } else { 0 })
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::test_util::*;

	#[test]
	fn insufficient_data_returns_zeros() {
		let (opens, highs, lows, closes) = ohlc(&[bar(100.0, 100.5, 99.5, 100.0); 2]);
		let result = island_reversal_strategy(&opens, &highs, &lows, &closes, None).unwrap();
		assert_eq!(result, vec![0; 2]);
	}

	#[test]
	fn detects_bullish_island_reversal() {
		let mut bars: Vec<[f64; 4]> = Vec::new();
		for _ in 0..10 {
			bars.push(bar(100.0, 100.5, 99.5, 100.0));
		}
		bars.push(bar(99.8, 100.2, 99.4, 99.9)); // pre-gap bar
		bars.push(bar(95.0, 95.8, 93.5, 94.2)); // gap down
		bars.push(bar(94.0, 94.6, 93.2, 94.1)); // island
		bars.push(bar(94.3, 94.8, 93.6, 94.0)); // island
		bars.push(bar(94.2, 95.2, 93.8, 95.0)); // island
		bars.push(bar(101.5, 102.5, 101.0, 102.0)); // gap up back over baseline
		bars.push(bar(102.0, 102.8, 101.5, 102.6));

		let (opens, highs, lows, closes) = ohlc(&bars);
		let config = Some(IslandReversalConfig {
			min_island_bars: Some(3),
			max_island_bars: Some(10),
		});
		let result = island_reversal_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s > 0).expect("no bullish signal");
		assert_eq!(result[idx], 1);
		assert_eq!(idx, 15, "signal should fire on the covering gap-up");
	}

	#[test]
	fn detects_bearish_island_reversal() {
		let mut bars: Vec<[f64; 4]> = Vec::new();
		for _ in 0..10 {
			bars.push(bar(100.0, 100.5, 99.5, 100.0));
		}
		bars.push(bar(100.2, 100.6, 99.8, 100.1)); // pre-gap bar
		bars.push(bar(105.0, 105.8, 104.5, 105.2)); // gap up
		bars.push(bar(104.4, 105.4, 104.2, 105.1)); // island
		bars.push(bar(104.7, 105.2, 104.0, 104.5)); // island
		bars.push(bar(104.5, 105.0, 104.3, 104.6)); // island
		bars.push(bar(98.5, 99.0, 98.0, 98.4)); // gap down back below baseline
		bars.push(bar(98.2, 98.8, 97.8, 98.0));

		let (opens, highs, lows, closes) = ohlc(&bars);
		let config = Some(IslandReversalConfig {
			min_island_bars: Some(3),
			max_island_bars: Some(10),
		});
		let result = island_reversal_strategy(&opens, &highs, &lows, &closes, config).unwrap();
		let idx = result.iter().position(|&s| s < 0).expect("no bearish signal");
		assert_eq!(result[idx], -1);
		assert_eq!(idx, 15, "signal should fire on the covering gap-down");
	}
}
