use crate::internal::ema::ema_internal;
use crate::utils::rolling::rolling_max_growing;
use crate::utils::rolling::rolling_min_growing;
use crate::{IndicatorError, IndicatorResult};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
struct LinearRegressionResult {
	m: f64,
	b: f64,
}

fn moving_least_square(period: usize, x: &[usize], y: &[f64]) -> Vec<LinearRegressionResult> {
	let len = y.len();
	let mut result = Vec::with_capacity(len);

	for i in 0..len {
		let start = if period > 0 && i + 1 >= period {
			i - (period - 1)
		} else {
			0
		};
		let end = i + 1;
		let x_slice = &x[start..end];
		let y_slice = &y[start..end];

		let n = x_slice.len();
		let mut sum_x = 0.0;
		let mut sum_y = 0.0;
		let mut sum_xy = 0.0;
		let mut sum_xx = 0.0;

		for j in 0..n {
			let xi = x_slice[j] as f64;
			let yi = y_slice[j];
			sum_x += xi;
			sum_y += yi;
			sum_xy += xi * yi;
			sum_xx += xi * xi;
		}

		let denominator = (n as f64) * sum_xx - sum_x * sum_x;
		let (m, b) = if denominator.abs() > 1e-10 {
			let m_val = ((n as f64) * sum_xy - sum_x * sum_y) / denominator;
			let b_val = (sum_y - m_val * sum_x) / (n as f64);
			(m_val, b_val)
		} else {
			(0.0, 0.0)
		};

		result.push(LinearRegressionResult { m, b });
	}

	result
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct POResult {
	pub po_result: Vec<f64>,
	pub spo_result: Vec<f64>,
}

/// Projection Oscillator — `po` short alias. Bounded 0..100.
pub fn po(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
	smooth: Option<u32>,
) -> IndicatorResult<POResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let len = highs.len();

	if len == 0 {
		return Err(IndicatorError::Custom(
			"Highs, lows, and closings arrays cannot be empty".into(),
		));
	}

	let period = period.unwrap_or(14) as usize;
	let smooth = smooth.unwrap_or(3) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_period(smooth)?;

	let x: Vec<usize> = (0..len).collect();

	let ls_highs = moving_least_square(period, &x, highs);
	let ls_lows = moving_least_square(period, &x, lows);

	let mut v_highs = vec![0.0; len];
	let mut v_lows = vec![0.0; len];

	for i in 0..len {
		v_highs[i] = highs[i] + ls_highs[i].m * x[i] as f64;
		v_lows[i] = lows[i] + ls_lows[i].m * x[i] as f64;
	}

	let pu = rolling_max_growing(&v_highs, period);
	let pl = rolling_min_growing(&v_lows, period);

	let mut po_result = vec![0.0; len];

	for i in 0..len {
		let pu_val = pu[i];
		let pl_val = pl[i];
		let close = closings[i];

		if !pu_val.is_nan() && !pl_val.is_nan() {
			let denominator = pu_val - pl_val;
			po_result[i] = if denominator.abs() > 1e-10 {
				100.0 * (close - pl_val) / denominator
			} else {
				0.0
			};
		}
	}

	let spo_result = ema_internal(&po_result, smooth);

	Ok(POResult {
		po_result,
		spo_result,
	})
}

/// Projection Oscillator — normalized position of close within projected highs/lows. Full name.
/// `PO = 100*(close - min_proj)/(max_proj - min_proj)` via linear regression over period. `NaN` until warmup.
pub fn projection_oscillator(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
	smooth: Option<u32>,
) -> IndicatorResult<POResult> {
	po(highs, lows, closings, period, smooth)
}
