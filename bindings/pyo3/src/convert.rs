use numpy::{AllowTypeChange, PyArray1, PyArray2, PyArrayLike1, PyArrayLike2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyBool, PyDict, PyList, PyTuple};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

/// pyo3 0.29 no longer ships a `PyObject` alias; this is the canonical one.
pub type PyObject = pyo3::Py<pyo3::PyAny>;

/// Newtype that implements `FromPyObject` for `serde_json::Value`.
///
/// pyo3's `serde` feature only (de)serializes `Py<T>` objects; it does not
/// extract `serde_json::Value` directly. This bridges that gap so pyfunctions
/// can declare `Option<Json>` (configs) and `Vec<Json>` (record lists).
#[derive(Debug, Clone)]
pub struct Json(pub Value);

impl<'a, 'py> FromPyObject<'a, 'py> for Json {
	type Error = PyErr;

	fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
		any_to_json(&obj).map(Json)
	}
}

/// Recursively convert any JSON-like Python object (dict, list, tuple, str,
/// int, float, bool, None, or any iterable) into `serde_json::Value`.
pub fn any_to_json(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
	if obj.is_none() {
		return Ok(Value::Null);
	}
	if let Ok(s) = obj.extract::<String>() {
		return Ok(Value::String(s));
	}
	if let Ok(b) = obj.extract::<bool>() {
		return Ok(Value::Bool(b));
	}
	if let Ok(i) = obj.extract::<i64>() {
		return Ok(Value::from(i));
	}
	if let Ok(u) = obj.extract::<u64>() {
		return Ok(Value::from(u));
	}
	if let Ok(f) = obj.extract::<f64>() {
		return Ok(Value::from(f));
	}
	if let Ok(d) = obj.cast::<PyDict>() {
		let mut map = serde_json::Map::new();
		for (k, v) in d.iter() {
			let key = k
				.extract::<String>()
				.map_err(|_| err("dict keys must be strings"))?;
			map.insert(key, any_to_json(&v)?);
		}
		return Ok(Value::Object(map));
	}
	if let Ok(l) = obj.cast::<PyList>() {
		let mut items = Vec::with_capacity(l.len());
		for item in l.iter() {
			items.push(any_to_json(&item)?);
		}
		return Ok(Value::Array(items));
	}
	if let Ok(t) = obj.cast::<PyTuple>() {
		let mut items = Vec::with_capacity(t.len());
		for item in t.iter() {
			items.push(any_to_json(&item)?);
		}
		return Ok(Value::Array(items));
	}
	// Fallback for other sequences (e.g. numpy arrays).
	if let Ok(iter) = obj.try_iter() {
		let mut items = Vec::new();
		for item in iter {
			items.push(any_to_json(&item?)?);
		}
		return Ok(Value::Array(items));
	}
	Err(err("unsupported Python object; expected a JSON-compatible value"))
}

/// Common array-input type: accepts a numpy float64 array (or anything numpy
/// can cast to float64, e.g. int arrays) as well as plain Python sequences.
pub type F64Arr1<'py> = PyArrayLike1<'py, f64, AllowTypeChange>;
pub type F64Arr2<'py> = PyArrayLike2<'py, f64, AllowTypeChange>;

/// Build a ValueError with the given message.
pub fn err(msg: impl std::fmt::Display) -> PyErr {
	PyValueError::new_err(msg.to_string())
}

/// Deserialize a serde_json::Value into `T`, with a contextual error.
pub fn from_value<T: DeserializeOwned>(value: Value, what: &str) -> PyResult<T> {
	serde_json::from_value(value).map_err(|e| err(format!("invalid {what}: {e}")))
}

/// snake_case -> camelCase, mirroring `#[serde(rename_all = "camelCase")]` on the
/// config structs. Keys with no underscores (already camelCase) pass through.
fn snake_to_camel(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	let mut cap = false;
	for c in s.chars() {
		if c == '_' {
			cap = true;
		} else if cap {
			out.extend(c.to_uppercase());
			cap = false;
		} else {
			out.push(c);
		}
	}
	out
}

/// Recursively rewrite dict keys to the camelCase serde field names used by the
/// config structs. Applied only to strategy/indicator config objects, never to
/// data records (which serialize snake_case).
pub fn normalize_config(value: Value) -> Value {
	match value {
		Value::Object(map) => Value::Object(
			map.into_iter()
				.map(|(k, v)| {
					let key = if k.contains('_') { snake_to_camel(&k) } else { k };
					(key, normalize_config(v))
				})
				.collect(),
		),
		Value::Array(items) => Value::Array(items.into_iter().map(normalize_config).collect()),
		other => other,
	}
}

