use indicators_core::chande_forecast_oscillator as cfo_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Chande Forecast Oscillator
#[napi]
pub fn chande_forecast_oscillator(values: Float64Array) -> Result<Vec<f64>> {
	cfo_core(values.as_ref()).map_err(|e| napi::Error::from_reason(e.to_string()))
}
