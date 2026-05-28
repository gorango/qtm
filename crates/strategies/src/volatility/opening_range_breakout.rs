use crate::types::configs::OpeningRangeBreakoutConfig;
use crate::utils::signals::consolidating;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Opening Range Breakout
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "openingRangeBreakout",
	name = "Opening Range Breakout Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals on breakout from consolidation and sell signals during consolidation",
	opt_params = r#"[
		{"param_name": "lookback", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "thresholdPct", "min": 0.005, "max": 0.05, "step": 0.005}
	]"#
)]
pub fn opening_range_breakout_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<OpeningRangeBreakoutConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let lookback = config.lookback.unwrap_or(10);
	let threshold_pct = config.threshold_pct.unwrap_or(0.02);

	if !(2..=50).contains(&lookback) {
		return Err(StrategyError::Validation(
			"Opening Range Breakout lookback must be between 2 and 50".into(),
		));
	}
	if !(0.001..=0.1).contains(&threshold_pct) {
		return Err(StrategyError::Validation(
			"Opening Range Breakout threshold_pct must be between 0.001 and 0.1".into(),
		));
	}

	let highs_vec = highs;
	let lows_vec = lows;
	let closes_vec = closes;
	let data_len = closes.len();
	let lookback_usize = lookback as usize;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let is_consolidating = consolidating(
			highs_vec,
			lows_vec,
			closes_vec,
			i,
			lookback_usize,
			threshold_pct,
		);
		let signal = if is_consolidating { -1 } else { 1 };
		signals.push(signal);
	}

	Ok(signals)
}
