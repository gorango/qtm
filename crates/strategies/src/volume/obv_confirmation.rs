use crate::types::configs::ObvConfirmationConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// OBV Confirmation Strategy
///
/// Generates buy signals when OBV crosses above its SMA
/// Generates sell signals when OBV crosses below its SMA
#[strategy(
	id = "obv_confirmation",
	name = "OBV Confirmation",
	category = "volume",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when OBV crosses above its SMA, sell signals when OBV crosses below its SMA",
	opt_params = r#"[
		{"param_name": "obvPeriod", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "pricePeriod", "min": 5.0, "max": 50.0, "step": 1.0}
	]"#
)]
pub fn obv_confirmation_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<ObvConfirmationConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let obv_period = config.obv_period.unwrap_or(10);
	let price_period = config.price_period.unwrap_or(10);

	let data_len = closes.len();
	if closes.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"Closes and volumes must have equal length".into(),
		));
	}
	if !(5..=50).contains(&obv_period) {
		return Err(StrategyError::Validation(
			"OBV period must be between 5 and 50".into(),
		));
	}
	if !(5..=50).contains(&price_period) {
		return Err(StrategyError::Validation(
			"Price period must be between 5 and 50".into(),
		));
	}
	let min_required = std::cmp::max(obv_period, price_period) as usize;
	if data_len < min_required + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for OBV Confirmation strategy".into(),
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let obv_values = indicators_core::on_balance_volume(&closes_vec, &volumes_vec);

	let obv_ma = indicators_core::sma(&obv_values, Some(obv_period))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_required {
			0
		} else if crossed_over_series(&obv_values, &obv_ma, i as u32) {
			1
		} else if crossed_under_series(&obv_values, &obv_ma, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
