use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PivotPointsResult {
	pub pivot: f64,
	pub r1: f64,
	pub r2: f64,
	pub r3: f64,
	pub s1: f64,
	pub s2: f64,
	pub s3: f64,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct FibonacciPivotPointsResult {
	pub pivot: f64,
	pub r1: f64,
	pub r2: f64,
	pub r3: f64,
	pub r4: f64,
	pub r5: f64,
	pub s1: f64,
	pub s2: f64,
	pub s3: f64,
	pub s4: f64,
	pub s5: f64,
}

pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	let pivot = (high + low + close) / 3.0;

	let r1 = 2.0 * pivot - low;
	let r2 = pivot + (high - low);
	let r3 = high + 2.0 * (pivot - low);

	let s1 = 2.0 * pivot - high;
	let s2 = pivot - (high - low);
	let s3 = low - 2.0 * (high - pivot);

	PivotPointsResult {
		pivot,
		r1,
		r2,
		r3,
		s1,
		s2,
		s3,
	}
}

pub fn fibonacci_pivot_points(high: f64, low: f64, close: f64) -> FibonacciPivotPointsResult {
	let pivot = (high + low + close) / 3.0;
	let range = high - low;

	let r1 = pivot + 0.382 * range;
	let r2 = pivot + 0.618 * range;
	let r3 = pivot + 1.0 * range;
	let r4 = pivot + 1.618 * range;
	let r5 = pivot + 2.618 * range;

	let s1 = pivot - 0.382 * range;
	let s2 = pivot - 0.618 * range;
	let s3 = pivot - 1.0 * range;
	let s4 = pivot - 1.618 * range;
	let s5 = pivot - 2.618 * range;

	FibonacciPivotPointsResult {
		pivot,
		r1,
		r2,
		r3,
		r4,
		r5,
		s1,
		s2,
		s3,
		s4,
		s5,
	}
}

pub fn camarilla_pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	let range = high - low;

	let r1 = close + range * 0.0916;
	let r2 = close + range * 0.183;
	let r3 = close + range * 0.275;

	let s1 = close - range * 0.0916;
	let s2 = close - range * 0.183;
	let s3 = close - range * 0.275;

	let pivot = close;

	PivotPointsResult {
		pivot,
		r1,
		r2,
		r3,
		s1,
		s2,
		s3,
	}
}
