use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::utils::rolling::rolling_max_growing;
use crate::utils::rolling::rolling_min_growing;
use crate::{IndicatorError, IndicatorResult};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct CEResult {
	/// Long exit — `highest_high(period) - multiplier*ATR`.
	pub long: Vec<f64>,
	/// Short exit — `lowest_low(period) + multiplier*ATR`.
	pub short: Vec<f64>,
}

/// Chandelier Exit — `ce` short alias.
/// Volatility trailing stops. Long = highest_high - multiplier*ATR.
pub fn ce(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
) -> IndicatorResult<CEResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let len = highs.len();

	if len == 0 {
		return Err(IndicatorError::Custom(
			"Highs, lows, and closings arrays cannot be empty".into(),
		));
	}

	let period = period.unwrap_or(22) as usize;
	crate::utils::validation::validate_period(period)?;

	let tr_line = tr_internal(highs, lows, closings);
	let atr_line = rma_internal(&tr_line, period);

	let atr3: Vec<f64> = atr_line.iter().map(|&val| val * 3.0).collect();

	let highest_high = rolling_max_growing(highs, period);
	let lowest_low = rolling_min_growing(lows, period);

	let mut long = vec![f64::NAN; len];
	let mut short = vec![f64::NAN; len];

	for i in 0..len {
		let hh = highest_high[i];
		let ll = lowest_low[i];
		let atr_val = atr3[i];

		if !hh.is_nan() && !atr_val.is_nan() {
			long[i] = hh - atr_val;
		}

		if !ll.is_nan() && !atr_val.is_nan() {
			short[i] = ll + atr_val;
		}
	}

	Ok(CEResult { long, short })
}

/// Chandelier Exit — volatility-based trailing stops.
///
/// Long stop = `highest_high(period) - multiplier*ATR(period)`;
/// short stop = `lowest_low(period) + multiplier*ATR(period)`.
/// Moves only in the favorable direction (ratchets). Defined by Chuck LeBeau.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn chandelier_exit(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
) -> IndicatorResult<CEResult> {
	ce(highs, lows, closings, period)
}
