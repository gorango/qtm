use std::collections::HashMap;

use factors_core::registry::get_factor_descriptors;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct FactorDefinition {
	pub id: String,
	pub name: String,
	pub category: String,
	pub description: Option<String>,
	pub needs_prices: bool,
}

#[napi(object)]
#[derive(Clone)]
pub struct FactorRegistry {
	pub factors: HashMap<String, FactorDefinition>,
}

#[napi]
pub fn get_factor_registry() -> FactorRegistry {
	let mut factors = HashMap::new();
	for desc in get_factor_descriptors() {
		factors.insert(
			desc.id.to_string(),
			FactorDefinition {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				description: Some(desc.description.to_string()),
				needs_prices: desc.needs_prices,
			},
		);
	}
	FactorRegistry { factors }
}

#[napi]
pub fn get_factors_by_category(category: String) -> Vec<FactorDefinition> {
	let registry = get_factor_registry();
	registry
		.factors
		.values()
		.filter(|s| s.category == category)
		.cloned()
		.collect()
}

#[napi]
pub fn get_factor_by_id(id: String) -> Option<FactorDefinition> {
	let registry = get_factor_registry();
	registry.factors.get(&id).cloned()
}

#[napi]
pub fn get_all_factor_categories() -> Vec<String> {
	let registry = get_factor_registry();
	let mut categories: Vec<String> = registry
		.factors
		.values()
		.map(|s| s.category.clone())
		.collect();
	categories.sort();
	categories.dedup();
	categories
}
