use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Diamond top (bearish reversal).
///
/// A diamond opens with a broadening phase — higher swing highs and lower
/// swing lows (diverging trendlines) — then narrows into a contracting phase —
/// lower highs and higher lows (converging trendlines). The confirmed signal
/// fires when a close breaks down through the contracting phase's support line.
pub fn diamond_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	tolerance: Option<f64>,
	breakout_threshold: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	diamond_internal(
		opens,
		highs,
		lows,
		closes,
		false,
		min_points,
		tolerance,
		breakout_threshold,
		lookback,
	)
}

/// Diamond bottom (bullish reversal).
///
/// Mirror of [`diamond_top`]: broadening (higher highs, lower lows) then
/// contracting (lower highs, higher lows). Fires when a close breaks up
/// through the contracting phase's resistance line.
pub fn diamond_bottom(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	tolerance: Option<f64>,
	breakout_threshold: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	diamond_internal(
		opens,
		highs,
		lows,
		closes,
		true,
		min_points,
		tolerance,
		breakout_threshold,
		lookback,
	)
}

/// Shared diamond detector.
///
/// Sliding-window scan: within each lookback window the pivot highs and lows
/// are split at their temporal midpoint. The left half must diverge (rising
/// highs + falling lows), the right half must converge (falling highs +
/// rising lows). A breakout of the right half's support/resistance fires the
/// signal. Slopes are normalized relative to the mean pivot price so the
/// tolerance is scale-free.
fn diamond_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	bullish: bool,
	min_points: Option<u32>,
	tolerance: Option<f64>,
	breakout_threshold: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	// Each half needs at least this many pivots per line; 1+2+1 = 4+ peaks /
	// 4+ troughs is the geometric minimum for a diamond.
	let min_points = min_points.unwrap_or(2) as usize;
	let tolerance = tolerance.unwrap_or(0.0005);
	let breakout_threshold = breakout_threshold.unwrap_or(0.0);
	let lookback = lookback.unwrap_or(150) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < lookback + 10 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 1);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 1);

	if peaks.len() < 4 || troughs.len() < 4 {
		return Ok(results);
	}

	for i in lookback..highs.len() {
		let start = i - lookback;

		let win_peaks: Vec<usize> = peaks
			.iter()
			.copied()
			.filter(|&p| p > start && p < i)
			.collect();
		let win_troughs: Vec<usize> = troughs
			.iter()
			.copied()
			.filter(|&t| t > start && t < i)
			.collect();

		// Need at least 2*min_points per pivot type so each half can get
		// >= min_points. Split at the count midpoint: the earlier pivot(s)
		// form the broadening (left) half, the later ones the contracting
		// (right) half. This is robust to uneven phase timing, unlike splitting
		// at the temporal window midpoint.
		if win_peaks.len() < 2 * min_points || win_troughs.len() < 2 * min_points {
			continue;
		}

		let peak_split = win_peaks.len() / 2;
		let trough_split = win_troughs.len() / 2;

		let left_peaks = &win_peaks[..peak_split];
		let right_peaks = &win_peaks[peak_split..];
		let left_troughs = &win_troughs[..trough_split];
		let right_troughs = &win_troughs[trough_split..];

		if left_peaks.len() < min_points
			|| left_troughs.len() < min_points
			|| right_peaks.len() < min_points
			|| right_troughs.len() < min_points
		{
			continue;
		}

		let (high_left, mean_left_high) = fit_line(left_peaks, highs);
		let (low_left, mean_left_low) = fit_line(left_troughs, lows);
		let (high_right, mean_right_high) = fit_line(right_peaks, highs);
		let (low_right, mean_right_low) = fit_line(right_troughs, lows);

		let hl = high_left[0] / mean_left_high;
		let ll = low_left[0] / mean_left_low;
		let hr = high_right[0] / mean_right_high;
		let lr = low_right[0] / mean_right_low;

		// Left half diverges: rising highs, falling lows.
		if !(hl > tolerance && ll < -tolerance) {
			continue;
		}
		// Right half converges: falling highs, rising lows.
		if !(hr < -tolerance && lr > tolerance) {
			continue;
		}

		// The converging lines must still be apart at bar `i` (before the apex).
		let support = low_right[1] + low_right[0] * i as f64;
		let resistance = high_right[1] + high_right[0] * i as f64;
		if support >= resistance {
			continue;
		}

		if bullish {
			let prev_resistance = high_right[1] + high_right[0] * (i - 1) as f64;
			if closes[i - 1] <= prev_resistance
				&& closes[i] > resistance * (1.0 + breakout_threshold)
			{
				results[i] = 1.0;
			}
		} else {
			let prev_support = low_right[1] + low_right[0] * (i - 1) as f64;
			if closes[i - 1] >= prev_support && closes[i] < support * (1.0 - breakout_threshold) {
				results[i] = -1.0;
			}
		}
	}

	Ok(results)
}

/// Fits a line to pivot prices and returns `(line, mean_pivot_price)`.
/// `line = [slope, intercept]` from [`crate::patterns::helpers::linear_regression_internal`].
fn fit_line(pivots: &[usize], prices: &[f64]) -> (Vec<f64>, f64) {
	let mut points = Vec::with_capacity(pivots.len() * 2);
	let mut mean = 0.0;
	for &p in pivots {
		points.push(p as f64);
		points.push(prices[p]);
		mean += prices[p];
	}
	mean /= pivots.len() as f64;

	(
		crate::patterns::helpers::linear_regression_internal(&points),
		mean,
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_diamond_top_breakdown() {
		// Broadening left half: rising highs (30,102) -> (46,106) and falling
		// lows (38,99) -> (54,95). Contracting right half: falling highs
		// (62,104) -> (78,101) -> (86,100) and rising lows (70,97) -> (84,98.5).
		// Then a breakdown below the support line (~98.8 at bar 87).
		let pivots = [
			(0, 100.0),
			(30, 102.0),
			(38, 99.0),
			(46, 106.0),
			(54, 95.0),
			(62, 104.0),
			(70, 97.0),
			(78, 101.0),
			(84, 98.5),
			(86, 100.0),
			(88, 96.0),
			(92, 90.0),
			(100, 88.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals =
			diamond_top(&opens, &highs, &lows, &closes, None, None, None, Some(60)).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(
			idx > 80,
			"signal should fire after the diamond completes, got {idx}"
		);
	}

	#[test]
	fn detects_diamond_bottom_breakout() {
		// Same broadening -> contracting geometry, but the right half's rising
		// lows (70,97) -> (84,98.5) -> (86,99) are broken upward instead.
		let pivots = [
			(0, 100.0),
			(30, 102.0),
			(38, 99.0),
			(46, 106.0),
			(54, 95.0),
			(62, 104.0),
			(70, 97.0),
			(78, 101.0),
			(84, 98.5),
			(86, 99.0),
			(88, 100.2),
			(90, 104.0),
			(92, 108.0),
			(100, 112.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals =
			diamond_bottom(&opens, &highs, &lows, &closes, None, None, None, Some(60)).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(
			idx > 80,
			"signal should fire after the diamond completes, got {idx}"
		);
	}
}
