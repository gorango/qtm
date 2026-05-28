use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct MACDResult {
	pub macd: Vec<f64>,
	pub signal: Vec<f64>,
	pub histogram: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct RSIResult {
	pub rsi: Vec<f64>,
}
