use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Wedge pattern detector.
///
/// Rising wedge: both trendlines slope up with the high line steeper than the
/// low line. Falling wedge: both slope down with the high line less steep
/// than the low line. The signal fires on the bar a close crosses the broken
/// line — down through support for a rising wedge (-1), up through resistance
/// for a falling wedge (+1).
///
/// Causal rolling-window scan: for every bar the formation is fitted to the
/// most recent `min_points` peaks/troughs inside the trailing `lookback`
/// window only, so the signal at bar `i` depends solely on data up to `i`.
/// Slopes are normalized by mean pivot price so `slope_tolerance` is
/// scale-free across symbols.
pub fn wedges(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	slope_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_points = min_points.unwrap_or(4).max(2) as usize;
	let slope_tolerance = slope_tolerance.unwrap_or(0.0001);
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

		// Most recent `min_points` pivots inside the window.
		let recent_peaks = &win_peaks[win_peaks.len() - min_points..];
		let recent_troughs = &win_troughs[win_troughs.len() - min_points..];

		let mut peak_points = Vec::with_capacity(min_points * 2);
		for &p in recent_peaks {
			peak_points.push(p as f64);
			peak_points.push(highs[p]);
		}

		let mut trough_points = Vec::with_capacity(min_points * 2);
		for &t in recent_troughs {
			trough_points.push(t as f64);
			trough_points.push(lows[t]);
		}

		let high_line = crate::patterns::helpers::linear_regression_internal(&peak_points);
		let low_line = crate::patterns::helpers::linear_regression_internal(&trough_points);

		// Scale-free slopes (price fraction per bar).
		let mean_high = peak_points[1..].iter().step_by(2).sum::<f64>() / min_points as f64;
		let mean_low = trough_points[1..].iter().step_by(2).sum::<f64>() / min_points as f64;
		if mean_high <= 0.0 || mean_low <= 0.0 {
			continue;
		}
		let hs = high_line[0] / mean_high;
		let ls = low_line[0] / mean_low;

		// Rising: both lines up, high line steeper. Falling: mirror.
		let wedge_type = if hs > ls && hs > slope_tolerance && ls > slope_tolerance {
			Some("rising")
		} else if hs < ls && hs < -slope_tolerance && ls < -slope_tolerance {
			Some("falling")
		} else {
			None
		};

		let support = low_line[1] + low_line[0] * i as f64;
		let prev_support = low_line[1] + low_line[0] * (i - 1) as f64;
		let resistance = high_line[1] + high_line[0] * i as f64;
		let prev_resistance = high_line[1] + high_line[0] * (i - 1) as f64;

		match wedge_type {
			Some("rising") => {
				if closes[i - 1] >= prev_support && closes[i] < support {
					results[i] = -1.0;
				}
			}
			Some("falling") => {
				if closes[i - 1] <= prev_resistance && closes[i] > resistance {
					results[i] = 1.0;
				}
			}
			_ => {}
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_rising_wedge_breakdown() {
		// Rising maxima (8,100) (20,104) (32,108) over rising minima
		// (14,95) (26,97): high line steeper than low line. The collapse off
		// the last peak crosses the projected support at bar 38 (-1).
		let pivots = [
			(0, 96.0),
			(8, 100.0),
			(14, 95.0),
			(20, 104.0),
			(26, 97.0),
			(32, 108.0),
			(40, 93.0),
		];
		let closes = series_from_pivots(&pivots, 42);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = wedges(&opens, &highs, &lows, &closes, Some(2), None, Some(25)).unwrap();

		assert!(
			signals.iter().any(|&s| s < -0.5),
			"no bearish breakdown signal"
		);
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert_eq!(idx, 37, "signal should fire on the breakdown bar");
	}

	#[test]
	fn detects_falling_wedge_breakout() {
		// Falling maxima (8,110) (24,104) over gently falling minima
		// (16,100) (32,99): lines converge downward. The recovery leg crosses
		// the projected resistance at bar 34 (+1).
		let pivots = [
			(0, 104.0),
			(8, 110.0),
			(16, 100.0),
			(24, 104.0),
			(32, 99.0),
			(44, 108.0),
		];
		let closes = series_from_pivots(&pivots, 46);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = wedges(&opens, &highs, &lows, &closes, Some(2), None, Some(30)).unwrap();

		assert!(
			signals.iter().any(|&s| s > 0.5),
			"no bullish breakout signal"
		);
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert_eq!(idx, 34, "signal should fire on the breakout bar");
	}

	#[test]
	fn no_signal_without_wedge_geometry() {
		// Flat oscillation around 100: no common-direction slopes.
		let pivots = [
			(0, 100.0),
			(8, 104.0),
			(16, 96.0),
			(24, 104.0),
			(32, 96.0),
			(40, 104.0),
			(48, 96.0),
			(52, 100.0),
		];
		let closes = series_from_pivots(&pivots, 56);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = wedges(&opens, &highs, &lows, &closes, Some(3), None, Some(45)).unwrap();

		assert!(
			signals.iter().all(|&s| s == 0.0),
			"unexpected signal on non-wedge series"
		);
	}
}
