use crate::utils::arrays::validate_arrays_equal_length;

/// Negative Volume Index (NVI) — accumulates only on days where volume decreased.
/// `NVI += (close - prev_close)/prev_close * 100` when volume < prev_volume. Starts at `start` (default 1000).
pub fn negative_volume_index(closings: &[f64], volumes: &[f64], start: Option<f64>) -> Vec<f64> {
	if validate_arrays_equal_length(&[closings, volumes]).is_err() {
		return vec![];
	}

	let len = closings.len();
	let mut result = vec![f64::NAN; len];
	let start_val = start.unwrap_or(1000.0);

	result[0] = start_val;

	for i in 1..len {
		if volumes[i - 1] < volumes[i] {
			result[i] = result[i - 1];
		} else {
			let prev_nvi = result[i - 1];
			let price_change = (closings[i] - closings[i - 1]) / closings[i - 1];
			result[i] = prev_nvi + price_change * prev_nvi;
		}
	}

	result
}

/// Alias `nvi` for Negative Volume Index.
pub fn nvi(closings: &[f64], volumes: &[f64], start: Option<f64>) -> Vec<f64> {
	negative_volume_index(closings, volumes, start)
}
