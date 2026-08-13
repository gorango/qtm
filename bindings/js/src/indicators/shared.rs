use indicators_core::{
	cointegration as cointegration_core, correlation as correlation_core,
	percent_rank as percent_rank_core,
	percentile_linear_interpolation as percentile_linear_interpolation_core,
	percentile_nearest_rank as percentile_nearest_rank_core, value_when as value_when_core,
	CointegrationConfig, CorrelationConfig, PercentRankConfig, PercentileLinearInterpolationConfig,
	PercentileNearestRankConfig, ValueWhenConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Cointegration
#[napi]
pub fn cointegration(
	values1: Float64Array,
	values2: Float64Array,
	config: Option<CointegrationConfig>,
) -> Result<Vec<f64>> {
	cointegration_core(values1.as_ref(), values2.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Correlation
#[napi]
pub fn correlation(
	values1: Float64Array,
	values2: Float64Array,
	config: Option<CorrelationConfig>,
) -> Result<Vec<f64>> {
	correlation_core(values1.as_ref(), values2.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Pearson Correlation
#[napi]
pub fn pearson_correlation(
	values1: Float64Array,
	values2: Float64Array,
	config: Option<CorrelationConfig>,
) -> Result<Vec<f64>> {
	correlation(values1, values2, config)
}

/// Percent Rank
#[napi]
pub fn percent_rank(values: Float64Array, config: Option<PercentRankConfig>) -> Result<Vec<f64>> {
	percent_rank_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Percentile Linear Interpolation
#[napi]
pub fn percentile_linear_interpolation(
	values: Float64Array,
	config: Option<PercentileLinearInterpolationConfig>,
) -> Result<Vec<f64>> {
	percentile_linear_interpolation_core(values.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Percentile Nearest Rank
#[napi]
pub fn percentile_nearest_rank(
	values: Float64Array,
	config: Option<PercentileNearestRankConfig>,
) -> Result<Vec<f64>> {
	percentile_nearest_rank_core(values.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Value When
#[napi]
pub fn value_when(
	condition: Float64Array,
	source: Float64Array,
	config: Option<ValueWhenConfig>,
) -> Result<Vec<f64>> {
	value_when_core(condition.as_ref(), source.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
