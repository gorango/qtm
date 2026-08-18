use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Head and shoulders (regular, bearish) and inverse head and shoulders
/// (bullish).
///
/// Pivot scan: the three most recent alternating peaks (or troughs) form the
/// left shoulder, head, and right shoulder. The head must exceed both
/// shoulders and the two shoulders must be roughly level (within `tolerance`
/// of their mean). A regular H&S fires a bearish signal (-1) when a low
/// breaks the neckline (the lowest low between the shoulders); the inverse
/// fires bullish (+1) when a high breaks the mirrored neckline. `deviation`,
/// when > 0, pre-filters the highs/lows with a zig-zag so only swings larger
/// than that percentage are considered pivots.
pub fn head_and_shoulders(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_distance: Option<u32>,
	tolerance: Option<f64>,
	deviation: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
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

	// `saturating_sub(2)` matters: with exactly 3 pivots the loop must still
	// run once (index 0 on its own). With `saturating_sub(3)` it would loop
	// zero times and never fire.
	for i in 0..peaks.len().saturating_sub(2) {
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
					// Regular H&S: breakdown below the neckline is bearish.
					results[j] = -1.0;
					break;
				}
			}
		}
	}

	for i in 0..troughs.len().saturating_sub(2) {
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
					// Inverse H&S: breakout above the neckline is bullish.
					results[j] = 1.0;
					break;
				}
			}
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_head_and_shoulders_breakdown() {
		// Regular H&S top with exactly three peaks: left shoulder (10,105),
		// head (30,110), right shoulder (50,104). The neckline is the lowest
		// low between the shoulders (~100 at bar 20). A decline through it at
		// bar 61 fires a bearish signal.
		let pivots = [
			(0, 100.0),
			(10, 105.0),
			(20, 100.0),
			(30, 110.0),
			(40, 102.0),
			(50, 104.0),
			(60, 101.0),
			(62, 97.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = head_and_shoulders(&opens, &highs, &lows, &closes, None, None, None).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(
			idx >= 55,
			"signal should fire after the right shoulder, got {idx}"
		);
	}

	#[test]
	fn detects_inverse_head_and_shoulders_breakout() {
		// Inverse H&S bottom with exactly three troughs: (10,95), (30,90),
		// (50,96). The neckline is the highest high between the troughs (~100
		// at bar 20). A rise through it at bar 61 fires a bullish signal.
		let pivots = [
			(0, 100.0),
			(10, 95.0),
			(20, 100.0),
			(30, 90.0),
			(40, 98.0),
			(50, 96.0),
			(60, 99.0),
			(62, 103.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = head_and_shoulders(&opens, &highs, &lows, &closes, None, None, None).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(
			idx >= 55,
			"signal should fire after the right shoulder, got {idx}"
		);
	}
}
