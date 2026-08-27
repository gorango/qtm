use crate::utils::arrays::validate_arrays_equal_length;

/// Anchored VWAP — VWAP computed from `anchor_idx` forward.
/// `VWAP = Σ(price*volume)/Σ(volume)` from anchor. Useful for event-anchored levels.
/// Returns `NaN` before anchor.
pub fn anchored_vwap(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	anchor_index: u32,
) -> Vec<f64> {
	if validate_arrays_equal_length(&[highs, lows, closes, volumes]).is_err() {
		return vec![];
	}

	let len = highs.len();
	let mut result = vec![f64::NAN; len];

	let anchor = anchor_index as usize;

	if anchor >= len {
		return result;
	}

	let mut cumulative_volume = 0.0;
	let mut cumulative_volume_price = 0.0;

	for i in anchor..len {
		let typical_price = (highs[i] + lows[i] + closes[i]) / 3.0;
		let volume = volumes[i];

		cumulative_volume += volume;
		cumulative_volume_price += typical_price * volume;

		result[i] = cumulative_volume_price / cumulative_volume;
	}

	result
}
