use pyo3::prelude::*;
use serde_json::{json, Map, Value};

use crate::convert::{err, i8_out, json_to_py, normalize_config};
use crate::convert::{Json, PyObject};
use strategies_core::registry::{
	get_strategy_descriptors, get_strategy_registry_impl, StrategyInput,
};

type PyResultO = PyResult<PyObject>;

fn strategy_input_from_py(obj: &Bound<'_, PyAny>) -> PyResult<StrategyInput> {
	let v: Value = crate::convert::any_to_json(obj)?;
	let m = v.as_object().ok_or_else(|| err("input must be a dict"))?;

	let get = |key: &str| -> PyResult<Option<Vec<f64>>> {
		match m.get(key) {
			None | Some(Value::Null) => Ok(None),
			Some(Value::Array(items)) => items
				.iter()
				.map(|i| {
					i.as_f64()
						.ok_or_else(|| err(format!("{key} must be an array of numbers")))
				})
				.collect::<PyResult<Vec<f64>>>()
				.map(Some),
			Some(_) => Err(err(format!("{key} must be an array of numbers"))),
		}
	};

	let closes = m
		.get("closes")
		.and_then(|v| v.as_array())
		.ok_or_else(|| err("input.closes is required"))?
		.iter()
		.map(|i| {
			i.as_f64()
				.ok_or_else(|| err("closes must be an array of numbers"))
		})
		.collect::<PyResult<Vec<f64>>>()?;

	Ok(StrategyInput {
		opens: get("opens")?,
		highs: get("highs")?,
		lows: get("lows")?,
		closes,
		volumes: get("volumes")?,
		timestamps: get("timestamps")?,
	})
}

/// Run a `#[strategy]`-registered strategy by id with a unified OHLCV input.
///
/// `input` is a dict with a required `closes` list plus optional `opens`,
/// `highs`, `lows`, `volumes`, `timestamps`. `config` is an optional dict whose
/// snake_case keys are normalized to the serde camelCase field names.
#[pyfunction]
#[pyo3(signature = (id, input, config = None))]
pub fn run_strategy(
	py: Python<'_>,
	id: String,
	input: &Bound<'_, PyAny>,
	config: Option<Json>,
) -> PyResultO {
	let input = strategy_input_from_py(input)?;
	let cfg = config.map(|c| normalize_config(c.0));
	match get_strategy_registry_impl().get(&id) {
		Some(handler) => {
			let out = handler(&input, cfg).map_err(|e| err(e.to_string()))?;
			Ok(i8_out(py, &out))
		}
		None => Err(err(format!("Unknown strategy: {id}"))),
	}
}

#[pyfunction]
pub fn get_strategy_registry(py: Python<'_>) -> PyResultO {
	let reg = strategies_core::registry::get_strategy_registry();
	let mut strategies = Map::new();
	for (id, def) in reg.strategies {
		let value = serde_json::to_value(def).map_err(|e| err(e.to_string()))?;
		strategies.insert(id, value);
	}
	let mut outer = Map::new();
	outer.insert("strategies".to_string(), Value::Object(strategies));
	json_to_py(py, &Value::Object(outer))
}

#[pyfunction]
pub fn get_strategy_by_id(py: Python<'_>, id: String) -> PyResultO {
	let reg = strategies_core::registry::get_strategy_registry();
	match reg.strategies.get(&id) {
		Some(def) => json_to_py(
			py,
			&serde_json::to_value(def).map_err(|e| err(e.to_string()))?,
		),
		None => Ok(py.None()),
	}
}

#[pyfunction]
pub fn get_strategies_by_category(py: Python<'_>, category: String) -> PyResultO {
	let reg = strategies_core::registry::get_strategy_registry();
	let mut defs = Vec::new();
	for def in reg.strategies.values() {
		if def.category == category {
			defs.push(serde_json::to_value(def).map_err(|e| err(e.to_string()))?);
		}
	}
	json_to_py(py, &Value::Array(defs))
}

#[pyfunction]
pub fn get_all_categories(py: Python<'_>) -> PyResultO {
	let reg = strategies_core::registry::get_strategy_registry();
	let mut categories: Vec<String> = reg
		.strategies
		.values()
		.map(|s| s.category.clone())
		.collect();
	categories.sort();
	categories.dedup();
	json_to_py(
		py,
		&Value::Array(categories.into_iter().map(Value::String).collect()),
	)
}

