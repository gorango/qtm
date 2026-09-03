use crate::types::configs::PowerOfThreeConfig;
use crate::StrategyResult;
use strategies_proc_macro::strategy;

/// Power of Three (AMD) Strategy
///
/// Structural AMD: tight accumulation (`accumulationThreshold` spread), false-
/// break manipulation that reclaims inside the range within `manipulationBars`,
/// then distribution break of the opposite side. No clock — filter returned
/// signals by session in caller for ICT-faithful London/NY alignment.
#[strategy(
	id = "power_of_three",
	name = "Power of Three (AMD) Strategy",
	category = "patterns",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Structural Power of Three (AMD): accumulation, manipulation trap that reclaims, then distribution; session alignment is caller-side",
	opt_params = r#"[
		{"param_name": "accumulationPeriod", "min": 10.0, "max": 60.0, "step": 5.0},
		{"param_name": "accumulationThreshold", "min": 0.005, "max": 0.05, "step": 0.005},
		{"param_name": "manipulationThreshold", "min": 0.001, "max": 0.02, "step": 0.001}
	]"#
)]
pub fn power_of_three_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<PowerOfThreeConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let accumulation_period = config.accumulation_period.unwrap_or(20);
	let accumulation_threshold = config.accumulation_threshold.unwrap_or(0.015);
	let manipulation_threshold = config.manipulation_threshold.unwrap_or(0.005);
	let manipulation_bars = config.manipulation_bars.unwrap_or(5);

	let signals = indicators_core::power_of_three(
		highs,
		lows,
		closes,
		Some(accumulation_period),
		Some(accumulation_threshold),
		Some(manipulation_threshold),
		Some(manipulation_bars),
	)?;

	let mut result = Vec::with_capacity(highs.len());
	for &s in &signals {
		let v = if s > 0.5 {
			1 // bullish distribution (bear trap)
		} else if s < -0.5 {
			-1 // bearish distribution (bull trap)
		} else {
			0
		};
		result.push(v);
	}
	Ok(result)
}
