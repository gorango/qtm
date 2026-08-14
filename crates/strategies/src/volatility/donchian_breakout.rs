use crate::types::configs::DonchianTurtleConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Donchian Breakout
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "donchian_breakout",
	name = "Donchian Breakout Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "True Turtle Trading strategy: generates buy signals when price crosses over upper channel and sell signals when price crosses under lower channel",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn donchian_breakout_strategy(
	closes: &[f64],
	config: Option<DonchianTurtleConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Donchian Breakout period must be between 2 and 100".into(),
		));
	}

	let data_len = closes.len();
	let dc = indicators_core::donchian_channel(closes, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	#[allow(clippy::needless_range_loop)] // compares against the PRIOR window (i-1)
	for i in 0..data_len {
		// Classic turtle: the current close breaks out beyond the channel of
		// the PRIOR window.  Comparing against dc.upper[i] is impossible —
		// the channel includes closes[i], so close can never exceed it.
		let signal = if i < period as usize + 1 {
			0
		} else if closes[i] > dc.upper[i - 1] {
			1 // Buy signal: price crosses over prior upper band (breakout)
		} else if closes[i] < dc.lower[i - 1] {
			-1 // Sell signal: price crosses under prior lower band (breakdown)
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
