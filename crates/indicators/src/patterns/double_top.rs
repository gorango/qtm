use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Double Top — M-shaped reversal with two peaks near same level.
/// Returns scores 0..100 per bar. Heuristic.
pub fn double_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let tolerance = tolerance.unwrap_or(0.03);
	let min_separation = min_separation.unwrap_or(10) as usize;
	let lookaround = lookaround.unwrap_or(2) as usize;

	let mut results = vec![0.0; highs.len()];

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, lookaround);

	if peaks.len() < 2 {
		return Ok(results);
	}

	for i in 1..peaks.len() {
		let p2_index = peaks[i];
		let p1_index = peaks[i - 1];

		if p2_index - p1_index < min_separation {
			continue;
		}

		let p1_price = highs[p1_index];
		let p2_price = highs[p2_index];

		if (p1_price - p2_price).abs() / p1_price > tolerance {
			continue;
		}

		let neckline_price = lows[p1_index..p2_index]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));

		for k in (p2_index + 1)..highs.len() {
			if closes[k] < neckline_price {
				results[k] = -1.0;
				break;
			}
		}
	}

	Ok(results)
}
