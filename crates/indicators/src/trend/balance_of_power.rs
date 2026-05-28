use crate::IndicatorResult;
pub fn balance_of_power(
	openings: &[f64],
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
) -> IndicatorResult<Vec<f64>> {
	crate::utils::validation::validate_multiple_arrays(&[openings, highs, lows, closings])?;

	let result: Vec<f64> = openings
		.iter()
		.enumerate()
		.map(|(i, open)| {
			let high = highs[i];
			let low = lows[i];
			let close = closings[i];
			let denominator = high - low;
			if denominator.abs() > 1e-10 {
				(close - open) / denominator
			} else {
				0.0
			}
		})
		.collect();

	Ok(result)
}
