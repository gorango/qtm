use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use factors_core::registry::get_factor_descriptors;
use indicators_core::registry::get_indicator_descriptors;
use serde::Serialize;
use strategies_core::registry::{get_strategy_descriptors, get_strategy_registry_impl};

#[derive(Serialize)]
struct RegistryJson {
	version: u8,
	factors: BTreeMap<String, FactorEntry>,
	indicators: BTreeMap<String, IndicatorEntry>,
	strategies: BTreeMap<String, StrategyEntry>,
}

#[derive(Serialize)]
struct FactorEntry {
	id: String,
	name: String,
	category: String,
	description: String,
	needs_prices: bool,
	params_schema: String,
	output_type: String,
}

#[derive(Serialize)]
struct IndicatorEntry {
	id: String,
	name: String,
	category: String,
	description: String,
	params_schema: String,
	output_type: String,
}

#[derive(Serialize)]
struct StrategyEntry {
	id: String,
	name: String,
	category: String,
	default_timeframes: Vec<String>,
	description: String,
	defaults: serde_json::Value,
	params_schema: String,
	output_type: String,
}

fn build_registry() -> String {
	let mut registry = RegistryJson {
		version: 1,
		factors: BTreeMap::new(),
		indicators: BTreeMap::new(),
		strategies: BTreeMap::new(),
	};

	for desc in get_factor_descriptors() {
		registry.factors.insert(
			desc.id.to_string(),
			FactorEntry {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				description: desc.description.to_string(),
				needs_prices: desc.needs_prices,
				params_schema: desc.params_schema.to_string(),
				output_type: desc.output_type.to_string(),
			},
		);
	}

	for desc in get_indicator_descriptors() {
		registry.indicators.insert(
			desc.id.to_string(),
			IndicatorEntry {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				description: desc.description.to_string(),
				params_schema: desc.params_schema.to_string(),
				output_type: desc.output_type.to_string(),
			},
		);
	}

	get_strategy_registry_impl();

	for desc in get_strategy_descriptors() {
		let defaults = (desc.defaults_fn)();
		let params = defaults.get("params").cloned().unwrap_or_default();
		let params_schema_str = (desc.params_schema_fn)();
		registry.strategies.insert(
			desc.id.to_string(),
			StrategyEntry {
				id: desc.id.to_string(),
				name: desc.name.to_string(),
				category: desc.category.to_string(),
				default_timeframes: desc.default_timeframes.iter().map(|s| s.to_string()).collect(),
				description: desc.description.to_string(),
				defaults: params,
				params_schema: params_schema_str.to_string(),
				output_type: desc.output_type.to_string(),
			},
		);
	}

	serde_json::to_string_pretty(&registry).expect("serialize registry")
}

const DEFAULT_PATH: &str = "packages/tools/src/generated/registry.json";

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let is_check = args.iter().any(|a| a == "--check");

	let json = build_registry();

	if is_check {
		let path = Path::new(DEFAULT_PATH);
		if !path.exists() {
			eprintln!("CHECK FAILED: {} does not exist", DEFAULT_PATH);
			std::process::exit(1);
		}
		let existing = std::fs::read_to_string(path).expect("read existing registry");
		if existing == json {
			eprintln!("Registry is up-to-date.");
		} else {
			let diff_path = "/tmp/opencode/registry-new.json";
			let _ = std::fs::create_dir_all(Path::new(diff_path).parent().unwrap());
			let mut f = std::fs::File::create(diff_path).expect("create temp file");
			f.write_all(json.as_bytes()).expect("write temp file");
			eprintln!("CHECK FAILED: registry.json differs from generated output.");
			eprintln!("Run `cargo run -p codegen` to update it.");
			match std::process::Command::new("diff")
				.args(["-u", DEFAULT_PATH, diff_path])
				.output()
			{
				Ok(out) => {
					let diff_str = String::from_utf8_lossy(&out.stdout);
					if !diff_str.is_empty() {
						eprintln!("{}", diff_str);
					}
				}
				Err(_) => {
					eprintln!("(install `diff` to see a diff)");
				}
			}
			std::process::exit(1);
		}
	} else {
		let path = args.get(1).map(|s| s.as_str()).filter(|&s| s != "--check").unwrap_or(DEFAULT_PATH);
		if let Some(parent) = Path::new(path).parent() {
			let _ = std::fs::create_dir_all(parent);
		}
		std::fs::write(path, &json).expect("write registry.json");
		eprintln!("Wrote registry to {}", path);
	}
}
