use crate::trend::wma::wma_internal;
use crate::IndicatorResult;

pub fn hma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(16) as usize;
	crate::utils::validation::validate_period(period)?;
	let half_period = period / 2;
	crate::utils::validation::validate_period(half_period)?;
	let sqrt_period = (period as f64).sqrt() as usize;
	crate::utils::validation::validate_period(sqrt_period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let wma_half = wma_internal(values, half_period);
	let wma_full = wma_internal(values, period);

	let diff: Vec<f64> = wma_half
		.iter()
		.enumerate()
		.map(|(i, h)| {
			let f = wma_full[i];
			if h.is_nan() || f.is_nan() {
				f64::NAN
			} else {
				2.0 * h - f
			}
		})
		.collect();

	Ok(wma_internal(&diff, sqrt_period))
}
