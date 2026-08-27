use crate::IndicatorResult;
/// Balance of Power (BOP).
///
/// `(close - open) / (high - low)` per bar; `0` when `high == low`. Range `[-1, 1]`.
/// Measures buying vs selling pressure within the bar. Direct definition.
/// No warmup; outputs `0` for flat bars.
///
/// # Errors
/// Returns an error if input arrays have mismatched lengths.
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
