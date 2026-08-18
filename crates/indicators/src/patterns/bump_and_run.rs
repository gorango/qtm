use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Bump-and-Run Reversal (BARR).
///
/// Three phases: a steep lead-in trendline, an accelerated "bump" away from
/// that line, and a return back through it. When price crosses back through
/// the (extrapolated) lead-in line the move is considered exhausted and a
/// reversal signal fires — bullish after a lead-in decline, bearish after a
/// lead-in advance.
pub fn bump_and_run(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	lead_in_bars: Option<u32>,
	min_slope: Option<f64>,
	bump_threshold: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let lead_in_bars = lead_in_bars.unwrap_or(20) as usize;
	let min_slope = min_slope.unwrap_or(0.001);
	let bump_threshold = bump_threshold.unwrap_or(0.03);
	let lookback = lookback.unwrap_or(80) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < lookback + 5 || lookback <= lead_in_bars + 5 {
		return Ok(results);
	}

	for i in lookback..highs.len() {
		let start = i - lookback;
		let lead_end = start + lead_in_bars;

		// Lead-in line fit on closes in the first segment of the window.
		let mut lead_points = Vec::with_capacity(lead_in_bars * 2);
		let mut mean = 0.0;
		for (k, &c) in closes[start..lead_end].iter().enumerate() {
			lead_points.push((start + k) as f64);
			lead_points.push(c);
			mean += c;
		}
		mean /= lead_in_bars as f64;

		let line = crate::patterns::helpers::linear_regression_internal(&lead_points);
		let slope = line[0] / mean;
		if slope.abs() < min_slope {
			continue;
		}

		// Bump: maximum distance of closes from the lead-in line after it.
		let mut max_dev = 0.0_f64;
		for (k, &c) in closes[lead_end..i].iter().enumerate() {
			let projected = line[1] + line[0] * (lead_end + k) as f64;
			max_dev = max_dev.max((c - projected).abs());
		}
		max_dev /= mean;
		if max_dev < bump_threshold {
			continue;
		}

		// Return: close crosses back through the lead-in line at bar `i`.
		let proj_prev = line[1] + line[0] * (i - 1) as f64;
		let proj_cur = line[1] + line[0] * i as f64;

		if slope > 0.0 && closes[i - 1] > proj_prev && closes[i] < proj_cur {
			// Lead-in up, bump above, fall back through -> bearish.
			results[i] = -1.0;
		} else if slope < 0.0 && closes[i - 1] < proj_prev && closes[i] > proj_cur {
			// Lead-in down, bump below, reclaim the line -> bullish.
			results[i] = 1.0;
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_bullish_bump_and_run() {
		// Steep lead-in decline (100 -> 95), bump below the line (78), then a
		// return back through the extrapolated line.
		let pivots = [
			(0, 100.0),
			(20, 95.0),
			(35, 78.0),
			(48, 86.0),
			(50, 89.0),
			(60, 94.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals =
			bump_and_run(&opens, &highs, &lows, &closes, Some(20), None, None, Some(50)).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert_eq!(idx, 50, "signal should fire at the return crossing");
	}

	#[test]
	fn detects_bearish_bump_and_run() {
		let pivots = [
			(0, 100.0),
			(20, 105.0),
			(35, 122.0),
			(48, 114.0),
			(50, 111.0),
			(60, 106.0),
		];
		let closes = series_from_pivots(&pivots, 70);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals =
			bump_and_run(&opens, &highs, &lows, &closes, Some(20), None, None, Some(50)).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert_eq!(idx, 50, "signal should fire at the return crossing");
	}
}
