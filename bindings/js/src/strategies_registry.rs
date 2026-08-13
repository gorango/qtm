use napi_derive::napi;
use std::collections::HashMap;
use strategies_core::{registry::get_strategy_descriptors, StrategyDefinition, StrategyRegistry};

/// Registry of all available strategies
#[napi]
pub fn get_strategy_registry() -> StrategyRegistry {
	let _ = crate::get_strategy_registry_impl();
	let mut strategies = HashMap::new();

	// Auto-collected from #[strategy] descriptors
	for desc in get_strategy_descriptors() {
		strategies.insert(
			desc.id.to_string(),
			StrategyDefinition {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				default_timeframes: desc
					.default_timeframes
					.iter()
					.map(|s| s.to_string())
					.collect(),
				description: Some(desc.description.to_string()),
			},
		);
	}

	StrategyRegistry { strategies }
}

/// Get strategies by category
#[napi]
pub fn get_strategies_by_category(category: String) -> Vec<StrategyDefinition> {
	let registry = get_strategy_registry();
	registry
		.strategies
		.values()
		.filter(|s| s.category == category)
		.cloned()
		.collect()
}

/// Get strategy by ID
#[napi]
pub fn get_strategy_by_id(id: String) -> Option<StrategyDefinition> {
	let registry = get_strategy_registry();
	registry.strategies.get(&id).cloned()
}

/// Get all categories
#[napi]
pub fn get_all_categories() -> Vec<String> {
	let registry = get_strategy_registry();
	let mut categories: Vec<String> = registry
		.strategies
		.values()
		.map(|s| s.category.clone())
		.collect();
	categories.sort();
	categories.dedup();
	categories
}

/// Get default parameters for all strategies
#[napi]
pub fn get_strategy_defaults() -> serde_json::Value {
	let mut defaults = serde_json::Map::new();

	// Auto-collected from #[strategy] descriptors
	for desc in get_strategy_descriptors() {
		defaults.insert(desc.id.to_string(), (desc.defaults_fn)());
	}

	serde_json::Value::Object(defaults)
}

/// Get all strategy categories
pub fn get_strategy_categories() -> Vec<String> {
	vec![
		"momentum".to_string(),
		"trend".to_string(),
		"volatility".to_string(),
		"volume".to_string(),
		"patterns".to_string(),
		"statistics".to_string(),
		"composite".to_string(),
		"special".to_string(),
	]
}

/// Run a `#[strategy]`-registered strategy by id with a unified OHLCV input.
///
/// This is the execution entry point that backs the LLM tool surface generated
/// from `registry.json` (`createDynamicStrategyTool`): every id advertised by
/// `getStrategyRegistry()` is dispatchable here through the descriptor handler.
///
/// `config`, when provided, is deserialized with serde — its keys must match
/// the serde field names emitted by `getStrategyDefaults()` (camelCase, e.g.
/// `secondCloses` for pair strategies). Unknown keys are ignored.
///
/// Strategies of the *hand-written* dialect (fundamentals/quantamentals, which
/// take typed `FactorPoint`/`FundamentalPoint`/`Bar` inputs rather than OHLCV)
/// are intentionally NOT registered here — they cannot be expressed as
/// `StrategyInput` and are executed via their dedicated exports instead
/// (e.g. `valueStrategy`, `qarpStrategy`).
#[napi]
pub fn run_strategy(
	id: String,
	input: crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	match crate::get_strategy_registry_impl().get(&id) {
		Some(handler) => {
			handler(&input, config).map_err(|e| napi::Error::from_reason(e.to_string()))
		}
		None => Err(napi::Error::new(
			napi::Status::GenericFailure,
			format!("Unknown strategy: {id}"),
		)),
	}
}
