use indicators_core::balance_of_power as bop_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Balance Of Power
#[napi]
pub fn balance_of_power(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
) -> Result<Vec<f64>> {
	bop_core(
		opens.as_ref(),
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
	)
	.map_err(napi::Error::from_reason)
}
