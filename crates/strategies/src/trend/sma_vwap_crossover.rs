use crate::types::configs::SmaVwapCrossoverConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// SMA-VWAP Crossover Strategy
///
/// Generates buy signals when SMA crosses above VWAP
/// Generates sell signals when SMA crosses below VWAP
#[strategy(
	id = "sma_vwap_crossover",
	name = "Sma Vwap Crossover",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when SMA crosses above VWAP and sell signals when SMA crosses below VWAP",
	opt_params = r#"[
		{"param_name": "smaPeriod", "min": 2.0, "max": 10.0, "step": 1.0},
		{"param_name": "vwapPeriod", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "anchored", "min": 0.0, "max": 1.0, "step": 1.0},
		{"param_name": "sessionLength", "min": 60.0, "max": 1440.0, "step": 60.0}
	]"#
)]
pub fn sma_vwap_crossover_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<SmaVwapCrossoverConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let sma_period = config.sma_period.unwrap_or(3);
	let vwap_period = config.vwap_period.unwrap_or(14);
	let price_source = config.price_source.unwrap_or("hlc3".to_string());
	let anchored = config.anchored.unwrap_or(true);
	let session_length = config.session_length.unwrap_or(1440);

	// Validate parameters
	if !(2..=100).contains(&sma_period) {
		return Err(StrategyError::Validation(
			"SMA period must be between 2 and 100".into(),
		));
	}
	if !(2..=200).contains(&vwap_period) {
		return Err(StrategyError::Validation(
			"VWAP period must be between 2 and 200".into(),
		));
	}
	let min_period = sma_period.max(vwap_period) as usize;
	let data_len = highs.len();
	if data_len < min_period
		|| lows.len() < min_period
		|| closes.len() < min_period
		|| volumes.len() < min_period
	{
		return Err(StrategyError::InsufficientData(
			"Insufficient data for SMA-VWAP crossover strategy".into(),
		));
	}

	// Calculate SMA
	let closes_vec: Vec<f64> = closes.to_vec();
	let sma_values = indicators_core::sma(&closes_vec, Some(sma_period))?;

	// Calculate VWAP
	let vwap_config = indicators_core::VWAPConfig {
		period: Some(vwap_period),
		price_source: Some(price_source),
		anchored: Some(anchored),
		session_length: Some(session_length),
	};
	let vwap_values = indicators_core::vwap(highs, lows, closes, volumes, Some(vwap_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_period {
			0 // Not enough data
		} else if crossed_over_series(&sma_values, &vwap_values, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&sma_values, &vwap_values, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}
