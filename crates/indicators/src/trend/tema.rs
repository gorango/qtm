use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

pub fn tema(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(2) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let ema1 = ema_internal(values, period);
	let ema2 = ema_internal(&ema1, period);
	let ema3 = ema_internal(&ema2, period);

	let result: Vec<f64> = ema1
		.iter()
		.enumerate()
		.map(|(i, e1)| 3.0 * e1 - 3.0 * ema2[i] + ema3[i])
		.collect();

	Ok(result)
}
