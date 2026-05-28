use crate::IndicatorResult;
pub fn typical_price_internal(highs: &[f64], lows: &[f64], closings: &[f64]) -> Vec<f64> {
	highs
		.iter()
		.enumerate()
		.map(|(i, high)| (high + lows[i] + closings[i]) / 3.0)
		.collect()
}

pub fn typical_price(highs: &[f64], lows: &[f64], closings: &[f64]) -> IndicatorResult<Vec<f64>> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;
	Ok(typical_price_internal(highs, lows, closings))
}
