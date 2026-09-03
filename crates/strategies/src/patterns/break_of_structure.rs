use crate::types::configs::BreakOfStructureConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Break of Structure Strategy
///
/// Wraps the `break_of_structure` indicator. Structural-only — session
/// filtering (e.g. London/NY) is caller-side. By default `mode=horizontal`
/// (close crosses last swing high/low). Use `trendline`/`either` to include
/// diagonal trendline breaks through the last `trendlinePoints` swings.
/// CHoCH (`2`/`-2` from the indicator) is emitted as the same direction as
/// BOS — callers that need the distinction can call the indicator directly.
#[strategy(
	id = "break_of_structure",
	name = "Break of Structure Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Structural BOS/CHoCH: close crosses last swing high/low (horizontal) or trendline through last N swings; session alignment is caller-side",
	opt_params = r#"[
		{"param_name": "lookaround", "min": 1.0, "max": 10.0, "step": 1.0},
		{"param_name": "trendlinePoints", "min": 2.0, "max": 10.0, "step": 1.0}
	]"#
)]
pub fn break_of_structure_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<BreakOfStructureConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let lookaround = config.lookaround.unwrap_or(2);
	let mode = config.mode.unwrap_or_else(|| "horizontal".to_string());
	let trendline_points = config.trendline_points.unwrap_or(3);

	let signals = indicators_core::break_of_structure(
		highs,
		lows,
		closes,
		Some(lookaround),
		Some(mode),
		Some(trendline_points),
	)?;

	let mut result = Vec::with_capacity(highs.len());
	for &s in &signals {
		let v = if s > 0.5 {
			1 // BOS and CHoCH share direction at strategy level
		} else if s < -0.5 {
			-1
		} else {
			0
		};
		result.push(v);
	}
	Ok(result)
}
