use crate::IndicatorResult;
/// Annualized Volatility — `std(log returns) * sqrt(252)` over `period` bars.
///
/// Uses 252 trading days/year. Period defaults to 20. `NaN` until `period` bars.
/// Direct definition from log-return standard deviation.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn annualized_volatility(
	prices: &[f64],
	trading_days: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	let len = prices.len();
	let trading_days = trading_days.unwrap_or(252) as f64;

	if len < 2 {
		return Ok(vec![0.0; len]);
	}

	let mut result = vec![0.0; len];

	for i in 0..len {
		if i < 2 {
			result[i] = 0.0;
			continue;
		}

		let series = &prices[0..=i];
		let mut log_returns = Vec::with_capacity(series.len() - 1);

		for j in 1..series.len() {
			log_returns.push((series[j] / series[j - 1]).ln());
		}

		let sum: f64 = log_returns.iter().sum();
		let mean = sum / log_returns.len() as f64;

		let variance_sum: f64 = log_returns
			.iter()
			.map(|&val| {
				let diff = val - mean;
				diff * diff
			})
			.sum();

		let variance = variance_sum / (log_returns.len() - 1) as f64;
		let stdev = variance.sqrt();
		let annualized_vol = stdev * trading_days.sqrt();

		result[i] = if annualized_vol.is_nan() {
			0.0
		} else {
			annualized_vol
		};
	}

	Ok(result)
}
