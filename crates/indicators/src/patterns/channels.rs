use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Rising/falling price channels (continuation).
///
/// A channel forms when swing highs and swing lows both trend in the same
/// direction along (near-)parallel lines. Unlike wedges the two lines do not
/// converge; unlike a rectangle they are not horizontal. The signal fires on
/// a breakout through the channel boundary in the direction of the channel:
/// rising channel breaks up (+1), falling channel breaks down (-1).
pub fn channels(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	min_slope: Option<f64>,
	parallelism_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_points = min_points.unwrap_or(3) as usize;
	let min_slope = min_slope.unwrap_or(0.0005);
	let parallelism_tolerance = parallelism_tolerance.unwrap_or(0.5);
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

		let rising = high_slope > min_slope && low_slope > min_slope;
		let falling = high_slope < -min_slope && low_slope < -min_slope;
		if !rising && !falling {
			continue;
		}

		// Parallelism: the normalized slopes must be close to each other.
		let max_mag = high_slope.abs().max(low_slope.abs());
		if (high_slope - low_slope).abs() > parallelism_tolerance * max_mag {
			continue;
		}

		let support = low_line[1] + low_line[0] * i as f64;
		let resistance = high_line[1] + high_line[0] * i as f64;

		if rising && closes[i - 1] <= resistance && closes[i] > resistance {
			results[i] = 1.0;
		} else if falling && closes[i - 1] >= support && closes[i] < support {
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
	fn detects_rising_channel_breakout() {
		// Two parallel rising lines carried by alternating swings: troughs
		// (20,84.9) (40,87.8) (58,92) and peaks (10,88) (30,91) (50,94), then a
		// pullback and an upside breakout (62,96.2).
		let pivots = [
			(0, 84.0),
			(10, 88.0),
			(20, 84.9),
			(30, 91.0),
			(40, 87.8),
			(50, 94.0),
			(58, 92.0),
			(61, 94.9),
			(62, 96.2),
			(64, 98.5),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = channels(&opens, &highs, &lows, &closes, None, None, None, Some(60))
			.unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(idx >= 58, "signal should fire on the breakout, got {idx}");
	}

	#[test]
	fn detects_falling_channel_breakdown() {
		// Mirror: falling parallel lines (peaks 115.1/112.2/108, troughs
		// 112/108.9/106), a pullback to (58,108) and a downside breakdown
		// (62,103.8).
		let pivots = [
			(0, 116.0),
			(10, 112.0),
			(20, 115.1),
			(30, 108.9),
			(40, 112.2),
			(50, 106.0),
			(58, 108.0),
			(61, 105.1),
			(62, 103.8),
			(64, 101.5),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = channels(&opens, &highs, &lows, &closes, None, None, None, Some(60))
			.unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(idx >= 58, "signal should fire on the breakdown, got {idx}");
	}
}
