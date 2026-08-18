use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Broadening / megaphone pattern.
///
/// Diverging trendlines: rising swing highs and falling swing lows. The
/// widening range is a classic sign of distribution/instability. Unlike a
/// wedge, where the lines converge and the breakout is unambiguous, a
/// broadening pattern can resolve either way — the signal fires when a close
/// breaks out through the resistance line (+1) or breaks down through the
/// support line (-1).
pub fn broadening(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_points = min_points.unwrap_or(3) as usize;
	let tolerance = tolerance.unwrap_or(0.0005);
	let lookback = lookback.unwrap_or(120) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < lookback + 5 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 1);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 1);

	if peaks.len() < min_points || troughs.len() < min_points {
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

		if win_peaks.len() < min_points || win_troughs.len() < min_points {
			continue;
		}

		let (high_line, mean_high) = fit_line(&win_peaks, highs);
		let (low_line, mean_low) = fit_line(&win_troughs, lows);

		let high_slope = high_line[0] / mean_high;
		let low_slope = low_line[0] / mean_low;

		// Diverging: higher highs AND lower lows.
		if !(high_slope > tolerance && low_slope < -tolerance) {
			continue;
		}

		let support = low_line[1] + low_line[0] * i as f64;
		let resistance = high_line[1] + high_line[0] * i as f64;

		if closes[i - 1] <= resistance && closes[i] > resistance {
			results[i] = 1.0;
		} else if closes[i - 1] >= support && closes[i] < support {
			results[i] = -1.0;
		}
	}

	Ok(results)
}

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
	fn detects_broadening_breakdown() {
		// Diverging lines: rising highs (10,103) (25,106) (40,109) and falling
		// lows (8,100) (18,97) (33,94). A pullback to (42,102) then a breakdown
		// below the support line (~91.2 at bar 44).
		let pivots = [
			(0, 102.0),
			(8, 100.0),
			(10, 103.0),
			(18, 97.0),
			(25, 106.0),
			(33, 94.0),
			(40, 109.0),
			(42, 102.0),
			(44, 90.0),
			(50, 88.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = broadening(&opens, &highs, &lows, &closes, None, None, Some(40)).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(idx >= 40, "signal should fire on the breakdown, got {idx}");
	}

	#[test]
	fn detects_broadening_breakout() {
		// Same diverging structure resolved to the upside: price tags the lower
		// line at (42,93.5), then a decisive push through the resistance line
		// (~109.8 at bar 44). The pullback sits on the falling low line so it
		// doesn't break the divergence.
		let pivots = [
			(0, 102.0),
			(8, 100.0),
			(10, 103.0),
			(18, 97.0),
			(25, 106.0),
			(33, 94.0),
			(40, 109.0),
			(42, 93.5),
			(44, 112.0),
			(50, 115.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = broadening(&opens, &highs, &lows, &closes, None, None, Some(40)).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(idx >= 40, "signal should fire on the breakout, got {idx}");
	}
}
