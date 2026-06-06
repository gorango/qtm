use std::collections::HashMap;
use std::sync::OnceLock;

use crate::StrategyResult;
use crate::*;

#[derive(Clone)]
pub struct StrategyInput {
	pub opens: Option<Vec<f64>>,
	pub highs: Option<Vec<f64>>,
	pub lows: Option<Vec<f64>>,
	pub closes: Vec<f64>,
	pub volumes: Option<Vec<f64>>,
	pub timestamps: Option<Vec<f64>>,
}

/// Descriptor for a registered strategy, collected via `inventory`.
pub struct StrategyDescriptor {
	pub id: &'static str,
	pub name: &'static str,
	pub category: &'static str,
	pub default_timeframes: &'static [&'static str],
	pub description: &'static str,
	pub handler: fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>>,
	pub defaults_fn: fn() -> serde_json::Value,
	/// JSON Schema describing the config parameters for this strategy.
	/// Empty string `""` means no config parameters.
	pub params_schema: &'static str,
	/// Output type: "signal" (Vec<i8>), "factor" (Vec<f64>), "indicator" (Vec<f64>)
	pub output_type: &'static str,
}

inventory::collect!(StrategyDescriptor);

/// Returns all strategy descriptors registered via `#[strategy]`.
pub fn get_strategy_descriptors() -> Vec<&'static StrategyDescriptor> {
	inventory::iter::<StrategyDescriptor>.into_iter().collect()
}

#[macro_export]
macro_rules! register_strategy {
	($registry:expr, $id:expr, $function:expr) => {
		$registry.insert(
			$id.to_string(),
			Box::new($function)
				as Box<
					dyn Fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>>
						+ Send
						+ Sync,
				>,
		);
	};
}

pub type StrategyRegistryImpl = HashMap<
	String,
	Box<dyn Fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>> + Send + Sync>,
>;

static STRATEGY_REGISTRY: OnceLock<StrategyRegistryImpl> = OnceLock::new();

pub fn get_strategy_registry_impl() -> &'static StrategyRegistryImpl {
	STRATEGY_REGISTRY.get_or_init(|| {
		let mut registry = HashMap::new();

		// Collect strategies registered via #[strategy] attribute macro
		for desc in inventory::iter::<StrategyDescriptor> {
			registry.insert(
				desc.id.to_string(),
				Box::new(desc.handler)
					as Box<
						dyn Fn(&StrategyInput, Option<serde_json::Value>) -> StrategyResult<Vec<i8>>
							+ Send
							+ Sync,
					>,
			);
		}

		// Base strategies not yet migrated to #[strategy]
		registry
	})
}

// ── Metadata registry ─────────────────────────────

pub fn get_strategy_registry() -> crate::types::results::StrategyRegistry {
	let mut strategies = HashMap::new();

	for desc in inventory::iter::<StrategyDescriptor> {
		strategies.insert(
			desc.id.to_string(),
			crate::types::results::StrategyDefinition {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				default_timeframes: desc.default_timeframes.iter().map(|s| s.to_string()).collect(),
				description: Some(desc.description.to_string()),
			},
		);
	}

	// Composite strategies (not yet migrated to #[strategy])
	for (key, meta_fn) in [
		("flag-pennant-macd-continuation", crate::flag_pennant_macd_strategy_metadata as fn() -> serde_json::Value),
		("macd-rsi-momentum", crate::macd_rsi_strategy_metadata as fn() -> serde_json::Value),
		// Aliases for strategies registered under alternate keys
		("percentRank-ranking", crate::percent_rank_strategy_metadata as fn() -> serde_json::Value),
		("elliott-wave-pattern", crate::percent_rank_strategy_metadata as fn() -> serde_json::Value),
	] {
		strategies.insert(
			key.to_string(),
			serde_json::from_value(meta_fn()).expect("valid strategy metadata"),
		);
	}

	crate::types::results::StrategyRegistry { strategies }
}
