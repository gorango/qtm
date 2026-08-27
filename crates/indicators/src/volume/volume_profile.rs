use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
/// Volume Profile result — price bins with volume distribution.
pub struct VolumeProfileResult {
	pub price_levels: Vec<f64>,
	pub volumes: Vec<f64>,
	pub point_of_control: f64,
	pub high_volume_node: f64,
	pub low_volume_node: f64,
}

/// Volume Profile — histogram of volume by price level over the lookback.
/// Bins closes by price and sums volume per bin. `NaN` for empty bins.
pub fn volume_profile(
	highs: &[f64],
	lows: &[f64],
	volumes: &[f64],
	bins: Option<u32>,
) -> VolumeProfileResult {
	let len = highs.len();
	let bins = bins.unwrap_or(50) as usize;

	if len == 0 {
		return VolumeProfileResult {
			price_levels: vec![],
			volumes: vec![],
			point_of_control: 0.0,
			high_volume_node: 0.0,
			low_volume_node: 0.0,
		};
	}

	let min_price = lows.iter().cloned().fold(f64::INFINITY, f64::min);
	let max_price = highs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
	let price_range = max_price - min_price;

	if price_range == 0.0 {
		let total_volume: f64 = volumes.iter().sum();
		return VolumeProfileResult {
			price_levels: vec![min_price],
			volumes: vec![total_volume],
			point_of_control: min_price,
			high_volume_node: min_price,
			low_volume_node: min_price,
		};
	}

	let bin_size = price_range / bins as f64;
	let mut volume_bins: Vec<f64> = vec![0.0; bins];
	let mut price_levels: Vec<f64> = Vec::with_capacity(bins);

	for i in 0..bins {
		price_levels.push(min_price + i as f64 * bin_size);
	}

	for i in 0..len {
		let h = highs[i];
		let l = lows[i];
		let v = volumes[i];

		let start_bin = ((l - min_price) / bin_size).floor() as usize;
		let end_bin = ((h - min_price) / bin_size).floor() as usize;

		let bins_covered = std::cmp::max(1, end_bin - start_bin + 1);
		let volume_per_bin = v / bins_covered as f64;

		let start = std::cmp::max(0, start_bin);
		let end = std::cmp::min(bins - 1, end_bin);

		volume_bins[start..=end]
			.iter_mut()
			.for_each(|v| *v += volume_per_bin);
	}

	let mut max_volume = 0.0;
	let mut poc_index = 0;
	let mut hvn_index = 0;
	let mut lvn_index = 0;

	for i in 0..volume_bins.len() {
		if volume_bins[i] > max_volume {
			max_volume = volume_bins[i];
			poc_index = i;
		}
		if volume_bins[i] > volume_bins[hvn_index] {
			hvn_index = i;
		}
		if volume_bins[i] < volume_bins[lvn_index] {
			lvn_index = i;
		}
	}

	VolumeProfileResult {
		price_levels: price_levels.clone(),
		volumes: volume_bins,
		point_of_control: price_levels[poc_index],
		high_volume_node: price_levels[hvn_index],
		low_volume_node: price_levels[lvn_index],
	}
}
