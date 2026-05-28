use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

pub fn double_bottom(
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

	let troughs = crate::patterns::helpers::find_troughs_internal(lows, lookaround);

	if troughs.len() < 2 {
		return Ok(results);
	}

	for i in 1..troughs.len() {
		let b2_index = troughs[i];
		let b1_index = troughs[i - 1];

		if b2_index - b1_index < min_separation {
			continue;
		}

		let b1_price = lows[b1_index];
		let b2_price = lows[b2_index];

		if (b1_price - b2_price).abs() / b1_price > tolerance {
			continue;
		}

		let neckline_price = highs[b1_index..b2_index]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

		for k in (b2_index + 1)..highs.len() {
			if closes[k] > neckline_price {
				results[k] = 1.0;
				break;
			}
		}
	}

	Ok(results)
}
