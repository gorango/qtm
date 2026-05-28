use crate::internal::moving_sum::moving_sum_internal;
use crate::{IndicatorError, IndicatorResult};

pub fn vwma(closings: &[f64], volumes: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(20) as usize;

	crate::utils::validation::validate_multiple_arrays(&[closings, volumes])?;

	if closings.is_empty() {
		return Err(IndicatorError::Custom("Arrays cannot be empty".into()));
	}

	let price_volume: Vec<f64> = closings
		.iter()
		.enumerate()
		.map(|(i, close)| close * volumes[i])
		.collect();

	let price_volume_sum = moving_sum_internal(&price_volume, period);
	let volume_sum = moving_sum_internal(volumes, period);

	let result: Vec<f64> = price_volume_sum
		.iter()
		.enumerate()
		.map(|(i, pv)| {
			let v = volume_sum[i];
			if v != 0.0 {
				pv / v
			} else {
				0.0
			}
		})
		.collect();

	Ok(result)
}
