use crate::internal::ema::ema_internal;
use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct KCResult {
	pub upper: Vec<f64>,
	pub middle: Vec<f64>,
	pub lower: Vec<f64>,
}

pub fn kc(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
) -> Result<KCResult, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let len = highs.len();

	let period = period.unwrap_or(20) as usize;
	crate::utils::validation::validate_period(period)?;

	let tr_line = tr_internal(highs, lows, closings);
	let atr_line = rma_internal(&tr_line, period);

	let atr2: Vec<f64> = atr_line.iter().map(|&val| val * 2.0).collect();

	let middle = ema_internal(closings, period);

	let mut upper = vec![f64::NAN; len];
	let mut lower = vec![f64::NAN; len];

	for i in 0..len {
		let mid = middle[i];
		let atr_val = atr2[i];

		if !mid.is_nan() && !atr_val.is_nan() {
			upper[i] = mid + atr_val;
			lower[i] = mid - atr_val;
		}
	}

	Ok(KCResult {
		upper,
		middle,
		lower,
	})
}

pub fn keltner_channel(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
) -> Result<KCResult, String> {
	kc(highs, lows, closings, period)
}
