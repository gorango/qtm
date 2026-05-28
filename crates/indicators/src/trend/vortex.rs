use crate::internal::moving_sum::moving_sum_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct VortexResult {
	pub plus: Vec<f64>,
	pub minus: Vec<f64>,
}

pub fn vortex(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
) -> Result<VortexResult, String> {
	if highs.is_empty() || lows.is_empty() || closings.is_empty() {
		return Err("Arrays cannot be empty".to_string());
	}

	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let period = period.unwrap_or(14) as usize;

	let len = highs.len();
	let mut plus_vm = vec![0.0; len];
	let mut minus_vm = vec![0.0; len];

	for i in 0..len {
		let prev_low = if i == 0 { 0.0 } else { lows[i - 1] };
		let prev_high = if i == 0 { 0.0 } else { highs[i - 1] };

		plus_vm[i] = (highs[i] - prev_low).abs();
		minus_vm[i] = (lows[i] - prev_high).abs();
	}

	let plus_vm_sum = moving_sum_internal(&plus_vm, period);
	let minus_vm_sum = moving_sum_internal(&minus_vm, period);

	let mut true_range = vec![0.0; len];
	for i in 0..len {
		let prev_close = if i == 0 { 0.0 } else { closings[i - 1] };
		let range1 = highs[i] - lows[i];
		let range2 = (highs[i] - prev_close).abs();
		let range3 = (lows[i] - prev_close).abs();
		true_range[i] = range1.max(range2).max(range3);
	}
	let tr_sum = moving_sum_internal(&true_range, period);

	let mut plus = vec![0.0; len];
	let mut minus = vec![0.0; len];

	for i in 0..len {
		let tr_sum_val = tr_sum[i];
		plus[i] = if tr_sum_val != 0.0 {
			plus_vm_sum[i] / tr_sum_val
		} else {
			0.0
		};
		minus[i] = if tr_sum_val != 0.0 {
			minus_vm_sum[i] / tr_sum_val
		} else {
			0.0
		};
	}

	Ok(VortexResult { plus, minus })
}
