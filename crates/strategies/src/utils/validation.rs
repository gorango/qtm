/// Validate that period parameters are reasonable
pub fn validate_period(period: u32, min: u32, max: u32, name: String) -> Result<(), String> {
	if period < min {
		return Err(format!("{} must be at least {}", name, min));
	}
	if period > max {
		return Err(format!("{} must be at most {}", name, max));
	}
	Ok(())
}

/// Validate threshold parameters
pub fn validate_threshold(threshold: f64, min: f64, max: f64, name: String) -> Result<(), String> {
	if threshold < min {
		return Err(format!("{} must be at least {}", name, min));
	}
	if threshold > max {
		return Err(format!("{} must be at most {}", name, max));
	}
	Ok(())
}

/// Validate that we have enough data for the strategy
pub fn validate_data_length(
	data_length: usize,
	required_length: usize,
	strategy_name: String,
) -> Result<(), String> {
	if data_length < required_length {
		return Err(format!(
			"{} strategy requires at least {} data points, got {}",
			strategy_name, required_length, data_length
		));
	}
	Ok(())
}
