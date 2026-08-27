use crate::utils::arrays::validate_arrays_equal_length;

/// On-Balance Volume (OBV) — cumulative volume signed by price direction.
/// `OBV += volume` if close > prev_close, `-= volume` if close < prev_close. Defined by Joseph Granville.
pub fn obv(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	if validate_arrays_equal_length(&[closings, volumes]).is_err() {
		return vec![];
	}

	let len = closings.len();
	let mut result = vec![f64::NAN; len];

	let mut cumulative = 0.0;
	result[0] = cumulative;

	for i in 1..len {
		if closings[i] > closings[i - 1] {
			cumulative += volumes[i];
		} else if closings[i] < closings[i - 1] {
			cumulative -= volumes[i];
		}
		result[i] = cumulative;
	}

	result
}

/// Alias `on_balance_volume` for OBV (full name).
pub fn on_balance_volume(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	obv(closings, volumes)
}
