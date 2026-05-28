use crate::internal::sma::sma_internal;
use crate::IndicatorResult;

pub fn tma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;

	let (n1, n2) = if period.is_multiple_of(2) {
		(period / 2, period / 2 + 1)
	} else {
		let n = period.div_ceil(2);
		(n, n)
	};

	let sma1 = sma_internal(values, n2);
	let result = sma_internal(&sma1, n1);

	Ok(result)
}
