use crate::utils::arrays::validate_arrays_equal_length;

pub fn accumulation_distribution(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volume: &[f64],
) -> Vec<f64> {
	validate_arrays_equal_length(&[highs, lows, closings, volume]).unwrap();

	let len = highs.len();
	let mut result = vec![f64::NAN; len];

	let mut cumulative = 0.0;
	for i in 0..len {
		let denominator = highs[i] - lows[i];
		let mfm = if denominator.abs() > 1e-10 {
			(closings[i] - lows[i] - (highs[i] - closings[i])) / denominator
		} else {
			0.0
		};
		let mfv = mfm * volume[i];

		cumulative += mfv;
		result[i] = cumulative;
	}

	result
}

pub fn ad(highs: &[f64], lows: &[f64], closings: &[f64], volume: &[f64]) -> Vec<f64> {
	accumulation_distribution(highs, lows, closings, volume)
}
