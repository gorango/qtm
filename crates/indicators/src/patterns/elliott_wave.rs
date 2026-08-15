use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

#[allow(clippy::too_many_arguments)]
pub fn elliott_wave(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	wave2_retracement: Option<f64>,
	wave4_retracement: Option<f64>,
	wave3_min_extension: Option<f64>,
	min_wave_separation: Option<u32>,
	lookaround: Option<u32>,
	retracement_tolerance: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let wave2_retracement = wave2_retracement.unwrap_or(0.618);
	let wave4_retracement = wave4_retracement.unwrap_or(0.382);
	let wave3_min_extension = wave3_min_extension.unwrap_or(1.618);
	let min_wave_separation = min_wave_separation.unwrap_or(5) as usize;
	let lookaround = lookaround.unwrap_or(2) as usize;
	let retracement_tolerance = retracement_tolerance.unwrap_or(0.1);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 20 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, lookaround);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, lookaround);

	// Corrective waves need only 2 peaks + 2 troughs; impulse checks no-op
	// below 3.  Block only when BOTH are too small to form anything.
	if peaks.len() < 2 || troughs.len() < 2 {
		return Ok(results);
	}

	check_bullish_impulse_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		retracement_tolerance,
	);

	check_bearish_impulse_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		retracement_tolerance,
	);

	check_bullish_corrective_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		retracement_tolerance,
	);

	check_bearish_corrective_wave(
		&peaks,
		&troughs,
		highs,
		lows,
		closes,
		&mut results[..],
		min_wave_separation,
		retracement_tolerance,
	);

	Ok(results)
}

/// Max peak index in `(lo, hi)`, or None.
fn max_peak_in(peaks: &[usize], highs: &[f64], lo: usize, hi: usize) -> Option<usize> {
	peaks
		.iter()
		.copied()
		.filter(|&p| p > lo && p < hi)
		.max_by(|&a, &b| {
			highs[a]
				.partial_cmp(&highs[b])
				.unwrap_or(std::cmp::Ordering::Equal)
		})
}

/// Min trough index in `(lo, hi)`, or None.
fn min_trough_in(troughs: &[usize], lows: &[f64], lo: usize, hi: usize) -> Option<usize> {
	troughs
		.iter()
		.copied()
		.filter(|&t| t > lo && t < hi)
		.min_by(|&a, &b| {
			lows[a]
				.partial_cmp(&lows[b])
				.unwrap_or(std::cmp::Ordering::Equal)
		})
}

/// Nearest (largest) index strictly below `idx` in `xs`, or None.
fn nearest_below(xs: &[usize], idx: usize) -> Option<usize> {
	xs.iter().copied().filter(|&x| x < idx).max()
}

fn in_range(v: f64, target: f64, tol: f64) -> bool {
	v >= target * (1.0 - tol) && v <= target * (1.0 + tol)
}

