use napi_derive::napi;

/// Utility to verify the module loads and returns the version.
#[napi]
pub fn init() -> f64 {
	0.1
}
