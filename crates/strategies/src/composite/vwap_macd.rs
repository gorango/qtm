use crate::types::configs::VwapMacdConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

#[strategy(
    id = "vwap-macd-momentum",
    name = "VWAP + MACD Momentum",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "VWAP + MACD momentum",
    opt_params = r#"[{"param_name": "macdFastPeriod", "min": 5.0, "max": 20.0, "step": 1.0}, {"param_name": "macdSlowPeriod", "min": 20.0, "max": 50.0, "step": 1.0}, {"param_name": "macdSignalPeriod", "min": 5.0, "max": 20.0, "step": 1.0}]"#
)]
pub fn vwap_macd_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VwapMacdConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let macd_fast_period = config.macd_fast_period.unwrap_or(12);
	let macd_slow_period = config.macd_slow_period.unwrap_or(26);
	let macd_signal_period = config.macd_signal_period.unwrap_or(9);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, closes, and volumes arrays must have the same length".into(),
		));
	}
	let min_data_length = 14.max(macd_slow_period + macd_signal_period) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: VWAP MACD requires at least {min_data_length} data points, got {data_len}"
		)));
	}

	let vwap_config = indicators_core::VWAPConfig {
		price_source: None,
		anchored: None,
		session_length: None,
		period: Some(14),
	};
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();
	let vwap_values = indicators_core::vwap(
		&highs_vec,
		&lows_vec,
		closes,
		&volumes_vec,
		Some(vwap_config),
	);

	let macd_config = indicators_core::MACDConfig {
		fast_period: Some(macd_fast_period),
		slow_period: Some(macd_slow_period),
		signal_period: Some(macd_signal_period),
	};
	let macd_result = indicators_core::macd(closes, Some(macd_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_data_length - 1 {
			0
		} else {
			let price_cross_above_vwap = crossed_over_series(&closes_vec, &vwap_values, i as u32);
			let price_cross_below_vwap = crossed_under_series(&closes_vec, &vwap_values, i as u32);
			let macd_bullish_crossover =
				crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32);
			let macd_bearish_crossover =
				crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32);

			if price_cross_above_vwap && macd_bullish_crossover {
				1
			} else if price_cross_below_vwap && macd_bearish_crossover {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
