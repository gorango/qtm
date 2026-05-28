use crate::types::configs::KeltnerVolatilityBreakoutConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Keltner Volatility Breakout
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "keltnerVolatilityBreakout",
	name = "Keltner Volatility Breakout Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates sell signals when close exceeds upper channel and buy signals when close falls below lower channel",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn keltner_volatility_breakout_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<KeltnerVolatilityBreakoutConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Keltner Volatility Breakout period must be between 2 and 100".into(),
		));
	}

	let data_len = closes.len();
	let kc = indicators_core::keltner_channel(highs, lows, closes, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, (&close, &upper, &lower)) in closes
		.iter()
		.zip(kc.upper.iter())
		.zip(kc.lower.iter())
		.map(|((c, u), l)| (c, u, l))
		.enumerate()
		.take(data_len)
	{
		let signal = if i < period as usize {
			0
		} else if close > upper {
			-1
		} else if close < lower {
			1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
