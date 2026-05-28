use crate::types::data::{FactorPoint, OnChainDataPoint};

/// Active Address Growth: `(current - previous) / previous` over `period` days.
/// Measures blockchain network adoption growth.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn active_address_growth(
	on_chain_data: Vec<OnChainDataPoint>,
	period: Option<f64>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(30.0);
	let mut results = Vec::new();

	let mut addrs: Vec<&OnChainDataPoint> = on_chain_data
		.iter()
		.filter(|d| d.metric == "activeAddresses")
		.collect();
	addrs.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

	if addrs.len() < 2 {
		return results;
	}

	let ms_in_day = 24.0 * 60.0 * 60.0 * 1000.0;
	for i in 1..addrs.len() {
		let cur = addrs[i];
		let mut prev_idx = None;
		for j in (0..i).rev() {
			let diff = (cur.time - addrs[j].time) / ms_in_day;
			if diff <= p {
				prev_idx = Some(j);
				break;
			}
		}
		match prev_idx {
			Some(j) if addrs[j].value > 0.0 => {
				let growth = (cur.value - addrs[j].value) / addrs[j].value;
				results.push(FactorPoint {
					date: cur.time,
					value: growth,
				});
			}
			_ => continue,
		}
	}
	results
}

/// Exchange Flow Momentum: `(currentFlow - previousFlow) / |previousFlow|` over `period` days.
/// Positive = inflow (accumulation), negative = outflow (distribution).
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn exchange_flow_momentum(
	on_chain_data: Vec<OnChainDataPoint>,
	period: Option<f64>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(30.0);
	let mut results = Vec::new();

	let mut flows: Vec<&OnChainDataPoint> = on_chain_data
		.iter()
		.filter(|d| d.metric == "exchangeNetflow")
		.collect();
	flows.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

	if flows.len() < 2 {
		return results;
	}

	let ms_in_day = 24.0 * 60.0 * 60.0 * 1000.0;
	for i in 1..flows.len() {
		let cur = flows[i];
		let mut prev_idx = None;
		for j in (0..i).rev() {
			let diff = (cur.time - flows[j].time) / ms_in_day;
			if diff <= p {
				prev_idx = Some(j);
				break;
			}
		}
		match prev_idx {
			Some(j) if flows[j].value != 0.0 => {
				let momentum = (cur.value - flows[j].value) / flows[j].value.abs();
				results.push(FactorPoint {
					date: cur.time,
					value: momentum,
				});
			}
			_ => continue,
		}
	}
	results
}

/// Network Value to Transactions (NVT) Ratio: `marketCap / transactionVolume`.
/// Lower values may indicate undervaluation relative to economic activity.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn nvt_ratio(on_chain_data: Vec<OnChainDataPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut mcap_map: std::collections::HashMap<i64, f64> = std::collections::HashMap::new();
	let mut vol_map: std::collections::HashMap<i64, f64> = std::collections::HashMap::new();

	for d in &on_chain_data {
		let ts = d.time as i64;
		match d.metric.as_str() {
			"marketCap" => {
				mcap_map.insert(ts, d.value);
			}
			"transactionVolume" => {
				vol_map.insert(ts, d.value);
			}
			_ => {}
		}
	}

	let mut common: Vec<i64> = mcap_map
		.keys()
		.filter(|k| vol_map.contains_key(k))
		.copied()
		.collect();
	common.sort();

	for ts in &common {
		let mcap = mcap_map[ts];
		let vol = vol_map[ts];
		if vol <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			date: *ts as f64,
			value: mcap / vol,
		});
	}
	results
}

