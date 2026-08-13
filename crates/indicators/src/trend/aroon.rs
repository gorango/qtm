use crate::trend::moving_max::moving_max_internal;
use crate::trend::moving_min::moving_min_internal;
use crate::trend::since::since_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct AroonResult {
	pub up: Vec<f64>,
	pub down: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AroonConfig {
	pub period: Option<u32>,
}

pub fn aroon(
	highs: &[f64],
	lows: &[f64],
	config: Option<AroonConfig>,
) -> IndicatorResult<AroonResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows])?;

	let config = config.unwrap_or(AroonConfig { period: Some(25) });
	let period = config.period.unwrap_or(25) as usize;

	let highest = moving_max_internal(highs, period);
	let lowest = moving_min_internal(lows, period);

	let since_last_high = since_internal(&highest);
	let since_last_low = since_internal(&lowest);

	let up: Vec<f64> = since_last_high
		.iter()
		.map(|since_val| ((period as f64 - since_val) / period as f64) * 100.0)
		.collect();

	let down: Vec<f64> = since_last_low
		.iter()
		.map(|since_val| ((period as f64 - since_val) / period as f64) * 100.0)
		.collect();

	Ok(AroonResult { up, down })
}
