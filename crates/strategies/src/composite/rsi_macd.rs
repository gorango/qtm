use strategies_proc_macro::strategy;
use crate::types::configs::RsiMacdConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use indicators_core::macd;
use indicators_core::rsi;


#[strategy(
    id = "rsi-macd-confirmation",
    name = "RSI + MACD Confirmation",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "RSI + MACD confirmation",
    opt_params = r#"[{"param_name": "rsi_period", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "rsi_oversold", "min": 10.0, "max": 40.0, "step": 5.0}, {"param_name": "rsi_overbought", "min": 60.0, "max": 90.0, "step": 5.0}, {"param_name": "macd_fast_period", "min": 5.0, "max": 20.0, "step": 1.0}, {"param_name": "macd_slow_period", "min": 20.0, "max": 50.0, "step": 1.0}, {"param_name": "macd_signal_period", "min": 5.0, "max": 20.0, "step": 1.0}]"#
)]
pub fn rsi_macd_strategy(closes: &[f64], config: Option<RsiMacdConfig>) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let rsi_period = config.rsi_period.unwrap_or(14);
	let rsi_oversold = config.rsi_oversold.unwrap_or(30.0);
	let rsi_overbought = config.rsi_overbought.unwrap_or(70.0);
	let macd_fast_period = config.macd_fast_period.unwrap_or(12);
	let macd_slow_period = config.macd_slow_period.unwrap_or(26);
	let macd_signal_period = config.macd_signal_period.unwrap_or(9);

	let min_data_length = (rsi_period + 1).max(macd_slow_period + macd_signal_period) as usize;

	if closes.len() < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: RSI MACD requires at least {} data points, got {}",
			min_data_length,
			closes.len()
		)));
	}

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = rsi(closes, Some(rsi_config));

	let macd_config = indicators_core::MACDConfig {
		fast_period: Some(macd_fast_period),
		slow_period: Some(macd_slow_period),
		signal_period: Some(macd_signal_period),
	};
	let macd_result = macd(closes, Some(macd_config))?;

	let data_len = closes.len();
	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_data_length {
			0
		} else {
			// Level-based filter: RSI in oversold/overbought zone
			let rsi_filter_oversold = rsi_value < rsi_oversold;
			let rsi_filter_overbought = rsi_value > rsi_overbought;

			// Event-based trigger: MACD line crosses signal line
			let macd_bullish_trigger =
				crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32);
			let macd_bearish_trigger =
				crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32);

			// Combine: Filter (RSI level) + Trigger (MACD crossover)
			if rsi_filter_oversold && macd_bullish_trigger {
				1
			} else if rsi_filter_overbought && macd_bearish_trigger {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
