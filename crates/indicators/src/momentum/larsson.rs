use crate::utils::arrays::validate_arrays_equal_length;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LarssonSignal {
	P1,
	P2,
	P3,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct LarssonResult {
	pub v1: Vec<f64>,
	pub v2: Vec<f64>,
	pub signal: Vec<LarssonSignal>,
}

/// Larsson indicator — price-action trend score from swing highs/lows.
/// Returns signal and per-bar scores. Heuristic; no single canonical formula.
///
/// # Errors
/// Returns an error if inputs mismatched.
pub fn larsson(highs: &[f64], lows: &[f64]) -> LarssonResult {
	let _ = validate_arrays_equal_length(&[highs, lows]);

	let len = highs.len();

	let mut hl2 = vec![0.0; len];
	for i in 0..len {
		hl2[i] = (highs[i] + lows[i]) / 2.0;
	}

	let v1 = crate::internal::smma::smma_internal(&hl2, 15);
	let m1 = crate::internal::smma::smma_internal(&hl2, 19);
	let m2 = crate::internal::smma::smma_internal(&hl2, 25);
	let v2 = crate::internal::smma::smma_internal(&hl2, 29);

	let mut signal = vec![LarssonSignal::P1; len];

	for i in 0..len {
		let v1_val = v1[i];
		let m1_val = m1[i];
		let m2_val = m2[i];
		let v2_val = v2[i];

		if v1_val.is_nan() || m1_val.is_nan() || m2_val.is_nan() || v2_val.is_nan() {
			continue;
		}

		let cond1 = (v1_val < m1_val) != (v1_val < v2_val);
		let cond2 = (m2_val < v2_val) != (v1_val < v2_val);
		let p2 = cond1 || cond2;
		let p3 = !p2 && (v1_val < v2_val);
		let p1 = !p2 && !p3;

		if p1 {
			signal[i] = LarssonSignal::P1;
		} else if p2 {
			signal[i] = LarssonSignal::P2;
		} else if p3 {
			signal[i] = LarssonSignal::P3;
		}
	}

	LarssonResult { v1, v2, signal }
}