// A real 5-wave impulse has THREE troughs (w1-start, w2-low, w4-low) and THREE
// peaks (w1-top, w3-top, w5-top).  The previous detector indexed FIVE troughs
// per sequence (t0-p1-t1-p2-t2-p3-t3-p4-t4 — 4 up-legs + 4 down-legs), which
// no impulse structure can satisfy, so it never fired on any data.
#[allow(clippy::too_many_arguments)]
fn check_bullish_impulse_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	retracement_tolerance: f64,
) {
	// Consecutive trough triples: t0 = w1 start, t1 = w2 low, t2 = w4 low.
	for i in 0..troughs.len().saturating_sub(2) {
		let t0 = troughs[i];
		let t1 = troughs[i + 1];
		let t2 = troughs[i + 2];

		// w1 top = max peak between t0 and t1; w3 top = max peak between t1 and t2.
		let p1 = match max_peak_in(peaks, highs, t0, t1) {
			Some(p) => p,
			None => continue,
		};
		let p2 = match max_peak_in(peaks, highs, t1, t2) {
			Some(p) => p,
			None => continue,
		};

		// legs must be non-degenerate (min_wave_separation bars)
		if p1 - t0 < min_wave_separation
			|| t1 - p1 < min_wave_separation
			|| p2 - t1 < min_wave_separation
			|| t2 - p2 < min_wave_separation
		{
			continue;
		}

		let w1_price = lows[t0];
		let w1_top_price = highs[p1];
		let w2_price = lows[t1];
		let w3_top_price = highs[p2];
		let w4_price = lows[t2];

		// structure: lows rising (w2 > w1, w4 > w2), w3 top above w1 top
		if w2_price <= w1_price || w4_price <= w2_price || w3_top_price <= w1_top_price {
			continue;
		}

		let w1_range = w1_top_price - w1_price;
		let w3_range = w3_top_price - w2_price;
		if w1_range <= 0.0 || w3_range <= 0.0 {
			continue;
		}

		let w2_retracement = (w1_top_price - w2_price) / w1_range;
		let w4_retracement = (w3_top_price - w4_price) / w3_range;
		let w3_extension = w3_range / w1_range;

		if !in_range(w2_retracement, wave2_retracement, retracement_tolerance)
			|| !in_range(w4_retracement, wave4_retracement, retracement_tolerance)
			|| w3_extension < wave3_min_extension
		{
			continue;
		}

		let breakout_level = w1_top_price;
		for j in (t2 + 1)..closes.len() {
			if closes[j] > breakout_level {
				results[j] = 1.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bearish_impulse_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	retracement_tolerance: f64,
) {
	for i in 0..peaks.len().saturating_sub(2) {
		let p0 = peaks[i];
		let p1 = peaks[i + 1];
		let p2 = peaks[i + 2];

		// w1 low = min trough between p0 and p1; w3 low = min trough between p1 and p2.
		let t1 = match min_trough_in(troughs, lows, p0, p1) {
			Some(t) => t,
			None => continue,
		};
		let t2 = match min_trough_in(troughs, lows, p1, p2) {
			Some(t) => t,
			None => continue,
		};

		if t1 - p0 < min_wave_separation
			|| p1 - t1 < min_wave_separation
			|| t2 - p1 < min_wave_separation
			|| p2 - t2 < min_wave_separation
		{
			continue;
		}

		let w1_price = highs[p0];
		let w1_bottom_price = lows[t1];
		let w2_price = highs[p1];
		let w3_bottom_price = lows[t2];
		let w4_price = highs[p2];

		// structure: highs falling (w2 < w1, w4 < w2), w3 low below w1 low
		if w2_price >= w1_price || w4_price >= w2_price || w3_bottom_price >= w1_bottom_price {
			continue;
		}

		let w1_range = w1_price - w1_bottom_price;
		let w3_range = w2_price - w3_bottom_price;
		if w1_range <= 0.0 || w3_range <= 0.0 {
			continue;
		}

		let w2_retracement = (w2_price - w1_bottom_price) / w1_range;
		let w4_retracement = (w4_price - w3_bottom_price) / w3_range;
		let w3_extension = w3_range / w1_range;

		if !in_range(w2_retracement, wave2_retracement, retracement_tolerance)
			|| !in_range(w4_retracement, wave4_retracement, retracement_tolerance)
			|| w3_extension < wave3_min_extension
		{
			continue;
		}

		let breakdown_level = w1_bottom_price;
		for j in (p2 + 1)..closes.len() {
			if closes[j] < breakdown_level {
				results[j] = -1.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bullish_corrective_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	retracement_tolerance: f64,
) {
	// A-B-C: A up (trough -> a_high), B retrace (a_high -> b_low), C up
	// (b_low -> c_high); buy on breakout above the B retracement high.
	for (i, &a_high) in peaks.iter().enumerate() {
		let a_low = nearest_below(troughs, a_high);
		let c_high = peaks.get(i + 1).copied();
		let b_low = c_high.and_then(|c| min_trough_in(troughs, lows, a_high, c));

		let (a_low, c_high, b_low) = match (a_low, c_high, b_low) {
			(Some(a), Some(b), Some(c)) => (a, b, c),
			_ => continue,
		};

		if a_high - a_low < min_wave_separation
			|| b_low - a_high < min_wave_separation
			|| c_high - b_low < min_wave_separation
		{
			continue;
		}

		let a_price = highs[a_high];
		let a_range = a_price - lows[a_low];
		let b_price = lows[b_low];
		let c_price = highs[c_high];
		if a_range <= 0.0 || c_price <= a_price {
			continue;
		}

		// B retraces 0.618-1.0 of A's range (measured low-to-low)
		let b_retracement = (a_price - b_price) / a_range;
		if b_retracement < 0.618 * (1.0 - retracement_tolerance)
			|| b_retracement > 1.0 * (1.0 + retracement_tolerance)
		{
			continue;
		}

		let breakout_level = a_price;
		for j in (c_high + 1)..closes.len() {
			if closes[j] > breakout_level {
				results[j] = 2.0;
				break;
			}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn check_bearish_corrective_wave(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	results: &mut [f64],
	min_wave_separation: usize,
	retracement_tolerance: f64,
) {
	// A-B-C down: A down (peak -> a_low), B retrace (a_low -> b_high), C down
	// (b_high -> c_low); sell on breakdown below the B retracement low.
	for (i, &a_low) in troughs.iter().enumerate() {
		let a_high = nearest_below(peaks, a_low);
		let c_low = troughs.get(i + 1).copied();
		let b_high = c_low.and_then(|c| max_peak_in(peaks, highs, a_low, c));

		let (a_high, c_low, b_high) = match (a_high, c_low, b_high) {
			(Some(a), Some(b), Some(c)) => (a, b, c),
			_ => continue,
		};

		if a_low - a_high < min_wave_separation
			|| b_high - a_low < min_wave_separation
			|| c_low - b_high < min_wave_separation
		{
			continue;
		}

		let a_price = lows[a_low];
		let a_range = highs[a_high] - a_price;
		let b_price = highs[b_high];
		let c_price = lows[c_low];
		if a_range <= 0.0 || c_price >= a_price {
			continue;
		}

		let b_retracement = (b_price - a_price) / a_range;
		if b_retracement < 0.618 * (1.0 - retracement_tolerance)
			|| b_retracement > 1.0 * (1.0 + retracement_tolerance)
		{
			continue;
		}

		let breakdown_level = a_price;
		for j in (c_low + 1)..closes.len() {
			if closes[j] < breakdown_level {
				results[j] = -2.0;
				break;
			}
		}
	}
}
