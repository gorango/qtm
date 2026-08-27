use crate::types::configs::VwapStochasticConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use indicators_core::stochastic_oscillator;
use indicators_core::vwap;
use strategies_proc_macro::strategy;

/// Vwap Stochastic Confirmation — strategy signal: `1` on entry long, `-1` on entry short, `0` otherwise (hold/flat).
/// See indicator docs for formula and regime notes. This is a thin signal wrapper.
#[strategy(
    id = "vwap_stochastic_confirmation",
    name = "VWAP + Stochastic Confirmation",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "VWAP + Stochastic confirmation",
    opt_params = r#"[{"param_name": "vwapPeriod", "min": 5.0, "max": 50.0, "step": 1.0}, {"param_name": "kPeriod", "min": 5.0, "max": 20.0, "step": 1.0}, {"param_name": "dPeriod", "min": 2.0, "max": 10.0, "step": 1.0}, {"param_name": "oversold", "min": 10.0, "max": 30.0, "step": 1.0}, {"param_name": "overbought", "min": 70.0, "max": 90.0, "step": 1.0}]"#
)]
pub fn vwap_stochastic_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VwapStochasticConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let vwap_period = config.vwap_period.unwrap_or(14);
	let k_period = config.k_period.unwrap_or(14);
	let d_period = config.d_period.unwrap_or(3);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, closes, and volumes arrays must have the same length".into(),
		));
	}
	let min_data_length = vwap_period.max(k_period + d_period) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: VWAP Stochastic requires at least {min_data_length} data points, got {data_len}"
		)));
	}

	let highs_vec = highs;
	let lows_vec = lows;
	let closes_vec = closes;
	let volumes_vec = volumes;

	let vwap_config = indicators_core::VWAPConfig {
		price_source: None,
		anchored: None,
		session_length: None,
		period: Some(vwap_period),
	};
	let vwap_values = vwap(
		highs_vec,
		lows_vec,
		closes_vec,
		volumes_vec,
		Some(vwap_config),
	);

	let stoch_config = indicators_core::StochConfig {
		k_period: Some(k_period),
		d_period: Some(d_period),
	};
	let stoch_result = stochastic_oscillator(highs_vec, lows_vec, closes_vec, Some(stoch_config));

	let data_len = closes.len();
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_data_length {
			0
		} else {
			let crossed_over_vwap = crossed_over_series(closes, &vwap_values, i as u32);
			let crossed_under_vwap = crossed_under_series(closes, &vwap_values, i as u32);

			if crossed_over_vwap && stoch_result.k[i] < oversold {
				1
			} else if crossed_under_vwap && stoch_result.k[i] > overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