/// Staking Ratio: `stakedSupply / totalSupply`.
/// Measures network security and participant commitment.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn staking_ratio(on_chain_data: Vec<OnChainDataPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut staked_map: std::collections::HashMap<i64, f64> = std::collections::HashMap::new();
	let mut total_map: std::collections::HashMap<i64, f64> = std::collections::HashMap::new();

	for d in &on_chain_data {
		let ts = d.time as i64;
		match d.metric.as_str() {
			"stakedSupply" => {
				staked_map.insert(ts, d.value);
			}
			"totalSupply" => {
				total_map.insert(ts, d.value);
			}
			_ => {}
		}
	}

	let mut common: Vec<i64> = staked_map
		.keys()
		.filter(|k| total_map.contains_key(k))
		.copied()
		.collect();
	common.sort();

	for ts in &common {
		let staked = staked_map[ts];
		let total = total_map[ts];
		if total <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			date: *ts as f64,
			value: staked / total,
		});
	}
	results
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::data::*;

	fn make_ocp(time: f64, metric: &str, value: f64) -> OnChainDataPoint {
		OnChainDataPoint {
			time,
			metric: metric.to_string(),
			value,
		}
	}

	fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
		assert!((a - b).abs() < epsilon, "expected {} ≈ {}", a, b);
	}

	#[test]
	fn test_nvt_ratio_happy() {
		let data = vec![
			make_ocp(100.0, "marketCap", 1_000_000_000.0),
			make_ocp(100.0, "transactionVolume", 500_000_000.0),
		];
		let result = nvt_ratio(data);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 2.0, 1e-6);
	}

	#[test]
	fn test_nvt_ratio_skips_zero_volume() {
		let data = vec![
			make_ocp(100.0, "marketCap", 1_000_000_000.0),
			make_ocp(100.0, "transactionVolume", 0.0),
		];
		let result = nvt_ratio(data);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_nvt_ratio_skips_when_metric_missing() {
		let data = vec![make_ocp(100.0, "marketCap", 1_000_000_000.0)];
		assert_eq!(nvt_ratio(data).len(), 0);
	}

	#[test]
	fn test_active_address_growth_happy() {
		let data = vec![
			make_ocp(100.0, "activeAddresses", 1000.0),
			make_ocp(200.0, "activeAddresses", 1200.0),
		];
		let result = active_address_growth(data, None);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.2, 1e-6);
	}

	#[test]
	fn test_active_address_growth_needs_two_points() {
		let data = vec![make_ocp(100.0, "activeAddresses", 1000.0)];
		assert_eq!(active_address_growth(data, None).len(), 0);
	}

	#[test]
	fn test_active_address_growth_uses_period() {
		let ms_day = 24.0 * 60.0 * 60.0 * 1000.0;
		let data = vec![
			make_ocp(100.0 * ms_day, "activeAddresses", 1000.0),
			make_ocp(150.0 * ms_day, "activeAddresses", 1100.0),
			make_ocp(2000.0 * ms_day, "activeAddresses", 2000.0),
		];
		let result = active_address_growth(data, Some(60.0));
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.1, 1e-6);
	}

	#[test]
	fn test_staking_ratio_happy() {
		let data = vec![
			make_ocp(100.0, "stakedSupply", 40_000_000.0),
			make_ocp(100.0, "totalSupply", 100_000_000.0),
		];
		let result = staking_ratio(data);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.4, 1e-6);
	}

	#[test]
	fn test_staking_ratio_skips_zero_total_supply() {
		let data = vec![
			make_ocp(100.0, "stakedSupply", 50.0),
			make_ocp(100.0, "totalSupply", 0.0),
		];
		assert_eq!(staking_ratio(data).len(), 0);
	}

	#[test]
	fn test_exchange_flow_momentum_happy() {
		let data = vec![
			make_ocp(100.0, "exchangeNetflow", 500.0),
			make_ocp(200.0, "exchangeNetflow", 800.0),
		];
		let result = exchange_flow_momentum(data, None);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, (800.0 - 500.0) / 500.0, 1e-6);
	}

	#[test]
	fn test_empty_data() {
		assert_eq!(nvt_ratio(vec![]).len(), 0);
		assert_eq!(active_address_growth(vec![], None).len(), 0);
		assert_eq!(staking_ratio(vec![]).len(), 0);
		assert_eq!(exchange_flow_momentum(vec![], None).len(), 0);
	}

	#[test]
	fn test_staking_ratio_skips_when_metric_missing() {
		let data = vec![make_ocp(100.0, "stakedSupply", 50.0)];
		assert_eq!(staking_ratio(data).len(), 0);
	}
}
