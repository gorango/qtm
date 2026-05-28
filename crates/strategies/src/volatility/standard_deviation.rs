use crate::types::configs::StandardDeviationConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Standard Deviation
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "standardDeviation",
	name = "Standard Deviation Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when standard deviation is below threshold and sell signals when it exceeds threshold",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "threshold", "min": 1.0, "max": 5.0, "step": 0.1}
	]"#
)]
pub fn standard_deviation_strategy(
	closes: &[f64],
	config: Option<StandardDeviationConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let threshold = config.threshold.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Standard Deviation period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&threshold) {
		return Err(StrategyError::Validation(
			"Standard Deviation threshold must be between 0.1 and 10.0".into(),
		));
	}

	let data_len = closes.len();
	let mstd_config = indicators_core::MSTDConfig {
		period: Some(period),
	};
	let std = indicators_core::moving_standard_deviation(closes, Some(mstd_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, &std_value) in std.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if std_value < threshold {
			1
		} else if std_value > threshold {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
