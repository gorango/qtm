use indicators_core::{
	camarilla_pivot_points as cpp_core, fibonacci_pivot_points as fpp_core,
	pivot_points as pp_core, FibonacciPivotPointsResult, PivotPointsResult,
};
use napi_derive::napi;

/// Pivot Points
#[napi]
pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	pp_core(high, low, close)
}

/// Fibonacci Pivot Points
#[napi]
pub fn fibonacci_pivot_points(high: f64, low: f64, close: f64) -> FibonacciPivotPointsResult {
	fpp_core(high, low, close)
}

/// Camarilla Pivot Points
#[napi]
pub fn camarilla_pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
	cpp_core(high, low, close)
}
