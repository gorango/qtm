#![allow(
	clippy::needless_range_loop,
	clippy::collapsible_else_if,
	clippy::map_clone
)]
#[allow(unused_imports)]
use napi::bindgen_prelude::*;
use napi_derive::napi;

pub mod buy_and_hold;
pub mod composite;
pub mod fundamentals;
pub mod momentum;
pub mod patterns;
pub mod quantamentals;
pub mod registry;
pub mod statistics;
pub mod trend;
pub mod volatility;
pub mod volume;

pub use buy_and_hold::*;
pub use composite::*;
pub use fundamentals::*;
pub use momentum::*;
pub use patterns::*;
pub use quantamentals::*;
pub use registry::*;
pub use statistics::*;
pub use trend::*;
pub use volatility::*;
pub use volume::*;

pub use strategies_core::registry::{get_strategy_registry_impl, StrategyInput};

/// Init
#[napi]
pub fn init() {}

/// Test Function
#[napi]
pub fn test_function() -> f64 {
	42.0
}
