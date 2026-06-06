use napi_derive::napi;

pub mod registry;

/// Utility to verify the module loads and returns the version.
#[napi]
pub fn init() -> f64 {
	0.1
}
