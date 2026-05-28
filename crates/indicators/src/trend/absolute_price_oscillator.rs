use crate::internal::ema::ema_internal;

pub fn absolute_price_oscillator(
	values: &[f64],
	fast_period: Option<u32>,
	slow_period: Option<u32>,
) -> Result<Vec<f64>, String> {
	let fast_period = fast_period.unwrap_or(14) as usize;
	let slow_period = slow_period.unwrap_or(30) as usize;

	crate::utils::validation::validate_period(fast_period)?;
	crate::utils::validation::validate_period(slow_period)?;

	let fast_ema = ema_internal(values, fast_period);
	let slow_ema = ema_internal(values, slow_period);

	let result: Vec<f64> = fast_ema
		.iter()
		.enumerate()
		.map(|(i, f)| {
			let slow_val = slow_ema[i];
			if slow_val.is_nan() {
				f64::NAN
			} else {
				f - slow_val
			}
		})
		.collect();

	Ok(result)
}
