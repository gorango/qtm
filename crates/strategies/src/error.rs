use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum StrategyError {
	#[error("Validation error: {0}")]
	Validation(String),

	#[error("Insufficient data: {0}")]
	InsufficientData(String),

	#[error("Volumes required: {0}")]
	VolumesRequired(String),

	#[error("Config error: {0}")]
	ConfigError(String),

	#[error("Strategy not found: {0}")]
	NotFound(String),

	#[error(transparent)]
	IndicatorError(#[from] indicators_core::IndicatorError),
}

pub type StrategyResult<T> = std::result::Result<T, StrategyError>;
