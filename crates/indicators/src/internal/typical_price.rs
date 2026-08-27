/// Typical price kernel — `(high+low+close)/3` per bar. No validation.
pub fn typical_price_internal(highs: &[f64], lows: &[f64], closes: &[f64]) -> Vec<f64> {
	let len = highs.len();
	(0..len)
		.map(|i| (highs[i] + lows[i] + closes[i]) / 3.0)
		.collect()
}
