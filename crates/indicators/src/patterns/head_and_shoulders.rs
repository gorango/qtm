use crate::utils::validation::validate_multiple_arrays;

pub fn head_and_shoulders(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_distance: Option<u32>,
	tolerance: Option<f64>,
	deviation: Option<f64>,
) -> Result<Vec<f64>, String> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let mut highs_vec = highs.to_vec();
	let mut lows_vec = lows.to_vec();
	let min_distance = min_distance.unwrap_or(5) as usize;
	let tolerance = tolerance.unwrap_or(0.02);
	let deviation = deviation.unwrap_or(0.0);

	if deviation > 0.0 {
		highs_vec = crate::patterns::helpers::zig_zag_filter_internal(&highs_vec, deviation);
		lows_vec = crate::patterns::helpers::zig_zag_filter_internal(&lows_vec, deviation);
	}

	let highs = highs_vec.as_slice();
	let lows = lows_vec.as_slice();

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 15 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 2);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 2);

	for i in 0..peaks.len().saturating_sub(3) {
		let left_shoulder = peaks[i];
		let head = peaks[i + 1];
		let right_shoulder = peaks[i + 2];

		if head - left_shoulder < min_distance || right_shoulder - head < min_distance {
			continue;
		}

		let left_shoulder_price = highs[left_shoulder];
		let head_price = highs[head];
		let right_shoulder_price = highs[right_shoulder];

		let shoulder_avg = (left_shoulder_price + right_shoulder_price) / 2.0;
		let shoulder_diff = (left_shoulder_price - right_shoulder_price).abs();
		let shoulder_tolerance = shoulder_avg * tolerance;

		if head_price > left_shoulder_price
			&& head_price > right_shoulder_price
			&& shoulder_diff <= shoulder_tolerance
		{
			let neckline_start = left_shoulder.max(head - (head - left_shoulder));
			let neckline_end = right_shoulder.min(head + (right_shoulder - head));

			let neckline_low = lows[neckline_start..=neckline_end]
				.iter()
				.fold(f64::INFINITY, |a, &b| a.min(b));

			for j in (right_shoulder + 1)..highs.len() {
				if lows[j] < neckline_low {
					results[j] = 1.0;
					break;
				}
			}
		}
	}

	for i in 0..troughs.len().saturating_sub(3) {
		let left_shoulder = troughs[i];
		let head = troughs[i + 1];
		let right_shoulder = troughs[i + 2];

		if head - left_shoulder < min_distance || right_shoulder - head < min_distance {
			continue;
		}

		let left_shoulder_price = lows[left_shoulder];
		let head_price = lows[head];
		let right_shoulder_price = lows[right_shoulder];

		let shoulder_avg = (left_shoulder_price + right_shoulder_price) / 2.0;
		let shoulder_diff = (left_shoulder_price - right_shoulder_price).abs();
		let shoulder_tolerance = shoulder_avg * tolerance;

		if head_price < left_shoulder_price
			&& head_price < right_shoulder_price
			&& shoulder_diff <= shoulder_tolerance
		{
			let neckline_start = left_shoulder.max(head - (head - left_shoulder));
			let neckline_end = right_shoulder.min(head + (right_shoulder - head));

			let neckline_high = highs[neckline_start..=neckline_end]
				.iter()
				.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

			for j in (right_shoulder + 1)..highs.len() {
				if highs[j] > neckline_high {
					results[j] = -1.0;
					break;
				}
			}
		}
	}

	Ok(results)
}
