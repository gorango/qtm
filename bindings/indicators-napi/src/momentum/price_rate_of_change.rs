use indicators_core::{price_rate_of_change as proc_core, PriceRateOfChangeConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Price Rate Of Change
#[napi]
pub fn price_rate_of_change(
	values: Float64Array,
	config: Option<PriceRateOfChangeConfig>,
) -> Result<Vec<f64>> {
	proc_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
