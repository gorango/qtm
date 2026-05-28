use crate::signals::{crossed_over_series, crossed_under_series};
use crate::types::configs::BollingerBandsConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Bollinger Bands Mean Reversion
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "bollingerBandsMeanReversion",
	name = "Bollinger Bands Mean Reversion Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when price falls below lower band and sell signals when price rises above upper band",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "stdDev", "min": 1.0, "max": 3.0, "step": 0.1}
	]"#
)]
pub fn bollinger_bands_mean_reversion_strategy(
	closes: &[f64],
	config: Option<BollingerBandsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let std_dev = config.std_dev.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Bollinger Bands Mean Reversion period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=5.0).contains(&std_dev) {
		return Err(StrategyError::Validation(
			"Bollinger Bands Mean Reversion std_dev must be between 0.1 and 5.0".into(),
		));
	}

	let data_len = closes.len();
	let bb_config = indicators_core::BBConfig {
		period: Some(period),
		std_dev: Some(std_dev),
	};
	let bb = indicators_core::bollinger_bands(closes, Some(bb_config))?;

	let mut signals = Vec::with_capacity(data_len);
	let closes_vec = closes; // Need vec for indexing if not using slices yet

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			// 1. Buy when price crosses OVER the Lower Band
			// (It was below, now it's back inside/above)
			if crossed_over_series(closes_vec, &bb.lower, i as u32) {
				1
			}
			// 2. Sell when price crosses UNDER the Upper Band
			// (It was above, now it's back inside/below)
			else if crossed_under_series(closes_vec, &bb.upper, i as u32) {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
