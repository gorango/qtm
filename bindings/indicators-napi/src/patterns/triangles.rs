use crate::utils::validation::validate_multiple_arrays;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Triangles
///
/// Detects ascending/descending/symmetrical triangles.
#[napi]
pub fn triangles(
	opens: Float64Array,
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	min_points: Option<u32>,
	tolerance: Option<f64>,
	convergence_tolerance: Option<f64>,
) -> Result<Vec<f64>> {
	validate_multiple_arrays(&[&opens, &highs, &lows, &closes])
		.map_err(napi::Error::from_reason)?;

	let highs = highs.as_ref();
	let lows = lows.as_ref();
	let closes = closes.as_ref();
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

	let filtered_peaks: Vec<usize> = peaks
		.iter()
		.copied()
		.filter(|&p| p > (highs.len() as f64 * 0.3) as usize)
		.collect();
	let recent_peaks: Vec<usize> = filtered_peaks
		.iter()
		.rev()
		.take(min_points)
		.copied()
		.collect();

	let filtered_troughs: Vec<usize> = troughs
		.iter()
		.copied()
		.filter(|&t| t > (lows.len() as f64 * 0.3) as usize)
		.collect();
	let recent_troughs: Vec<usize> = filtered_troughs
		.iter()
		.rev()
		.take(min_points)
		.copied()
		.collect();

	if recent_peaks.len() < 2 || recent_troughs.len() < 2 {
		return Ok(results);
	}

	let mut peak_points = Vec::new();
	for &p in &recent_peaks {
		peak_points.push(p as f64);
		peak_points.push(highs[p]);
	}

	let mut trough_points = Vec::new();
	for &t in &recent_troughs {
		trough_points.push(t as f64);
		trough_points.push(lows[t]);
	}

	let high_line = crate::patterns::helpers::linear_regression_internal(&peak_points);
	let low_line = crate::patterns::helpers::linear_regression_internal(&trough_points);

	let high_slope = high_line[0].abs();
	let low_slope = low_line[0].abs();

	let triangle_type = if high_slope < tolerance && low_slope > convergence_tolerance {
		Some("ascending")
	} else if low_slope < tolerance && high_slope > convergence_tolerance {
		Some("descending")
	} else if high_line[0] < -convergence_tolerance && low_line[0] > convergence_tolerance {
		let convergence = high_line[0].abs() + low_line[0];
		if convergence > convergence_tolerance {
			Some("symmetrical")
		} else {
			None
		}
	} else {
		None
	};

	let triangle_type = match triangle_type {
		Some(t) => t,
		None => return Ok(results),
	};

	let end_index = *recent_peaks
		.last()
		.unwrap_or(&0)
		.max(recent_troughs.last().unwrap_or(&0));

	for i in (end_index + 1)..highs.len() {
		let close = closes[i];

		let breakout = match triangle_type {
			"ascending" => {
				let resistance = high_line[1] + high_line[0] * i as f64;
				if close > resistance {
					Some(1.0)
				} else {
					None
				}
			}
			"descending" => {
				let support = low_line[1] + low_line[0] * i as f64;
				if close < support {
					Some(-1.0)
				} else {
					None
				}
			}
			"symmetrical" => {
				let resistance = high_line[1] + high_line[0] * i as f64;
				let support = low_line[1] + low_line[0] * i as f64;
				if close > resistance {
					Some(1.0)
				} else if close < support {
					Some(-1.0)
				} else {
					None
				}
			}
			_ => None,
		};

		if let Some(signal) = breakout {
			results[i] = signal;
			break;
		}
	}

	Ok(results)
}
