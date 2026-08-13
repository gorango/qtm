use crate::IndicatorResult;
pub fn max_drawdown(prices: &[f64], period: u32) -> IndicatorResult<Vec<f64>> {
	let len = prices.len();
	let period = period as usize;

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[prices])?;

	let mut result = vec![0.0; len];

	for i in 0..len {
		if i < period - 1 {
			continue;
		}

		let window_start = i - period + 1;
		let window = &prices[window_start..=i];

		let mut peak = f64::NEG_INFINITY;
		let mut max_dd = 0.0;

		for &price in window {
			if price > peak {
				peak = price;
			}
			let drawdown = (peak - price) / peak;
			if drawdown > max_dd {
				max_dd = drawdown;
			}
		}

		result[i] = max_dd * 100.0;
	}

	Ok(result)
}
