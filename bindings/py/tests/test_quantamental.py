import json
import os
from pathlib import Path

import numpy as np
import pytest

import quantamental as q


def test_module_loads_and_exposes_expected_exports():
	assert callable(q.rsi)
	assert callable(q.get_strategy_registry)
	assert callable(q.awesome_oscillator)


def test_indicator_returns_numeric_numpy_array():
	closes = np.array([10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24])
	out = q.rsi(closes)
	assert isinstance(out, np.ndarray)
	assert out.dtype == np.float64
	assert out.shape == closes.shape
	assert np.any(np.isfinite(out))


def test_indicator_accepts_plain_lists():
	out = q.ema([1.0, 2.0, 3.0, 4.0, 5.0])
	assert isinstance(out, np.ndarray)
	assert out.dtype == np.float64


def test_indicator_config_accepts_snake_case_keys():
	closes = np.linspace(10, 40, 300)
	with_defaults = q.rsi(closes)
	with_period = q.rsi(closes, {"period": 7})
	assert isinstance(with_period, np.ndarray)
	assert not np.array_equal(with_defaults, with_period)


def test_multi_array_indicator_validates_lengths():
	with pytest.raises(ValueError):
		q.obv(np.array([1.0, 2.0]), np.array([1.0, 2.0, 3.0]))


def test_result_struct_returns_dict_of_arrays():
	out = q.bollinger_bands(np.linspace(10, 40, 100))
	assert isinstance(out, dict)
	assert set(out.keys()) >= {"upper", "middle", "lower"}
	assert all(isinstance(v, np.ndarray) for v in out.values())


def test_warmup_functions():
	assert q.calculate_rsi_warmup(14) >= 14
	assert q.calculate_macd_warmup(12, 26, 9) > 0


def test_strategy_registry_is_populated():
	registry = q.get_strategy_registry()
	assert isinstance(registry, dict)
	ids = set(registry.get("strategies", {}))
	assert len(ids) > 0
	assert "buy_and_hold" in ids


def test_registry_strategies_are_dispatchable_by_id():
	signals = q.run_strategy("buy_and_hold", {"closes": [100, 101, 102, 103, 104]})
	assert isinstance(signals, np.ndarray)
	assert signals.dtype == np.int8
	assert signals.tolist() == [1, 0, 0, 0, 0]

	with pytest.raises(ValueError, match="Unknown strategy"):
		q.run_strategy("no-such-strategy", {"closes": [100]})


def test_all_registry_strategies_produce_signals():
	closes = list(np.arange(300, dtype=float) * 0.5 + 100)
	registry = q.get_strategy_registry()

	registry_json_path = (
		Path(__file__).parents[3] / "packages" / "tools" / "src" / "generated" / "registry.json"
	)
	registry_json = json.loads(registry_json_path.read_text())

	for sid in registry["strategies"]:
		schema = json.loads(registry_json["strategies"][sid].get("params_schema") or "{}")
		config = {"second_closes": closes} if schema.get("properties", {}).get("secondCloses") else None
		out = q.run_strategy(sid, {"closes": closes}, config)
		assert isinstance(out, np.ndarray)
		assert out.shape == (len(closes),), f"strategy {sid} should produce one signal per bar"


def test_handwritten_strategy_metadata_and_defaults():
	meta = q.strategy_metadata("value_strategy")
	assert meta.get("id") == "value-investing"
	assert meta.get("description")

	defaults = q.strategy_defaults("value_strategy")
	assert defaults.get("params") is not None

	with pytest.raises(ValueError, match="Unknown strategy"):
		q.strategy_metadata("no-such")


def test_handwritten_strategy_execution():
	factors = [
		{"symbol": "A", "date": 1.0, "value": 10.0},
		{"symbol": "A", "date": 2.0, "value": 12.0},
		{"symbol": "A", "date": 3.0, "value": 11.0},
	]
	out = q.value_strategy(factors, {"pe_threshold": 25})
	assert isinstance(out, np.ndarray)
	assert out.dtype == np.int8
	assert out.shape[0] == 3


def test_factor_functions():
	fundamentals = [
		{
			"symbol": "A",
			"date": 1.0,
			"filing_date": 1.0,
			"period": "2024Q1",
			"data": {"revenue": 100.0, "total_assets": 200.0, "net_income": 10.0, "shareholders_equity": 50.0},
		}
	]
	out = q.asset_turnover(fundamentals)
	assert isinstance(out, list)
	assert out[0]["value"] == pytest.approx(0.5)


def test_factor_registry():
	registry = q.get_factor_registry()
	assert isinstance(registry, dict)
	assert len(registry["factors"]) > 0
	assert "asset_turnover" in registry["factors"]


def test_indicator_registry():
	registry = q.get_indicator_registry()
	assert isinstance(registry, dict)
	assert len(registry["indicators"]) > 0
	assert "rsi" in registry["indicators"]
