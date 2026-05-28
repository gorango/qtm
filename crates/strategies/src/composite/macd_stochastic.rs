use crate::types::configs::MacdStochasticConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// Macd Stochastic
///
/// Buy on MACD bullish crossover with stochastic confirmation. Sell on bearish crossover.
pub fn macd_stochastic_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<MacdStochasticConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(12);
	let slow_period = config.slow_period.unwrap_or(26);
	let signal_period = config.signal_period.unwrap_or(9);
	let k_period = config.k_period.unwrap_or(14);
	let d_period = config.d_period.unwrap_or(3);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	let data_len = closes.len();
	if data_len != highs.len() || data_len != lows.len() {
		return Err(StrategyError::Validation(
			"Highs, lows, and closes arrays must have the same length".into(),
		));
	}
	let min_data_length = (slow_period + signal_period).max(k_period + d_period + 1) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: MACD Stochastic requires at least {} data points",
			min_data_length
		)));
	}

	let macd_config = indicators_core::MACDConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
		signal_period: Some(signal_period),
	};
	let closes_vec: Vec<f64> = closes.to_vec();
	let macd_result = indicators_core::macd(&closes_vec, Some(macd_config))?;

	let stoch_config = indicators_core::StochConfig {
		k_period: Some(k_period),
		d_period: Some(d_period),
	};
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let stoch_result = indicators_core::stochastic_oscillator(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(stoch_config),
	);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32)
			&& stoch_result.k[i] < oversold
		{
			1
		} else if crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32)
			&& stoch_result.k[i] > overbought
		{
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn macd_stochastic_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "macd-stochastic-confirmation",
		"name": "MACD + Stochastic Confirmation",
		"category": "composite",
		"description": "MACD + Stochastic confirmation",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn macd_stochastic_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fast_period": 12,
			"slow_period": 26,
			"signal_period": 9,
			"k_period": 14,
			"d_period": 3,
			"oversold": 20.0,
			"overbought": 80.0
		},
		"optimization_bounds": [
			{
				"param_name": "fast_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "slow_period",
				"min": 20.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "signal_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "k_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "d_period",
				"min": 2.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 70.0,
				"max": 90.0,
				"step": 1.0
			}
		]
	})
}
