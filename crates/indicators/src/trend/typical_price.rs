use crate::IndicatorResult;
/// Typical price kernel: `(high + low + close) / 3` per bar. No validation; callers must validate.
pub fn typical_price_internal(highs: &[f64], lows: &[f64], closings: &[f64]) -> Vec<f64> {
	highs
		.iter()
		.enumerate()
		.map(|(i, high)| (high + lows[i] + closings[i]) / 3.0)
		.collect()
}

/// Typical Price — `(high + low + close) / 3`.
///
/// Common input to CCI and similar indicators. Direct definition. No warmup.
///
/// # Errors
/// Returns an error if input lengths mismatch.
pub fn typical_price(highs: &[f64], lows: &[f64], closings: &[f64]) -> IndicatorResult<Vec<f64>> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;
	crate::utils::validation::validate_finite(&[highs, lows, closings])?;
	Ok(typical_price_internal(highs, lows, closings))
}
