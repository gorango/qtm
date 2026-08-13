use crate::internal::sma::sma_internal;
use crate::utils::rolling::rolling_max;
use crate::{IndicatorError, IndicatorResult};

pub fn ui(closings: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let len = closings.len();

	if len == 0 {
		return Err(IndicatorError::Custom(
			"Closings array cannot be empty".into(),
		));
	}

	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[closings])?;

	let high_closings = rolling_max(closings, period);

	let mut percentage_drawdown = vec![f64::NAN; len];

	for i in 0..len {
		let close = closings[i];
		let high = high_closings[i];

		if !high.is_nan() && high != 0.0 {
			percentage_drawdown[i] = 100.0 * ((close - high) / high);
		}
	}

	let squared_drawdown: Vec<f64> = percentage_drawdown
		.iter()
		.map(|&val| if !val.is_nan() { val * val } else { f64::NAN })
		.collect();

	let squared_average = sma_internal(&squared_drawdown, period);

	let result: Vec<f64> = squared_average
		.iter()
		.map(|&val| {
			if !val.is_nan() && val >= 0.0 {
				val.sqrt()
			} else {
				f64::NAN
			}
		})
		.collect();

	Ok(result)
}

pub fn ulcer_index(closings: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	ui(closings, period)
}
