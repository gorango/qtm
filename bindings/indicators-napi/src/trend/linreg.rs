use indicators_core::{linreg as linreg_core, LinRegConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Linreg
#[napi]
pub fn linreg(values: Float64Array, config: Option<LinRegConfig>) -> Result<Vec<f64>> {
	linreg_core(values.as_ref(), config).map_err(napi::Error::from_reason)
}