/// Deserialize a normalized config into its concrete struct type.
pub fn deserialize_cfg<T: DeserializeOwned>(config: Option<Value>) -> PyResult<Option<T>> {
	match config {
		None => Ok(None),
		Some(v) => Ok(Some(from_value(v, "config")?)),
	}
}

/// Extract a list of record dicts (Bar, FundamentalPoint, ...) into `T`.
/// Record keys are snake_case, matching serde directly (no renaming).
pub fn records<T: DeserializeOwned>(items: Vec<Json>, what: &str) -> PyResult<Vec<T>> {
	items.into_iter().map(|j| from_value(j.0, what)).collect()
}

/// Extract a 2D array to Vec<Vec<f64>>.
pub fn f64_matrix(arr: &F64Arr2<'_>, _what: &str) -> PyResult<Vec<Vec<f64>>> {
	Ok(arr
		.as_array()
		.outer_iter()
		.map(|r| r.to_vec())
		.collect())
}

/// Read the `period` field out of a normalized config, with a default.
pub fn cfg_u32(config: &Option<Value>, key: &str, default: u32) -> u32 {
	config
		.as_ref()
		.and_then(|c| c.get(key))
		.and_then(Value::as_u64)
		.map(|v| v as u32)
		.unwrap_or(default)
}

// ── Output helpers: numpy arrays ─────────────────────────────

pub fn f64_out(py: Python<'_>, v: &[f64]) -> PyObject {
	PyArray1::from_slice(py, v).into_any().unbind()
}

pub fn i8_out(py: Python<'_>, v: &[i8]) -> PyObject {
	PyArray1::from_slice(py, v).into_any().unbind()
}

pub fn bool_out(py: Python<'_>, v: &[bool]) -> PyObject {
	PyArray1::from_slice(py, v).into_any().unbind()
}

pub fn u32_out(py: Python<'_>, v: &[u32]) -> PyObject {
	PyArray1::from_slice(py, v).into_any().unbind()
}

/// Convert a serde_json::Value into a Python object. Numeric/bool/array JSON
/// values become numpy arrays; objects become dicts.
pub fn json_to_py(py: Python<'_>, v: &Value) -> PyResult<PyObject> {
	Ok(match v {
		Value::Null => py.None(),
		Value::Bool(b) => PyBool::new(py, *b).to_owned().into_any().unbind(),
		Value::Number(n) => {
			if let Some(i) = n.as_i64() {
				i.into_pyobject(py)?.into_any().unbind()
			} else if let Some(u) = n.as_u64() {
				u.into_pyobject(py)?.into_any().unbind()
			} else {
				n.as_f64()
					.unwrap_or(f64::NAN)
					.into_pyobject(py)?
					.into_any()
					.unbind()
			}
		}
		Value::String(s) => s.into_pyobject(py)?.into_any().unbind(),
		Value::Array(items) => {
			if items
				.iter()
				.all(|i| i.is_number() || i.is_null())
			{
				let arr: Vec<f64> = items
					.iter()
					.map(|i| i.as_f64().unwrap_or(f64::NAN))
					.collect();
				PyArray1::from_vec(py, arr).into_any().unbind()
			} else if items.iter().all(|i| i.is_boolean() || i.is_null()) {
				let arr: Vec<Option<bool>> = items.iter().map(|i| i.as_bool()).collect();
				if arr.iter().all(|b| b.is_some()) {
					let v: Vec<bool> = arr.into_iter().flatten().collect();
					PyArray1::from_vec(py, v).into_any().unbind()
				} else {
					let list = PyList::empty(py);
					for item in items {
						list.append(json_to_py(py, item)?)?;
					}
					list.into_any().unbind()
				}
			} else if items.iter().all(|i| i.is_array()) {
				let rows: Vec<Vec<f64>> = items
					.iter()
					.map(|row| {
						row.as_array()
							.unwrap()
							.iter()
							.filter_map(|i| i.as_f64())
							.collect()
					})
					.collect();
				PyArray2::from_vec2(py, &rows)
					.map_err(|e| err(e.to_string()))?
					.into_any()
					.unbind()
			} else {
				let list = PyList::empty(py);
				for item in items {
					list.append(json_to_py(py, item)?)?;
				}
				list.into_any().unbind()
			}
		}
		Value::Object(map) => {
			let d = PyDict::new(py);
			for (k, v) in map {
				d.set_item(k, json_to_py(py, v)?)?;
			}
			d.into_any().unbind()
		}
	})
}

/// Serialize any `Serialize` value to Python (via serde_json), mapping JSON
/// arrays to numpy arrays.
pub fn to_py<T: Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
	let json = serde_json::to_value(value).map_err(|e| err(e.to_string()))?;
	json_to_py(py, &json)
}
