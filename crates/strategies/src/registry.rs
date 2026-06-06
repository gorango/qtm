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

		// Composite strategies (manually registered — combine 2+ indicators)
		register_strategy!(registry, "obv_rsi", obv_rsi);
		register_strategy!(registry, "adx_rsi", adx_rsi);
		register_strategy!(registry, "bb_rsi", bb_rsi);
		register_strategy!(registry, "double_top_stochastic", double_top_stochastic);
		register_strategy!(registry, "flag_pennant_macd", flag_pennant_macd);
		register_strategy!(registry, "ma_rsi", ma_rsi);
		register_strategy!(registry, "macd_rsi", macd_rsi);
		register_strategy!(registry, "macd_stochastic", macd_stochastic);
		register_strategy!(registry, "mfi_obv", mfi_obv);
		register_strategy!(registry, "roc_obv_rsi", roc_obv_rsi);
		register_strategy!(registry, "rsi_macd", rsi_macd);
		register_strategy!(registry, "triangle_rsi", triangle_rsi);
		register_strategy!(registry, "volume_profile_rsi", volume_profile_rsi);
		register_strategy!(registry, "vwap_ema_rsi", vwap_ema_rsi);
		register_strategy!(registry, "vwap_macd", vwap_macd);
		register_strategy!(registry, "vwap_rsi", vwap_rsi);
		register_strategy!(registry, "vwap_stochastic", vwap_stochastic);

		// Base strategies not yet migrated to #[strategy]
		registry
	})
}

// ── Wrapper functions ──────────

pub fn bb_rsi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<BbRsiConfig>(c).unwrap_or_default());
	crate::bb_rsi_strategy(&input.closes, config)
}

pub fn ma_rsi(input: &StrategyInput, config: Option<serde_json::Value>) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MaRsiConfig>(c).unwrap_or_default());
	crate::ma_rsi_strategy(&input.closes, config)
}

pub fn obv_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RSIConfig>(c).unwrap_or_default());
	crate::obv_rsi_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn macd_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MacdStochasticConfig>(c).unwrap_or_default());
	crate::macd_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn volume_profile_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolumeProfileRsiConfig>(c).unwrap_or_default());
	crate::volume_profile_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn double_top_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DoubleTopStochasticConfig>(c).unwrap_or_default());
	crate::double_top_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn roc_obv_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RocObvRsiConfig>(c).unwrap_or_default());
	crate::roc_obv_rsi_strategy(
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn macd_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MACDConfig>(c).unwrap_or_default());
	crate::macd_rsi_strategy(&input.closes, config, None)
}

pub fn adx_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<AdxRsiConfig>(c).unwrap_or_default());
	crate::adx_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn rsi_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RsiMacdConfig>(c).unwrap_or_default());
	crate::rsi_macd_strategy(&input.closes, config)
}

pub fn mfi_obv(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MfiObvConfig>(c).unwrap_or_default());
	crate::mfi_obv_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_ema_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapEmaRsiConfig>(c).unwrap_or_default());
	crate::vwap_ema_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn triangle_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<TriangleRsiConfig>(c).unwrap_or_default());
	crate::triangle_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
}

pub fn vwap_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapMacdConfig>(c).unwrap_or_default());
	crate::vwap_macd_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_stochastic(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VwapStochasticConfig>(c).unwrap_or_default());
	crate::vwap_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn vwap_rsi(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwapRsiConfig>(c).unwrap_or_default());
	crate::vwap_rsi_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input
			.volumes
			.as_ref()
			.ok_or_else(|| StrategyError::VolumesRequired("Volumes required".into()))?,
		config,
	)
}

pub fn flag_pennant_macd(
	input: &StrategyInput,
	config: Option<serde_json::Value>,
) -> StrategyResult<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FlagsPennantsConfig>(c).unwrap_or_default());
	crate::flag_pennant_macd_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
		None,
	)
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
		("adx-rsi-trend-momentum", crate::adx_rsi_strategy_metadata as fn() -> serde_json::Value),
		("bb-rsi-breakout", crate::bb_rsi_strategy_metadata),
		("double-top-stochastic-reversal", crate::double_top_stochastic_strategy_metadata),
		("flag-pennant-macd-continuation", crate::flag_pennant_macd_strategy_metadata),
		("ma-rsi-trend-following", crate::ma_rsi_strategy_metadata),
		("macd-rsi-momentum", crate::macd_rsi_strategy_metadata),
		("macd-stochastic-confirmation", crate::macd_stochastic_strategy_metadata),
		("mfi-obv-volume-flow", crate::mfi_obv_strategy_metadata),
		("obv-rsi-volume-confirmation", crate::obv_rsi_strategy_metadata),
		("roc-obv-rsi-momentum", crate::roc_obv_rsi_strategy_metadata),
		("rsi-macd-confirmation", crate::rsi_macd_strategy_metadata),
		("triangle-rsi-breakout", crate::triangle_rsi_strategy_metadata),
		("volume-profile-rsi", crate::volume_profile_rsi_strategy_metadata),
		("vwap-ema-rsi-trend", crate::vwap_ema_rsi_strategy_metadata),
		("vwap-macd-momentum", crate::vwap_macd_strategy_metadata),
		("vwap-rsi-breakout", crate::vwap_rsi_strategy_metadata),
		("vwap-stochastic-confirmation", crate::vwap_stochastic_strategy_metadata),
		// Aliases for strategies registered under alternate keys
		("percentRank-ranking", crate::percent_rank_strategy_metadata),
		("elliott-wave-pattern", crate::percent_rank_strategy_metadata),
	] {
		strategies.insert(
			key.to_string(),
			serde_json::from_value(meta_fn()).expect("valid strategy metadata"),
		);
	}

	crate::types::results::StrategyRegistry { strategies }
}
