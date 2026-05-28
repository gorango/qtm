use crate::types::configs::ZScoreConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Z-Score Breakout Strategy
///
/// Generates buy signals when z-score crosses over positive threshold (strong upward momentum)
/// Generates sell signals when z-score crosses under negative threshold (strong downward momentum)
#[strategy(
	id = "zScoreBreakout",
	name = "Z-Score Breakout Strategy",
	category = "volatility",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Generates buy signals when z-score crosses over positive threshold and sell signals when z-score crosses under negative threshold",
	opt_params = r#"[
		{"param_name": "meanPeriod", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "stdPeriod", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "threshold", "min": 1.0, "max": 3.0, "step": 0.1}
	]"#
)]
pub fn z_score_breakout_strategy(
	closes: &[f64],
	config: Option<ZScoreConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let mean_period = config.mean_period.unwrap_or(20);
	let std_period = config.std_period.unwrap_or(20);
	let threshold = config.threshold.unwrap_or(2.0);

	if !(2..=100).contains(&mean_period) {
		return Err(StrategyError::Validation(
			"Mean period must be between 2 and 100".into(),
		));
	}
	if !(2..=100).contains(&std_period) {
		return Err(StrategyError::Validation(
			"Std period must be between 2 and 100".into(),
		));
	}
	if threshold <= 0.0 {
		return Err(StrategyError::Validation(
			"Threshold must be positive".into(),
		));
	}

	let data_len = closes.len();
	let min_period = mean_period.max(std_period) as usize;
	if data_len < min_period {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Z-Score Breakout strategy".into(),
		));
	}

	let closes_slice: Vec<f64> = closes.to_vec();
	let closes_for_means: &[f64] = &closes_slice;
	let closes_for_stds: &[f64] = &closes_slice;

	let means = indicators_core::sma(closes_for_means, Some(mean_period))?;

	let mstd_config = indicators_core::MSTDConfig {
		period: Some(std_period),
	};
	let stds = indicators_core::mstd(closes_for_stds, Some(mstd_config))?;

	let mut z_scores = Vec::with_capacity(data_len);
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < min_period - 1 {
			z_scores.push(0.0);
			signals.push(0);
			continue;
		}

		let mean = means[i];
		let std = stds[i];

		if std.is_nan() || std == 0.0 {
			z_scores.push(0.0);
			signals.push(0);
			continue;
		}

		let z = (closes_slice[i] - mean) / std;
		z_scores.push(z);

		let signal = if crossed_over(&z_scores, threshold, i as u32) {
			1 // Buy signal: z-score crosses over positive threshold (strong upward momentum)
		} else if crossed_under(&z_scores, -threshold, i as u32) {
			-1 // Sell signal: z-score crosses under negative threshold (strong downward momentum)
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}
