use crate::error::{StrategyError, StrategyResult};

pub fn validate_period(period: u32, min: u32, max: u32, name: &str) -> StrategyResult<()> {
	if period < min {
		return Err(StrategyError::Validation(format!(
			"{} must be at least {}",
			name, min
		)));
	}
	if period > max {
		return Err(StrategyError::Validation(format!(
			"{} must be at most {}",
			name, max
		)));
	}
	Ok(())
}

pub fn validate_threshold(threshold: f64, min: f64, max: f64, name: &str) -> StrategyResult<()> {
	if threshold < min {
		return Err(StrategyError::Validation(format!(
			"{} must be at least {}",
			name, min
		)));
	}
	if threshold > max {
		return Err(StrategyError::Validation(format!(
			"{} must be at most {}",
			name, max
		)));
	}
	Ok(())
}

pub fn validate_data_length(
	data_length: usize,
	required_length: usize,
	strategy_name: &str,
) -> StrategyResult<()> {
	if data_length < required_length {
		return Err(StrategyError::InsufficientData(format!(
			"{} strategy requires at least {} data points, got {}",
			strategy_name, required_length, data_length
		)));
	}
	Ok(())
}

pub fn require_volumes(strategy_name: &str) -> StrategyResult<()> {
	Err(StrategyError::VolumesRequired(strategy_name.to_string()))
}
