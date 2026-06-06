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
