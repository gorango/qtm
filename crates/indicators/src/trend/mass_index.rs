use crate::internal::ema::ema_internal;
use crate::internal::moving_sum::moving_sum_internal;

pub fn mass_index(
	highs: &[f64],
	lows: &[f64],
	ema_period: Option<u32>,
	mi_period: Option<u32>,
) -> Result<Vec<f64>, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows])?;

	let ema_period = ema_period.unwrap_or(9) as usize;
	let mi_period = mi_period.unwrap_or(25) as usize;

	let ranges: Vec<f64> = highs
		.iter()
		.enumerate()
		.map(|(i, high)| high - lows[i])
		.collect();

	let ema1 = ema_internal(&ranges, ema_period);
	let ema2 = ema_internal(&ema1, ema_period);

	let ratio: Vec<f64> = ema1
		.iter()
		.enumerate()
		.map(|(i, e1)| {
			let e2 = ema2[i];
			if e2 != 0.0 {
				e1 / e2
			} else {
				0.0
			}
		})
		.collect();

	Ok(moving_sum_internal(&ratio, mi_period))
}
