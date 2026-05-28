use crate::error::{StrategyError, StrategyResult};
use indicators_core::validate_min_data;
use indicators_core::validate_range;

pub fn validate_period(period: u32, min: u32, max: u32, name: &str) -> StrategyResult<()> {
	validate_range!(period, min, max, name, |msg| {
		StrategyError::Validation(msg)
	});
	Ok(())
}

pub fn validate_threshold(threshold: f64, min: f64, max: f64, name: &str) -> StrategyResult<()> {
	validate_range!(threshold, min, max, name, |msg| {
		StrategyError::Validation(msg)
	});
	Ok(())
}

pub fn validate_data_length(
	data_length: usize,
	required_length: usize,
	strategy_name: &str,
) -> StrategyResult<()> {
	validate_min_data!(data_length, required_length, |min, actual| {
		StrategyError::InsufficientData(format!(
			"{} strategy requires at least {} data points, got {}",
			strategy_name, min, actual
		))
	});
	Ok(())
}

pub fn require_volumes(strategy_name: &str) -> StrategyResult<()> {
	Err(StrategyError::VolumesRequired(strategy_name.to_string()))
}
