use crate::types::configs::AtrVolatilityThresholdConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Atr Volatility Threshold
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "atrVolatilityThreshold",
	name = "ATR Volatility Threshold Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when ATR is below threshold and sell signals when ATR exceeds threshold",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "volatilityThreshold", "min": 0.4, "max": 4.0, "step": 0.05}
	]"#
)]
pub fn atr_volatility_threshold_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<AtrVolatilityThresholdConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let volatility_threshold = config.volatility_threshold.unwrap_or(1.2);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"ATR Volatility Threshold period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&volatility_threshold) {
		return Err(StrategyError::Validation(
			"ATR Volatility Threshold volatility_threshold must be between 0.1 and 10.0".into(),
		));
	}

	let data_len = closes.len();
	let atr_config = indicators_core::ATRConfig {
		period: Some(period),
	};
	let atr = indicators_core::average_true_range(highs, lows, closes, Some(atr_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let current_atr = atr.atr_line[i];

			if current_atr < volatility_threshold {
				1
			} else if current_atr > volatility_threshold {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
