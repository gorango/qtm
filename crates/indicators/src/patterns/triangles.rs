use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

pub fn triangles(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	tolerance: Option<f64>,
	convergence_tolerance: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_points = min_points.unwrap_or(4) as usize;
	let tolerance = tolerance.unwrap_or(0.01);
	let convergence_tolerance = convergence_tolerance.unwrap_or(0.001);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 20 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 1);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 1);

	if peaks.len() < 2 || troughs.len() < 2 {
		return Ok(results);
	}

	// Sliding-window scan: at every bar, fit the min_points most recent
	// peaks/troughs inside the lookback, classify, and fire on a breakout
	// CROSSING of the fitted lines.  Previously the detector was
	// end-anchored — it examined only the final min_points peaks after an
	// arbitrary `index > len*0.3` cutoff, then required the breakout in the
	// few remaining bars, so mid-history triangles were never detected.
	let lookback = 120usize;
	for i in lookback..highs.len() {
		let win_peaks: Vec<usize> = peaks
			.iter()
			.copied()
			.filter(|&p| p >= i - lookback && p < i)
			.collect();
		let win_troughs: Vec<usize> = troughs
			.iter()
			.copied()
			.filter(|&t| t >= i - lookback && t < i)
			.collect();

		if win_peaks.len() < min_points || win_troughs.len() < min_points {
			continue;
		}

		let mut peak_points = Vec::with_capacity(min_points * 2);
		for &p in win_peaks.iter().rev().take(min_points).rev() {
			peak_points.push(p as f64);
			peak_points.push(highs[p]);
		}

		let mut trough_points = Vec::with_capacity(min_points * 2);
		for &t in win_troughs.iter().rev().take(min_points).rev() {
			trough_points.push(t as f64);
			trough_points.push(lows[t]);
		}

		let high_line = crate::patterns::helpers::linear_regression_internal(&peak_points);
		let low_line = crate::patterns::helpers::linear_regression_internal(&trough_points);

		let high_slope = high_line[0];
		let low_slope = low_line[0];

		let resistance = high_line[1] + high_slope * i as f64;
		let support = low_line[1] + low_slope * i as f64;
		let prev_resistance = high_line[1] + high_slope * (i - 1) as f64;
		let prev_support = low_line[1] + low_slope * (i - 1) as f64;

		let buy_cross = closes[i - 1] <= prev_resistance && closes[i] > resistance;
		let sell_cross = closes[i - 1] >= prev_support && closes[i] < support;

		if high_slope.abs() < tolerance && low_slope > convergence_tolerance {
			// ascending: flat highs, rising lows — bullish
			if buy_cross {
				results[i] = 1.0;
			}
		} else if low_slope.abs() < tolerance && high_slope < -convergence_tolerance {
			// descending: flat lows, falling highs — bearish
			if sell_cross {
				results[i] = -1.0;
			}
		} else if high_slope < -convergence_tolerance && low_slope > convergence_tolerance {
			// symmetrical: converging lines — either direction
			if buy_cross {
				results[i] = 1.0;
			} else if sell_cross {
				results[i] = -1.0;
			}
		}
	}

	Ok(results)
}
