use crate::types::configs::RocObvRsiConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

#[strategy(
    id = "roc-obv-rsi-momentum",
    name = "ROC OBV + RSI Momentum",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Complex: ROC of OBV + RSI",
    opt_params = r#"[{"param_name": "obvRocPeriod", "min": 1.0, "max": 10.0, "step": 1.0}, {"param_name": "rsiPeriod", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "rsiOverbought", "min": 60.0, "max": 90.0, "step": 5.0}, {"param_name": "rsiOversold", "min": 10.0, "max": 40.0, "step": 5.0}]"#
)]
pub fn roc_obv_rsi_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<RocObvRsiConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let obv_roc_period = config.obv_roc_period.unwrap_or(3);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let rsi_overbought = config.rsi_overbought.unwrap_or(70.0);
	let rsi_oversold = config.rsi_oversold.unwrap_or(30.0);

	let data_len = closes.len();
	if volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Closes and volumes arrays must have the same length".into(),
		));
	}
	let min_data_length = (rsi_period + 1).max(obv_roc_period + 1).max(2) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: ROC OBV + RSI requires at least {min_data_length} data points, got {data_len}"
		)));
	}

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let obv_values = indicators_core::on_balance_volume(&closes_vec, &volumes_vec);

	let roc_config = indicators_core::PriceRateOfChangeConfig {
		period: Some(obv_roc_period),
	};
	let roc_obv_values = indicators_core::price_rate_of_change(&obv_values, Some(roc_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i == 0 {
			0
		} else {
			let roc_obv_value = roc_obv_values[i];
			let rsi_value = rsi_values[i];

			if roc_obv_value > 0.0 && rsi_value <= rsi_oversold {
				1
			} else if roc_obv_value < 0.0 && rsi_value >= rsi_overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
