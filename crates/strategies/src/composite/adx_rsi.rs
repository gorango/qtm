use crate::types::configs::AdxRsiConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

#[strategy(
    id = "adx_rsi_trend_momentum",
    name = "ADX + RSI Trend Momentum",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Combine ADX trend + RSI momentum",
    opt_params = r#"[{"param_name": "adxPeriod", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "trendThreshold", "min": 20.0, "max": 40.0, "step": 1.0}, {"param_name": "rsiPeriod", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "oversold", "min": 10.0, "max": 40.0, "step": 5.0}, {"param_name": "overbought", "min": 60.0, "max": 90.0, "step": 5.0}]"#
)]
pub fn adx_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<AdxRsiConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let adx_period = config.adx_period.unwrap_or(14);
	let trend_threshold = config.trend_threshold.unwrap_or(25.0);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, and closes arrays must have the same length".into(),
		));
	}
	let min_data_length = (adx_period * 2).max(rsi_period + 1) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: ADX + RSI requires at least {min_data_length} data points, got {data_len}"
		)));
	}

	let adx_config = indicators_core::ADXConfig {
		period: Some(adx_period),
	};
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let adx_result = indicators_core::adx(&highs_vec, &lows_vec, &closes_vec, Some(adx_config));

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let adx = adx_result?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, (&adx_value, &rsi_value)) in adx
		.adx
		.iter()
		.zip(rsi_values.iter())
		.enumerate()
		.take(data_len)
	{
		let signal = if i < (adx_period * 2) as usize {
			0
		} else if adx_value > trend_threshold && rsi_value < oversold {
			1
		} else if adx_value > trend_threshold && rsi_value > overbought {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
