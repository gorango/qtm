use crate::types::configs::DmiConfig;
use crate::utils::signals::crossed_over;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// DMI Trend Strategy
///
/// Uses ADX and Directional Indicators for trend signals
/// Generates buy signals when ADX crosses over threshold and +DI > -DI
/// Generates sell signals when ADX crosses over threshold and +DI < -DI
#[strategy(
	id = "dmi",
	name = "DMI Trend",
	category = "trend",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Uses ADX and Directional Indicators for trend signals when ADX crosses over threshold with directional bias",
	opt_params = r#"[
		{"param_name": "period_di", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "period_adx", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "adx_threshold", "min": 15.0, "max": 35.0, "step": 1.0}
	]"#
)]
pub fn dmi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<DmiConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period_di = config.period_di.unwrap_or(14);
	let period_adx = config.period_adx.unwrap_or(14);
	let adx_threshold = config.adx_threshold.unwrap_or(25.0);

	// Validate parameters
	if !(2..=100).contains(&period_di) {
		return Err(StrategyError::Validation(
			"DMI DI period must be between 2 and 100".into(),
		));
	}
	if !(2..=100).contains(&period_adx) {
		return Err(StrategyError::Validation(
			"DMI ADX period must be between 2 and 100".into(),
		));
	}
	let data_len = highs.len();
	let min_periods = (period_adx * 3) as usize; // ADX needs more periods
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for DMI strategy".into(),
		));
	}

	// Calculate ADX (which includes +DI and -DI)
	let adx_config = indicators_core::ADXConfig {
		period: Some(period_adx),
	};
	let adx_result = indicators_core::adx(highs, lows, closes, Some(adx_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over(&adx_result.adx, adx_threshold, i as u32)
			&& adx_result.plus_di[i] > adx_result.minus_di[i]
		{
			1 // Buy signal: ADX crosses over threshold in uptrend
		} else if crossed_over(&adx_result.adx, adx_threshold, i as u32)
			&& adx_result.plus_di[i] < adx_result.minus_di[i]
		{
			-1 // Sell signal: ADX crosses over threshold in downtrend
		} else {
			0 // Hold: not trending or weak trend
		};
		signals.push(signal);
	}

	Ok(signals)
}
