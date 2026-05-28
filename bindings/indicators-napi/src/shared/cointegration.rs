use indicators_core::{cointegration as cointegration_core, CointegrationConfig};
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
