use indicators_core::{
	percentile_linear_interpolation as percentile_linear_interpolation_core,
	PercentileLinearInterpolationConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Percentile Linear Interpolation
#[napi]
pub fn percentile_linear_interpolation(
	values: Float64Array,
	config: Option<PercentileLinearInterpolationConfig>,
) -> Result<Vec<f64>> {
	percentile_linear_interpolation_core(values.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
