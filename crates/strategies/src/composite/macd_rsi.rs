use crate::types::configs::RsiMacdConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Macd Rsi
///
/// Buy when RSI is oversold and MACD confirms bullish crossover. Sell on bearish alignment.
#[strategy(
	id = "macd_rsi_momentum",
	name = "MACD + RSI Momentum",
	category = "composite",
	default_timeframes = ["15m", "1h", "4h"],
	description = "MACD + RSI momentum confirmation",
	opt_params = r#"[{"param_name": "macdFastPeriod","min":5.0,"max":20.0,"step":1.0},{"param_name": "macdSlowPeriod","min":20.0,"max":50.0,"step":1.0},{"param_name": "macdSignalPeriod","min":5.0,"max":20.0,"step":1.0},{"param_name": "rsiPeriod","min":5.0,"max":30.0,"step":1.0},{"param_name": "rsiOversold","min":10.0,"max":40.0,"step":5.0},{"param_name": "rsiOverbought","min":60.0,"max":90.0,"step":5.0}]"#
)]
pub fn macd_rsi_strategy(closes: &[f64], config: Option<RsiMacdConfig>) -> StrategyResult<Vec<i8>> {
	let cfg = config.unwrap_or_default();

	let fast_period = cfg.macd_fast_period.unwrap_or(12);
	let slow_period = cfg.macd_slow_period.unwrap_or(26);
	let signal_period = cfg.macd_signal_period.unwrap_or(9);
	let rsi_period = cfg.rsi_period.unwrap_or(14) as usize;
	let oversold = cfg.rsi_oversold.unwrap_or(30.0);
	let overbought = cfg.rsi_overbought.unwrap_or(70.0);

	let data_len = closes.len();
	let min_data_length = (slow_period + signal_period).max(rsi_period as u32 + 1) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: MACD + RSI requires at least {min_data_length} data points, got {data_len}"
		)));
	}

	let macd_cfg_ind = indicators_core::MACDConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
		signal_period: Some(signal_period),
	};
	let closes_vec: Vec<f64> = closes.to_vec();
	let macd_result = indicators_core::macd(&closes_vec, Some(macd_cfg_ind))?;

	let rsi_cfg_ind = indicators_core::RSIConfig {
		period: Some(rsi_period as u32),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_cfg_ind));

	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_data_length {
			0
		} else {
			let macd_bullish =
				crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32);
			let macd_bearish =
				crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32);

			if macd_bullish && rsi_value < oversold {
				1
			} else if macd_bearish && rsi_value > overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
