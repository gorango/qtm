use crate::types::configs::AccumulationDistributionConfig;
use crate::{StrategyError, StrategyResult};

/// Accumulation/Distribution Line Strategy
///
/// Generates buy signals when AD line and price are both increasing (bullish confirmation)
/// Generates sell signals when AD and price diverge (bearish divergence)
///
/// @strategy_id accumulation-distribution
/// @strategy_name Accumulation/Distribution Line
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn accumulation_distribution_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<AccumulationDistributionConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20) as usize;

	let data_len = closes.len();
	if closes.len() != highs.len() || closes.len() != lows.len() || closes.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}
	if !(5..=50).contains(&period) {
		return Err(StrategyError::Validation(
			"Period must be between 5 and 50".into(),
		));
	}
	if data_len < period + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Accumulation/Distribution strategy".into(),
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let ad_values = indicators_core::accumulation_distribution(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		&volumes_vec,
	);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period {
			0
		} else {
			let ad_increasing = ad_values[i] > ad_values[i - period];
			let price_increasing = closes[i] > closes[i - period];

			if ad_increasing && price_increasing {
				1
			} else if (ad_increasing && !price_increasing) || (!ad_increasing && price_increasing) {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Accumulation/Distribution strategy metadata for registry
pub fn accumulation_distribution_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "accumulation-distribution",
		"name": "Accumulation/Distribution Line",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when AD line and price are both increasing, sell signals on divergence between AD and price"
	})
}

/// Get Accumulation/Distribution strategy default parameters
pub fn accumulation_distribution_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
