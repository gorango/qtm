use std::collections::HashMap;

use indicators_core::registry::get_indicator_descriptors;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct IndicatorDefinition {
	pub id: String,
	pub name: String,
	pub category: String,
	pub description: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct IndicatorRegistry {
	pub indicators: HashMap<String, IndicatorDefinition>,
}

#[napi]
pub fn get_indicator_registry() -> IndicatorRegistry {
	let mut indicators = HashMap::new();
	for desc in get_indicator_descriptors() {
		indicators.insert(
			desc.id.to_string(),
			IndicatorDefinition {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				description: Some(desc.description.to_string()),
			},
		);
	}
	IndicatorRegistry { indicators }
}

#[napi]
pub fn get_indicators_by_category(category: String) -> Vec<IndicatorDefinition> {
	let registry = get_indicator_registry();
	registry
		.indicators
		.values()
		.filter(|s| s.category == category)
		.cloned()
		.collect()
}

#[napi]
pub fn get_indicator_by_id(id: String) -> Option<IndicatorDefinition> {
	let registry = get_indicator_registry();
	registry.indicators.get(&id).cloned()
}

#[napi]
pub fn get_all_indicator_categories() -> Vec<String> {
	let registry = get_indicator_registry();
	let mut categories: Vec<String> = registry
		.indicators
		.values()
		.map(|s| s.category.clone())
		.collect();
	categories.sort();
	categories.dedup();
	categories
}
