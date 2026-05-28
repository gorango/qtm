use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FactorError {
	#[error("Missing required field: {0}")]
	MissingField(String),

	#[error("Invalid or non-finite value for field: {0}")]
	InvalidValue(String),

	#[error("Insufficient data: {0}")]
	InsufficientData(String),

	#[error("Calculation failed: {0}")]
	CalculationError(String),
}

pub type FactorResult<T> = std::result::Result<T, FactorError>;
