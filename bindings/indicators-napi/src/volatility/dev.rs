use indicators_core::volatility::dev::{
	dev as dev_core, mean_absolute_deviation as mad_core, MeanAbsoluteDeviationConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Dev
#[napi]
pub fn dev(values: Float64Array, config: Option<MeanAbsoluteDeviationConfig>) -> Result<Vec<f64>> {
	dev_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Mean Absolute Deviation (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn meanAbsoluteDeviation(
	values: Float64Array,
	config: Option<MeanAbsoluteDeviationConfig>,
) -> Result<Vec<f64>> {
	mad_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
