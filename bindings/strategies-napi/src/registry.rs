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

	// Composite strategies (manually registered — not yet migrated to #[strategy])
	insert_composite(
		&mut strategies,
		"adx-rsi-trend-momentum",
		crate::composite::adx_rsi::adx_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"bb-rsi-breakout",
		crate::composite::bb_rsi::bb_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"double-top-stochastic-reversal",
		crate::composite::double_top_stochastic::double_top_stochastic_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"flag-pennant-macd-continuation",
		crate::composite::flag_pennant_macd::flag_pennant_macd_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"ma-rsi-trend-following",
		crate::composite::ma_rsi::ma_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"macd-rsi-momentum",
		crate::composite::macd_rsi::macd_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"macd-stochastic-confirmation",
		crate::composite::macd_stochastic::macd_stochastic_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"mfi-obv-volume-flow",
		crate::composite::mfi_obv::mfi_obv_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"obv-rsi-volume-confirmation",
		crate::composite::obv_rsi::obv_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"roc-obv-rsi-momentum",
		crate::composite::roc_obv_rsi::roc_obv_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"rsi-macd-confirmation",
		crate::composite::rsi_macd::rsi_macd_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"triangle-rsi-breakout",
		crate::composite::triangle_rsi::triangle_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"volume-profile-rsi",
		crate::composite::volume_profile_rsi::volume_profile_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"vwap-ema-rsi-trend",
		crate::composite::vwap_ema_rsi::vwap_ema_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"vwap-macd-momentum",
		crate::composite::vwap_macd::vwap_macd_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"vwap-rsi-breakout",
		crate::composite::vwap_rsi::vwap_rsi_strategy_metadata(),
	);
	insert_composite(
		&mut strategies,
		"vwap-stochastic-confirmation",
		crate::composite::vwap_stochastic::vwap_stochastic_strategy_metadata(),
	);

	StrategyRegistry { strategies }
}

fn insert_composite(
	map: &mut HashMap<String, StrategyDefinition>,
	id: &str,
	metadata: serde_json::Value,
) {
	if let Ok(def) = serde_json::from_value::<StrategyDefinition>(metadata) {
		map.insert(id.to_string(), def);
	}
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

	// Composite strategy defaults
	insert_default(
		&mut defaults,
		"adx-rsi-trend-momentum",
		crate::composite::adx_rsi::adx_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"bb-rsi-breakout",
		crate::composite::bb_rsi::bb_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"double-top-stochastic-reversal",
		crate::composite::double_top_stochastic::double_top_stochastic_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"flag-pennant-macd-continuation",
		crate::composite::flag_pennant_macd::flag_pennant_macd_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"ma-rsi-trend-following",
		crate::composite::ma_rsi::ma_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"macd-rsi-momentum",
		crate::composite::macd_rsi::macd_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"macd-stochastic-confirmation",
		crate::composite::macd_stochastic::macd_stochastic_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"mfi-obv-volume-flow",
		crate::composite::mfi_obv::mfi_obv_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"obv-rsi-volume-confirmation",
		crate::composite::obv_rsi::obv_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"roc-obv-rsi-momentum",
		crate::composite::roc_obv_rsi::roc_obv_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"rsi-macd-confirmation",
		crate::composite::rsi_macd::rsi_macd_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"triangle-rsi-breakout",
		crate::composite::triangle_rsi::triangle_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"volume-profile-rsi",
		crate::composite::volume_profile_rsi::volume_profile_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"vwap-ema-rsi-trend",
		crate::composite::vwap_ema_rsi::vwap_ema_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"vwap-macd-momentum",
		crate::composite::vwap_macd::vwap_macd_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"vwap-rsi-breakout",
		crate::composite::vwap_rsi::vwap_rsi_strategy_defaults(),
	);
	insert_default(
		&mut defaults,
		"vwap-stochastic-confirmation",
		crate::composite::vwap_stochastic::vwap_stochastic_strategy_defaults(),
	);

	serde_json::Value::Object(defaults)
}

fn insert_default(
	map: &mut serde_json::Map<String, serde_json::Value>,
	id: &str,
	value: serde_json::Value,
) {
	map.insert(id.to_string(), value);
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
