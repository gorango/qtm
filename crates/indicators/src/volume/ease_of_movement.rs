use crate::internal::sma::sma_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;

pub fn ease_of_movement(highs: &[f64], lows: &[f64], volumes: &[f64], period: u32) -> Vec<f64> {
	validate_arrays_equal_length(&[highs, lows, volumes]).unwrap();
	validate_period(period as usize).unwrap();

	let len = highs.len();
	let period = period as usize;

	let mut midpoints = Vec::with_capacity(len);
	let mut distance_moved = Vec::with_capacity(len);
	let mut box_ratios = Vec::with_capacity(len);
	let mut emv1 = Vec::with_capacity(len);

	for i in 0..len {
		midpoints.push((highs[i] + lows[i]) / 2.0);

		if i == 0 {
			distance_moved.push(midpoints[i]);
		} else {
			distance_moved.push(midpoints[i] - midpoints[i - 1]);
		}

		let denominator = highs[i] - lows[i];
		let br = if denominator.abs() > 1e-10 {
			volumes[i] / 100_000_000.0 / denominator
		} else {
			0.0
		};
		box_ratios.push(br);
		emv1.push(if br != 0.0 {
			distance_moved[i] / br
		} else {
			0.0
		});
	}

	sma_internal(&emv1, period)
}

pub fn emv(highs: &[f64], lows: &[f64], volumes: &[f64], period: u32) -> Vec<f64> {
	ease_of_movement(highs, lows, volumes, period)
}
