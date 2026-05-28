use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

pub fn trix(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;

	let ema1 = ema_internal(values, period);
	let ema2 = ema_internal(&ema1, period);
	let ema3 = ema_internal(&ema2, period);

	let mut result = vec![f64::NAN; ema3.len()];
	for i in 1..ema3.len() {
		let current = ema3[i];
		let previous = ema3[i - 1];

		if current.is_nan() || previous.is_nan() {
			result[i] = f64::NAN;
		} else if previous != 0.0 {
			result[i] = (current - previous) / previous;
		} else {
			result[i] = 0.0;
		}
	}

	Ok(result)
}
