use napi_derive::napi;

pub mod market;
pub mod momentum;
pub mod patterns;
pub mod shared;
pub mod trend;
pub mod volatility;
pub mod volume;
pub mod warmup;

pub use market::*;
pub use momentum::*;
pub use patterns::*;
pub use shared::*;
pub use trend::*;
pub use volatility::*;
pub use volume::*;
pub use warmup::*;

#[napi]
pub fn init() {}

#[napi]
pub fn test_function() -> f64 {
	42.0
}
