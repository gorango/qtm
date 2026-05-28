#![allow(
	clippy::needless_range_loop,
	clippy::collapsible_else_if,
	clippy::map_clone
)]
#[allow(unused_imports)]
use napi::bindgen_prelude::*;
use napi_derive::napi;

pub mod composite;
pub mod fundamentals;
pub mod quantamentals;
pub mod registry;

pub use composite::*;
pub use fundamentals::*;
pub use quantamentals::*;
pub use registry::*;

pub use strategies_core::registry::{get_strategy_registry_impl, StrategyInput};

/// Init
#[napi]
pub fn init() {}

/// Test Function
#[napi]
pub fn test_function() -> f64 {
	42.0
}
