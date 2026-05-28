use crate::internal::ema::ema_internal;

pub fn dema(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(12) as usize;
	crate::utils::validation::validate_period(period)?;

	let ema1 = ema_internal(values, period);
	let ema2 = ema_internal(&ema1, period);

	let result: Vec<f64> = ema1
		.iter()
		.enumerate()
		.map(|(i, e1)| 2.0 * e1 - ema2[i])
		.collect();

	Ok(result)
}
