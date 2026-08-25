use crate::types::configs::MomentumConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Momentum Trend Strategy
///
/// Generates buy signals when momentum crosses below oversold level
/// Generates sell signals when momentum crosses above overbought level
#[strategy(
	id = "momentum",
	name = "Momentum Trend",
	category = "momentum",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when momentum crosses below oversold level and sell signals when momentum crosses above overbought level",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "overbought", "min": 60.0, "max": 90.0, "step": 5.0},
		{"param_name": "oversold", "min": 10.0, "max": 40.0, "step": 5.0}
	]"#
)]
pub fn momentum_strategy(
	closes: &[f64],
	config: Option<MomentumConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let overbought = config.overbought.unwrap_or(70.0);
	let oversold = config.oversold.unwrap_or(30.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Momentum period must be between 2 and 100".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Momentum strategy".into(),
		));
	}

	// Calculate Momentum
	let momentum_config = indicators_core::MomentumIndexConfig {
		period: Some(period),
	};
	let momentum_result = indicators_core::momentum_index(closes, Some(momentum_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_under(&momentum_result, oversold, i as u32) {
			1 // Buy signal: momentum crosses below oversold
		} else if crossed_over(&momentum_result, overbought, i as u32) {
			-1 // Sell signal: momentum crosses above overbought
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn insufficient_data_returns_error() {
		let closes = vec![100.0; 5];
		let result = momentum_strategy(&closes, None);
		assert!(result.is_err());
		assert!(result
			.err()
			.unwrap()
			.to_string()
			.contains("Insufficient data"));
	}

	#[test]
	fn invalid_period_returns_error() {
		let config = Some(MomentumConfig {
			period: Some(200),
			..Default::default()
		});
		let closes = vec![100.0; 50];
		let result = momentum_strategy(&closes, config);
		assert!(result.is_err());
	}

	#[test]
	fn buy_signal_on_cross_under_oversold() {
		// momentum[13] = close[13] - close[0]  = 100 - 0   = 100  (above oversold)
		// momentum[14] = close[14] - close[1]  = 100 - 100 = 0    (cross below 30)
		let closes = vec![
			0.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0,
			100.0, 100.0, 100.0,
		];
		let result = momentum_strategy(&closes, None).unwrap();
		assert_eq!(result[14], 1);
	}

	#[test]
	fn sell_signal_on_cross_above_overbought() {
		// momentum[13] = close[13] - close[0]  = 100 - 100 = 0   (below overbought)
		// momentum[14] = close[14] - close[1]  = 200 - 100 = 100 (cross above 70)
		let closes: Vec<f64> = {
			let mut c = vec![100.0; 13];
			c.push(100.0);
			c.push(200.0);
			c
		};
		let result = momentum_strategy(&closes, None).unwrap();
		assert_eq!(result[14], -1);
	}

	#[test]
	fn initial_signals_are_zero() {
		let closes = vec![100.0; 20];
		let result = momentum_strategy(&closes, None).unwrap();
		for (i, &signal) in result.iter().enumerate().take(13) {
			assert_eq!(signal, 0, "signal[{i}] should be 0");
		}
	}

	#[test]
	fn config_default_works() {
		let config = MomentumConfig::default();
		let closes = vec![100.0; 20];
		let result = momentum_strategy(&closes, Some(config));
		assert!(result.is_ok());
	}
}
