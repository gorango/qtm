use crate::types::configs::LinregSlopeConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Linear Regression Slope Trend Strategy
///
/// Generates signals based on slope direction with ADX confirmation
/// Buy when slope > 0 and ADX > threshold, sell when slope < 0 and ADX > threshold
#[strategy(
	id = "lin_reg_slope",
	name = "Linear Regression Slope Trend",
	category = "trend",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Generates signals based on slope direction with ADX confirmation for trending markets",
	opt_params = r#"[
		{"param_name": "period", "min": 10.0, "max": 50.0, "step": 1.0},
		{"param_name": "slopePeriod", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "periodAdx", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "adxThreshold", "min": 15.0, "max": 35.0, "step": 1.0}
	]"#
)]
pub fn lin_reg_slope_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<LinregSlopeConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let slope_period = config.slope_period.unwrap_or(10);
	let period_adx = config.period_adx.unwrap_or(14);
	let adx_threshold = config.adx_threshold.unwrap_or(25.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"LinReg period must be between 2 and 100".into(),
		));
	}
	if !(1..=50).contains(&slope_period) {
		return Err(StrategyError::Validation(
			"Slope period must be between 1 and 50".into(),
		));
	}
	if !(2..=100).contains(&period_adx) {
		return Err(StrategyError::Validation(
			"ADX period must be between 2 and 100".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = (period + slope_period).max(period_adx * 3) as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Linear Regression Slope strategy".into(),
		));
	}

	// Convert to vec for multiple uses
	let closes_vec: Vec<f64> = closes.to_vec();

	// Calculate Linear Regression
	let linreg_config = indicators_core::LinRegConfig {
		period: Some(period),
		offset: Some(0),
	};
	let linreg_result = indicators_core::linreg(&closes_vec, Some(linreg_config))?;

	// Calculate ADX for confirmation
	let adx_config = indicators_core::ADXConfig {
		period: Some(period_adx),
	};
	let adx_result = indicators_core::adx(highs, lows, closes, Some(adx_config))?;

	// Calculate slope (difference over slope_period)
	let mut slopes = vec![0.0; data_len];
	for i in slope_period as usize..data_len {
		slopes[i] = linreg_result[i] - linreg_result[i - slope_period as usize];
	}

	// Generate signals
	let signals: Vec<i8> = slopes
		.iter()
		.zip(adx_result.adx.iter())
		.enumerate()
		.take(data_len)
		.map(|(i, (&slope, &adx_val))| {
			if i < min_periods {
				0 // Not enough data
			} else if slope > 0.0 && adx_val > adx_threshold {
				1 // Buy signal: positive slope with ADX confirmation
			} else if slope < 0.0 && adx_val > adx_threshold {
				-1 // Sell signal: negative slope with ADX confirmation
			} else {
				0 // Hold
			}
		})
		.collect();

	Ok(signals)
}
