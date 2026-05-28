use indicators_core::{correlation as correlation_core, CorrelationConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

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
