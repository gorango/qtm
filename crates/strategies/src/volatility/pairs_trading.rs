use crate::types::configs::PairsTradingConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Pairs Trading
///
/// Buy when spread deviates below threshold (undervalued). Sell when above threshold (overvalued).
#[strategy(
	id = "pairsTrading",
	name = "Pairs Trading Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Stateful strategy that enters positions based on z-score entry thresholds and exits on exit thresholds",
	opt_params = r#"[
		{"param_name": "period", "min": 20.0, "max": 200.0, "step": 10.0},
		{"param_name": "entryThreshold", "min": 1.0, "max": 3.0, "step": 0.1},
		{"param_name": "exitThreshold", "min": 0.1, "max": 1.0, "step": 0.1}
	]"#
)]
pub fn pairs_trading_strategy(
	closes: &[f64],
	config: Option<PairsTradingConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(100);
	let entry_threshold = config.entry_threshold.unwrap_or(2.0);
	let exit_threshold = config.exit_threshold.unwrap_or(0.5);

	if !(2..=500).contains(&period) {
		return Err(StrategyError::Validation(
			"Pairs Trading period must be between 2 and 500".into(),
		));
	}
	if !(0.1..=10.0).contains(&entry_threshold) {
		return Err(StrategyError::Validation(
			"Pairs Trading entry_threshold must be between 0.1 and 10.0".into(),
		));
	}
	if !(0.1..=5.0).contains(&exit_threshold) {
		return Err(StrategyError::Validation(
			"Pairs Trading exit_threshold must be between 0.1 and 5.0".into(),
		));
	}

	let data_len = closes.len();
	let z_config = indicators_core::ZScoreConfig {
		period: Some(period),
	};
	let z_arr = indicators_core::z_score(closes, Some(z_config))?;
	let mut signals = Vec::with_capacity(data_len);
	let mut position = 0i8;

	for (i, &z) in z_arr.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if position == 0 {
			if z < -entry_threshold {
				position = 1;
				1
			} else if z > entry_threshold {
				position = -1;
				-1
			} else {
				0
			}
		} else if position == 1 && z > -exit_threshold {
			position = 0;
			-1
		} else if position == -1 && z < exit_threshold {
			position = 0;
			1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
