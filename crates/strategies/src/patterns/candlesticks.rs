use crate::types::configs::CandlestickConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// All candlestick patterns the strategy can detect, keyed by the `pattern`
/// config selector.
const VALID_PATTERNS: &[&str] = &[
	"hammer",
	"inverted_hammer",
	"hanging_man",
	"shooting_star",
	"spinning_top",
	"long_legged_doji",
	"dragonfly_doji",
	"gravestone_doji",
	"bullish_harami",
	"bearish_harami",
	"piercing_line",
	"dark_cloud_cover",
	"tweezer_bottom",
	"tweezer_top",
	"three_white_soldiers",
	"three_black_crows",
	"three_inside_up",
	"three_inside_down",
	"three_outside_up",
	"three_outside_down",
	"abandoned_baby",
];

/// Candlestick Reversal Strategy
///
/// Detects single-, two-, and three-candle Japanese candlestick reversal
/// patterns selected via the `pattern` config parameter. Numeric config
/// fields left `None` fall through to each pattern's own detector defaults.
///
/// @strategy_id candlestick_reversal
/// @strategy_name Candlestick Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
#[strategy(
	id = "candlestick_reversal",
	name = "Candlestick Reversal Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Detects Japanese candlestick reversal patterns selected via the pattern parameter",
	opt_params = r#"[
		{"param_name": "bodyRatio", "min": 0.0, "max": 0.5, "step": 0.05},
		{"param_name": "shadowMultiplier", "min": 1.0, "max": 4.0, "step": 0.5},
		{"param_name": "trendBars", "min": 2, "max": 10, "step": 1},
		{"param_name": "minBodyRatio", "min": 0.1, "max": 0.9, "step": 0.1},
		{"param_name": "shadowTolerance", "min": 0.0, "max": 0.01, "step": 0.0005}
	]"#
)]
pub fn candlestick_reversal_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<CandlestickConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let pattern = config.pattern.as_deref().unwrap_or("hammer");
	let body_ratio = config.body_ratio;
	let shadow_multiplier = config.shadow_multiplier;
	let trend_bars = config.trend_bars;
	let min_body_ratio = config.min_body_ratio;
	let shadow_tolerance = config.shadow_tolerance;

	// Dispatch to the detector for the requested pattern. Each detector
	// already encodes direction in the sign (+1 bullish, -1 bearish).
	let signals: Vec<f64> = match pattern {
		"hammer" => indicators_core::hammer(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"inverted_hammer" => indicators_core::inverted_hammer(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"hanging_man" => indicators_core::hanging_man(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"shooting_star" => indicators_core::shooting_star(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"spinning_top" => {
			indicators_core::spinning_top(opens, highs, lows, closes, body_ratio, trend_bars)?
		}
		"long_legged_doji" => indicators_core::long_legged_doji(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"dragonfly_doji" => indicators_core::dragonfly_doji(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"gravestone_doji" => indicators_core::gravestone_doji(
			opens, highs, lows, closes, body_ratio, shadow_multiplier, trend_bars,
		)?,
		"bullish_harami" => {
			indicators_core::bullish_harami(opens, highs, lows, closes, body_ratio)?
		}
		"bearish_harami" => {
			indicators_core::bearish_harami(opens, highs, lows, closes, body_ratio)?
		}
		"piercing_line" => indicators_core::piercing_line(opens, highs, lows, closes)?,
		"dark_cloud_cover" => indicators_core::dark_cloud_cover(opens, highs, lows, closes)?,
		"tweezer_bottom" => {
			indicators_core::tweezer_bottom(opens, highs, lows, closes, shadow_tolerance)?
		}
		"tweezer_top" => {
			indicators_core::tweezer_top(opens, highs, lows, closes, shadow_tolerance)?
		}
		"three_white_soldiers" => {
			indicators_core::three_white_soldiers(opens, highs, lows, closes, min_body_ratio)?
		}
		"three_black_crows" => {
			indicators_core::three_black_crows(opens, highs, lows, closes, min_body_ratio)?
		}
		"three_inside_up" => {
			indicators_core::three_inside_up(opens, highs, lows, closes, min_body_ratio)?
		}
		"three_inside_down" => {
			indicators_core::three_inside_down(opens, highs, lows, closes, min_body_ratio)?
		}
		"three_outside_up" => {
			indicators_core::three_outside_up(opens, highs, lows, closes, min_body_ratio)?
		}
		"three_outside_down" => {
			indicators_core::three_outside_down(opens, highs, lows, closes, min_body_ratio)?
		}
		"abandoned_baby" => indicators_core::abandoned_baby(
			opens, highs, lows, closes, body_ratio, min_body_ratio,
		)?,
		other => {
			return Err(StrategyError::ConfigError(format!(
				"Unknown candlestick pattern '{other}'. Valid patterns: {}",
				VALID_PATTERNS.join(", ")
			)))
		}
	};

	Ok(signals
		.iter()
		.map(|&s| if s > 0.5 { 1 } else if s < -0.5 { -1 } else { 0 })
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::test_util::*;

	/// Four declining closes then `pattern` at index 4 and a neutral
	/// continuation bar. With `trend_bars = 3` the signal fires at index 4.
	fn after_downtrend(pattern: [f64; 4]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
		let mut bars = vec![
			bar(10.0, 10.5, 9.5, 9.8),
			bar(9.8, 10.2, 9.3, 9.5),
			bar(9.5, 9.9, 9.0, 9.2),
			bar(9.2, 9.6, 8.8, 9.0),
		];
		bars.push(pattern);
		bars.push(bar(9.2, 9.6, 9.0, 9.45));
		ohlc(&bars)
	}

	/// Four rising closes then `pattern` at index 4 and a neutral
	/// continuation bar.
	fn after_uptrend(pattern: [f64; 4]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
		let mut bars = vec![
			bar(9.8, 10.3, 9.6, 10.0),
			bar(10.0, 10.5, 9.8, 10.4),
			bar(10.4, 10.9, 10.2, 10.6),
			bar(10.6, 11.0, 10.4, 10.9),
		];
		bars.push(pattern);
		bars.push(bar(11.0, 11.4, 10.9, 11.3));
		ohlc(&bars)
	}

	fn run(pattern: &str, ohlc: &(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)) -> Vec<i8> {
		let config = Some(CandlestickConfig {
			pattern: Some(pattern.to_string()),
			trend_bars: Some(3),
			..Default::default()
		});
		candlestick_reversal_strategy(&ohlc.0, &ohlc.1, &ohlc.2, &ohlc.3, config).unwrap()
	}

	#[test]
	fn detects_hammer_after_decline() {
		let ohlc = after_downtrend(bar(9.1, 9.25, 8.5, 9.2));
		let result = run("hammer", &ohlc);
		assert_eq!(result[4], 1);
	}

	#[test]
	fn detects_shooting_star_after_advance() {
		let ohlc = after_uptrend(bar(10.8, 11.7, 10.75, 10.86));
		let result = run("shooting_star", &ohlc);
		assert_eq!(result[4], -1);
	}

	#[test]
	fn detects_three_white_soldiers() {
		let (o, h, l, c) = ohlc(&[
			bar(9.5, 10.2, 9.4, 10.0),
			bar(9.9, 10.7, 9.85, 10.5),
			bar(10.4, 11.2, 10.35, 11.0),
		]);
		let config = Some(CandlestickConfig {
			pattern: Some("three_white_soldiers".to_string()),
			..Default::default()
		});
		let result = candlestick_reversal_strategy(&o, &h, &l, &c, config).unwrap();
		assert_eq!(result[2], 1);
	}

	#[test]
	fn unknown_pattern_returns_config_error() {
		let (opens, highs, lows, closes) = ohlc(&[bar(100.0, 100.5, 99.5, 100.0); 6]);
		let config = Some(CandlestickConfig {
			pattern: Some("not_a_pattern".to_string()),
			..Default::default()
		});
		let err =
			candlestick_reversal_strategy(&opens, &highs, &lows, &closes, config).unwrap_err();
		assert!(err.to_string().contains("Unknown candlestick pattern"));
	}

	#[test]
	fn all_patterns_dispatch_without_error() {
		let (opens, highs, lows, closes) = ohlc(&[bar(100.0, 100.5, 99.5, 100.0); 8]);
		for &pattern in VALID_PATTERNS {
			let config = Some(CandlestickConfig {
				pattern: Some(pattern.to_string()),
				..Default::default()
			});
			let result =
				candlestick_reversal_strategy(&opens, &highs, &lows, &closes, config).unwrap();
			assert_eq!(result.len(), 8, "pattern '{pattern}' returned wrong length");
		}
	}

	#[test]
	fn default_pattern_is_hammer() {
		let ohlc = after_downtrend(bar(9.1, 9.25, 8.5, 9.2));
		let config = Some(CandlestickConfig {
			pattern: None,
			trend_bars: Some(3),
			..Default::default()
		});
		let result =
			candlestick_reversal_strategy(&ohlc.0, &ohlc.1, &ohlc.2, &ohlc.3, config).unwrap();
		assert_eq!(result[4], 1);
	}
}
