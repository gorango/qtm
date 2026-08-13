use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;

pub fn chaikin_money_flow(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	period: u32,
) -> Vec<f64> {
	if validate_arrays_equal_length(&[highs, lows, closings, volumes]).is_err() {
		return vec![];
	}
	if validate_period(period as usize).is_err() {
		return vec![];
	}

	let len = highs.len();
	let mut result = vec![f64::NAN; len];
	let period = period as usize;

	for (i, result_val) in result.iter_mut().enumerate().take(len) {
		if period == 0 {
			*result_val = f64::NAN;
			continue;
		}
		let mut sum_mfv = 0.0;
		let mut sum_vol = 0.0;
		let start = if i >= period.saturating_sub(1) {
			i.saturating_sub(period).saturating_add(1)
		} else {
			0
		};

		for j in start..=i {
			let h = highs[j];
			let l = lows[j];
			let c = closings[j];
			let v = volumes[j];

			let denominator = h - l;
			let mfm = if denominator.abs() > 1e-10 {
				(c - l - (h - c)) / denominator
			} else {
				0.0
			};
			let mfv = mfm * v;

			sum_mfv += mfv;
			sum_vol += v;
		}

		*result_val = if sum_vol != 0.0 {
			sum_mfv / sum_vol
		} else {
			0.0
		};
	}

	result
}

pub fn cmf(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	period: u32,
) -> Vec<f64> {
	chaikin_money_flow(highs, lows, closings, volumes, period)
}
