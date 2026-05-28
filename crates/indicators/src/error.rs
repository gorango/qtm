use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IndicatorError {
	#[error("Period must be greater than 0, got {0}")]
	InvalidPeriod(usize),

	#[error("Input array must have at least {min} elements, got {actual}")]
	InsufficientData { min: usize, actual: usize },

	#[error("Arrays must have equal length: {0}")]
	ArrayLengthMismatch(String),

	#[error("At least one array must be provided")]
	EmptyInput,

	#[error("Unknown indicator: {0}")]
	UnknownIndicator(String),

	#[error("{0}")]
	Custom(String),
}

pub type IndicatorResult<T> = std::result::Result<T, IndicatorError>;

impl From<IndicatorError> for String {
	fn from(e: IndicatorError) -> String {
		e.to_string()
	}
}
