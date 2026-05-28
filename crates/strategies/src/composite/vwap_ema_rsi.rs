use crate::types::configs::VwapEmaRsiConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use indicators_core::ema;
use indicators_core::rsi;
use indicators_core::vwap;

/// Vwap Ema Rsi
///
/// Buy when price above MA and RSI oversold. Sell when below MA and RSI overbought.
pub fn vwap_ema_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VwapEmaRsiConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let ema_fast_period = config.ema_fast_period.unwrap_or(5);
	let ema_slow_period = config.ema_slow_period.unwrap_or(20);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let rsi_oversold = config.rsi_oversold.unwrap_or(30.0);
	let rsi_overbought = config.rsi_overbought.unwrap_or(70.0);

	let min_data_length = ema_slow_period.max(rsi_period) as usize;

	if closes.len() < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: VWAP EMA RSI requires at least {} data points, got {}",
			min_data_length,
			closes.len()
		)));
	}

	let closes_vec = closes;
	let highs_vec = highs;
	let lows_vec = lows;
	let volumes_vec = volumes;

	let ema_fast = ema(closes_vec, Some(ema_fast_period))?;
	let ema_slow = ema(closes_vec, Some(ema_slow_period))?;

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = rsi(closes_vec, Some(rsi_config));

	let vwap_config = indicators_core::VWAPConfig {
		price_source: None,
		anchored: None,
		session_length: None,
		period: Some(14),
	};
	let vwap_values = vwap(
		highs_vec,
		lows_vec,
		closes_vec,
		volumes_vec,
		Some(vwap_config),
	);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, closes, and volumes arrays must have the same length".into(),
		));
	}
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_data_length - 1 {
			0
		} else {
			let buy_condition = crossed_over_series(&ema_fast, &ema_slow, i as u32)
				&& closes[i] > vwap_values[i]
				&& rsi_values[i] < rsi_oversold;

			let sell_condition = crossed_under_series(&ema_fast, &ema_slow, i as u32)
				&& closes[i] < vwap_values[i]
				&& rsi_values[i] > rsi_overbought;

			if buy_condition {
				1
			} else if sell_condition {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn vwap_ema_rsi_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "vwap-ema-rsi-trend",
		"name": "VWAP + EMA + RSI Trend",
		"category": "composite",
		"description": "VWAP + EMA crossover + RSI (triple confluence)",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn vwap_ema_rsi_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"ema_fast_period": 5,
			"ema_slow_period": 20,
			"rsi_period": 14,
			"rsi_oversold": 30.0,
			"rsi_overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "ema_fast_period",
				"min": 3.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "ema_slow_period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "rsi_period",
				"min": 7.0,
				"max": 21.0,
				"step": 1.0
			},
			{
				"param_name": "rsi_oversold",
				"min": 20.0,
				"max": 40.0,
				"step": 1.0
			},
			{
				"param_name": "rsi_overbought",
				"min": 60.0,
				"max": 80.0,
				"step": 1.0
			}
		]
	})
}
