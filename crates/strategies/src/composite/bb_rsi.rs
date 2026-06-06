use strategies_proc_macro::strategy;
use crate::types::configs::BbRsiConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};


#[strategy(
    id = "bb-rsi-breakout",
    name = "Bollinger Bands + RSI Breakout",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Bollinger Bands + RSI breakout confirmation",
    opt_params = r#"[{"param_name": "bb_period", "min": 5.0, "max": 50.0, "step": 1.0}, {"param_name": "bb_std_dev", "min": 1.0, "max": 3.0, "step": 0.1}, {"param_name": "rsi_period", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "rsi_oversold", "min": 10.0, "max": 40.0, "step": 5.0}, {"param_name": "rsi_overbought", "min": 60.0, "max": 90.0, "step": 5.0}]"#
)]
pub fn bb_rsi_strategy(closes: &[f64], config: Option<BbRsiConfig>) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let bb_period = config.bb_period.unwrap_or(20);
	let bb_std_dev = config.bb_std_dev.unwrap_or(2.0);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let rsi_oversold = config.rsi_oversold.unwrap_or(30.0);
	let rsi_overbought = config.rsi_overbought.unwrap_or(70.0);

	let data_len = closes.len();
	let min_data_length = bb_period.max(rsi_period) as usize + 1;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: BB-RSI requires at least {} data points, got {}",
			min_data_length, data_len
		)));
	}

	let bb_config = indicators_core::BBConfig {
		period: Some(bb_period),
		std_dev: Some(bb_std_dev),
	};
	let closes_vec: Vec<f64> = closes.to_vec();
	let bb_result = indicators_core::bollinger_bands(&closes_vec, Some(bb_config))?;

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_data_length {
			0
		} else {
			// Trigger: price crosses band, Filter: RSI confirmation
			if crossed_over_series(&closes_vec, &bb_result.lower, i as u32)
				&& rsi_value <= rsi_oversold
			{
				1 // Buy: price crosses over lower band while RSI is oversold
			} else if crossed_under_series(&closes_vec, &bb_result.upper, i as u32)
				&& rsi_value >= rsi_overbought
			{
				-1 // Sell: price crosses under upper band while RSI is overbought
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
