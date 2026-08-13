use crate::types::configs::ParabolicSarConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Parabolic SAR Trend Strategy
///
/// Generates buy signals when price is above SAR
/// Generates sell signals when price is below SAR
#[strategy(
	id = "parabolicSar",
	name = "Parabolic SAR Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when price is above SAR and sell signals when price is below SAR",
	opt_params = r#"[
		{"param_name": "step", "min": 0.01, "max": 0.05, "step": 0.005},
		{"param_name": "maxStep", "min": 0.01, "max": 0.1, "step": 0.01}
	]"#
)]
pub fn parabolic_sar_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<ParabolicSarConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let step = config.step.unwrap_or(0.02);
	let max_step = config.max_step.unwrap_or(0.02);

	let data_len = highs.len();
	if data_len < 2 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Parabolic SAR strategy".into(),
		));
	}

	// Convert for later use
	let closes_vec: Vec<f64> = closes.to_vec();

	// Calculate Parabolic SAR
	let psar_config = indicators_core::PSARConfig {
		step: Some(step),
		max: Some(max_step),
	};
	let psar_result = indicators_core::parabolic_sar(highs, lows, closes, Some(psar_config))?;

	let mut signals = Vec::with_capacity(data_len);
	let mut prev_trend = 0; // 1 for Long, -1 for Short

	for (i, (&current_sar, &current_price)) in psar_result
		.psar_result
		.iter()
		.zip(closes_vec.iter())
		.enumerate()
		.take(data_len)
	{
		// Determine current trend based on SAR position
		let current_trend = if current_price > current_sar { 1 } else { -1 };

		let signal = if i == 0 {
			0
		} else {
			// Signal only when trend flips
			if prev_trend == -1 && current_trend == 1 {
				1 // Buy Flip
			} else if prev_trend == 1 && current_trend == -1 {
				-1 // Sell Flip
			} else {
				0
			}
		};

		prev_trend = current_trend;
		signals.push(signal);
	}

	Ok(signals)
}