/// Defaults for every `#[strategy]`-registered strategy, keyed by id.
#[pyfunction]
pub fn get_strategy_defaults(py: Python<'_>) -> PyResultO {
	let mut defaults = Map::new();
	for desc in get_strategy_descriptors() {
		defaults.insert(desc.id.to_string(), (desc.defaults_fn)());
	}
	json_to_py(py, &Value::Object(defaults))
}

// ── Factor registry ───────────────────────────────────────────

#[pyfunction]
pub fn get_factor_registry(py: Python<'_>) -> PyResultO {
	let mut factors = Map::new();
	for d in factors_core::registry::get_factor_descriptors() {
		factors.insert(
			d.id.to_string(),
			json!({
				"id": d.id,
				"name": d.name,
				"category": d.category,
				"description": d.description,
				"needs_prices": d.needs_prices,
			}),
		);
	}
	let mut outer = Map::new();
	outer.insert("factors".to_string(), Value::Object(factors));
	json_to_py(py, &Value::Object(outer))
}

#[pyfunction]
pub fn get_factor_by_id(py: Python<'_>, id: String) -> PyResultO {
	for d in factors_core::registry::get_factor_descriptors() {
		if d.id == id {
			return json_to_py(
				py,
				&json!({
					"id": d.id,
					"name": d.name,
					"category": d.category,
					"description": d.description,
					"needs_prices": d.needs_prices,
				}),
			);
		}
	}
	Ok(py.None())
}

#[pyfunction]
pub fn get_factors_by_category(py: Python<'_>, category: String) -> PyResultO {
	let mut defs = Vec::new();
	for d in factors_core::registry::get_factor_descriptors() {
		if d.category == category {
			defs.push(json!({
				"id": d.id,
				"name": d.name,
				"category": d.category,
				"description": d.description,
				"needs_prices": d.needs_prices,
			}));
		}
	}
	json_to_py(py, &Value::Array(defs))
}

#[pyfunction]
pub fn get_all_factor_categories(py: Python<'_>) -> PyResultO {
	let mut categories: Vec<String> = factors_core::registry::get_factor_descriptors()
		.iter()
		.map(|d| d.category.to_string())
		.collect();
	categories.sort();
	categories.dedup();
	json_to_py(
		py,
		&Value::Array(categories.into_iter().map(Value::String).collect()),
	)
}

// ── Indicator registry ────────────────────────────────────────

#[pyfunction]
pub fn get_indicator_registry(py: Python<'_>) -> PyResultO {
	let mut indicators = Map::new();
	for d in indicators_core::registry::get_indicator_descriptors() {
		indicators.insert(
			d.id.to_string(),
			json!({
				"id": d.id,
				"name": d.name,
				"category": d.category,
				"description": d.description,
			}),
		);
	}
	let mut outer = Map::new();
	outer.insert("indicators".to_string(), Value::Object(indicators));
	json_to_py(py, &Value::Object(outer))
}

#[pyfunction]
pub fn get_indicators_by_category(py: Python<'_>, category: String) -> PyResultO {
	let mut defs = Vec::new();
	for d in indicators_core::registry::get_indicator_descriptors() {
		if d.category == category {
			defs.push(json!({
				"id": d.id,
				"name": d.name,
				"category": d.category,
				"description": d.description,
			}));
		}
	}
	json_to_py(py, &Value::Array(defs))
}

#[pyfunction]
pub fn get_indicator_by_id(py: Python<'_>, id: String) -> PyResultO {
	for d in indicators_core::registry::get_indicator_descriptors() {
		if d.id == id {
			return json_to_py(
				py,
				&json!({
					"id": d.id,
					"name": d.name,
					"category": d.category,
					"description": d.description,
				}),
			);
		}
	}
	Ok(py.None())
}

#[pyfunction]
pub fn get_all_indicator_categories(py: Python<'_>) -> PyResultO {
	let mut categories: Vec<String> = indicators_core::registry::get_indicator_descriptors()
		.iter()
		.map(|d| d.category.to_string())
		.collect();
	categories.sort();
	categories.dedup();
	json_to_py(
		py,
		&Value::Array(categories.into_iter().map(Value::String).collect()),
	)
}
