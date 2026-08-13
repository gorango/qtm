use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ATRResult {
	pub tr_line: Vec<f64>,
	pub atr_line: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ATRConfig {
	pub period: Option<u32>,
}

impl Default for ATRConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

pub fn atr(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<ATRConfig>,
) -> IndicatorResult<ATRResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;

	let tr_line = tr_internal(highs, lows, closings);
	let atr_line = rma_internal(&tr_line, period);

	Ok(ATRResult { tr_line, atr_line })
}

pub fn average_true_range(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<ATRConfig>,
) -> IndicatorResult<ATRResult> {
	atr(highs, lows, closings, config)
}
