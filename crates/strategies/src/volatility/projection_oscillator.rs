use crate::types::configs::ProjectionOscillatorConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Projection Oscillator
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
#[strategy(
	id = "projection_oscillator",
	name = "Projection Oscillator Strategy",
	category = "volatility",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when projection oscillator exceeds overbought level and sell signals when it falls below oversold level",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "smooth", "min": 2.0, "max": 10.0, "step": 1.0}
	]"#
)]
pub fn projection_oscillator_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<ProjectionOscillatorConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let smooth = config.smooth.unwrap_or(3);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Projection Oscillator period must be between 2 and 100".into(),
		));
	}
	if !(2..=20).contains(&smooth) {
		return Err(StrategyError::Validation(
			"Projection Oscillator smooth must be between 2 and 20".into(),
		));
	}

	let data_len = closes.len();
	let po =
		indicators_core::projection_oscillator(highs, lows, closes, Some(period), Some(smooth))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period + smooth) as usize {
			0
		} else {
			let po_value = po.spo_result[i];

			if po_value > 70.0 {
				1
			} else if po_value < 30.0 {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
