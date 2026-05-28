use indicators_core::volatility::moving_standard_deviation::{
	moving_standard_deviation as msd_core, mstd as mstd_core, MSTDConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Mstd
#[napi]
pub fn mstd(values: Float64Array, config: Option<MSTDConfig>) -> Result<Vec<f64>> {
	mstd_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Moving Standard Deviation (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn movingStandardDeviation(
	values: Float64Array,
	config: Option<MSTDConfig>,
) -> Result<Vec<f64>> {
	msd_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
