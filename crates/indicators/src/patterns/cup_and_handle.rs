use crate::utils::validation::validate_multiple_arrays;

pub fn cup_and_handle(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	cup_depth: Option<f64>,
	handle_retracement: Option<f64>,
	min_duration: Option<u32>,
) -> Result<Vec<f64>, String> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let cup_depth = cup_depth.unwrap_or(0.15);
	let handle_retracement = handle_retracement.unwrap_or(0.3);
	let min_duration = min_duration.unwrap_or(20) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < min_duration {
		return Ok(results);
	}

	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 5);

	if troughs.is_empty() {
		return Ok(results);
	}

	for bottom_index in troughs.iter().rev().take(3).rev() {
		let bottom_index = *bottom_index;

		if bottom_index < min_duration / 2 || bottom_index > highs.len() - min_duration / 2 {
			continue;
		}

		let bottom_price = lows[bottom_index];

		let left_shoulder = (bottom_index.saturating_sub(min_duration / 2)..bottom_index)
			.rev()
			.max_by_key(|&i| (highs[i] as u64).to_le_bytes())
			.map(|i| (i, highs[i]));

		let (_left_shoulder_index, left_shoulder_price) = match left_shoulder {
			Some((idx, price)) => (idx, price),
			None => continue,
		};

		let right_shoulder = (bottom_index + 1..(bottom_index + min_duration / 2).min(highs.len()))
			.max_by_key(|&i| (highs[i] as u64).to_le_bytes())
			.map(|i| (i, highs[i]));

		let (right_shoulder_index, right_shoulder_price) = match right_shoulder {
			Some((idx, price)) => (idx, price),
			None => continue,
		};

		let avg_shoulder = (left_shoulder_price + right_shoulder_price) / 2.0;
		let depth = (avg_shoulder - bottom_price) / avg_shoulder;

		if depth < cup_depth {
			continue;
		}

		let handle_start = right_shoulder_index;
		let handle_end = (handle_start + min_duration / 4).min(highs.len() - 1);

		if handle_end - handle_start < 5 {
			continue;
		}

		let handle_high = highs[handle_start..=handle_end]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
		let handle_low = lows[handle_start..=handle_end]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));

		let retracement = (handle_high - handle_low) / (right_shoulder_price - handle_low);

		if retracement > handle_retracement {
			continue;
		}

		for i in (handle_end + 1)..highs.len() {
			if closes[i] > handle_high {
				results[i] = 1.0;
				break;
			}
		}
	}

	Ok(results)
}
