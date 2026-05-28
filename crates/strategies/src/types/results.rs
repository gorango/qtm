#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Strategy signal result
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategySignal {
	/// Signal value: 1=buy, -1=sell, 0=hold
	pub signal: i8,
	/// Optional timestamp for the signal
	pub timestamp: Option<String>,
}

/// Strategy metadata for registry
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyDefinition {
	pub id: String,
	pub name: String,
	pub category: String,
	pub default_timeframes: Vec<String>,
	pub description: Option<String>,
}

/// Registry containing all strategy definitions
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug)]
pub struct StrategyRegistry {
	pub strategies: std::collections::HashMap<String, StrategyDefinition>,
}

/// Optimization bounds for strategy parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizationBounds {
	pub param_name: String,
	pub min: f64,
	pub max: f64,
	pub step: f64,
}

/// Default parameters for a strategy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyDefaults {
	pub params: serde_json::Value,
	pub optimization_bounds: Vec<OptimizationBounds>,
}
