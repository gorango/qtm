use crate::types::configs::PercentRankConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};
use serde_json;

/// Percent Rank Strategy
///
/// Generates buy signals when percent rank crosses over entry percentile
/// Generates sell signals when percent rank crosses under exit percentile
///
/// @strategy_id percentRank-ranking
/// @strategy_name Percent Rank Strategy
/// @category statistics
/// @default_timeframes 1h,4h,1d
pub fn percent_rank_strategy(
	closes: &[f64],
	config: Option<PercentRankConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let entry_percentile = config.entry_percentile.unwrap_or(80.0);
	let exit_percentile = config.exit_percentile.unwrap_or(50.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Period must be between 2 and 100".into(),
		));
	}
	if !(0.0..=100.0).contains(&entry_percentile) || !(0.0..=100.0).contains(&exit_percentile) {
		return Err(StrategyError::Validation(
			"Percentiles must be between 0 and 100".into(),
		));
	}

	let data_len = closes.len();
	if data_len < period as usize + 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Percent Rank strategy".into(),
		));
	}

	let pr_config = indicators_core::PercentRankConfig {
		period: Some(period),
	};
	let ranks = indicators_core::percent_rank(closes, Some(pr_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < period as usize {
			signals.push(0);
			continue;
		}

		let rank = ranks[i];
		if rank.is_nan() {
			signals.push(0);
			continue;
		}

		let signal = if crossed_over(&ranks, entry_percentile, i as u32) {
			1 // Buy signal: rank crosses over entry percentile
		} else if crossed_under(&ranks, exit_percentile, i as u32) {
			-1 // Sell signal: rank crosses under exit percentile
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn percent_rank_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "percentRank-ranking",
		"name": "Percent Rank Strategy",
		"category": "statistics",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when percent rank crosses over entry percentile and sell signals when percent rank crosses under exit percentile"
	})
}

pub fn percent_rank_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"entryPercentile": 80.0,
			"exitPercentile": 50.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "entryPercentile",
				"min": 50.0,
				"max": 95.0,
				"step": 5.0
			},
			{
				"param_name": "exitPercentile",
				"min": 5.0,
				"max": 50.0,
				"step": 5.0
			}
		]
	})
}
