use indicators_core::{
	fi as fi_alias, force_index as fi_core, mfi as mfi_core, money_flow_index as mfi_alias,
	volume::accumulation_distribution::{accumulation_distribution as ad_core, ad as ad_alias},
	volume::anchored_vwap::anchored_vwap as avwap_core,
	volume::chaikin_money_flow::{chaikin_money_flow as cmf_core, cmf as cmf_alias},
	volume::ease_of_movement::{ease_of_movement as eom_core, emv as emv_core},
	volume::negative_volume_index::{negative_volume_index as nvi_core, nvi as nvi_alias},
	volume::obv::{obv as obv_core, on_balance_volume as obv_alias},
	volume::volume_price_trend::{volume_price_trend as vpt_core, vpt as vpt_alias},
	volume_profile as vp_core, volume_surge as vs_core,
	volume_weighted_average_price as vwap_alias, vs as vs_alias, vwap as vwap_core, FIConfig,
	MFIConfig, VWAPConfig, VolumeProfileResult, VolumeSurgeConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Accumulation Distribution
#[napi]
pub fn accumulation_distribution(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
) -> Vec<f64> {
	ad_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
	)
}

/// Ad
#[napi]
pub fn ad(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
) -> Vec<f64> {
	ad_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
	)
}

/// Anchored Vwap
#[napi]
pub fn anchored_vwap(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	anchor_index: Option<u32>,
) -> Vec<f64> {
	let anchor = anchor_index.unwrap_or(0);
	avwap_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		anchor,
	)
}

/// Chaikin Money Flow
#[napi]
pub fn chaikin_money_flow(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(20);
	cmf_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		period,
	)
}

/// Cmf
#[napi]
pub fn cmf(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(20);
	cmf_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		period,
	)
}

/// Ease Of Movement
#[napi]
pub fn ease_of_movement(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(14);
	eom_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), period)
}

/// Emv
#[napi]
pub fn emv(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(14);
	emv_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), period)
}

/// Force Index
#[napi]
pub fn force_index(
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<FIConfig>,
) -> Vec<f64> {
	fi_core(closings.as_ref(), volumes.as_ref(), config)
}

/// Fi
#[napi]
pub fn fi(closings: Float64Array, volumes: Float64Array, config: Option<FIConfig>) -> Vec<f64> {
	fi_alias(closings.as_ref(), volumes.as_ref(), config)
}

/// Mfi
#[napi]
pub fn mfi(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<MFIConfig>,
) -> Vec<f64> {
	mfi_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Money Flow Index
#[napi]
pub fn money_flow_index(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<MFIConfig>,
) -> Vec<f64> {
	mfi_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Negative Volume Index
#[napi]
pub fn negative_volume_index(
	closings: Float64Array,
	volumes: Float64Array,
	start: Option<f64>,
) -> Vec<f64> {
	nvi_core(closings.as_ref(), volumes.as_ref(), start)
}

/// Nvi
#[napi]
pub fn nvi(closings: Float64Array, volumes: Float64Array, start: Option<f64>) -> Vec<f64> {
	nvi_alias(closings.as_ref(), volumes.as_ref(), start)
}

/// Obv
#[napi]
pub fn obv(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	obv_core(closings.as_ref(), volumes.as_ref())
}

/// On Balance Volume
#[napi]
pub fn on_balance_volume(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	obv_alias(closings.as_ref(), volumes.as_ref())
}

/// Volume Price Trend
#[napi]
pub fn volume_price_trend(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	vpt_core(closings.as_ref(), volumes.as_ref())
}

/// Vpt
#[napi]
pub fn vpt(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	vpt_alias(closings.as_ref(), volumes.as_ref())
}

/// Volume Profile
#[napi]
pub fn volume_profile(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	bins: Option<u32>,
) -> VolumeProfileResult {
	vp_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), bins)
}

/// Volume Surge
#[napi]
pub fn volume_surge(volumes: Float64Array, config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	vs_core(volumes.as_ref(), config)
}

/// Vs
#[napi]
pub fn vs(volumes: Float64Array, config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	vs_alias(volumes.as_ref(), config)
}

/// Vwap
#[napi]
pub fn vwap(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	vwap_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Volume Weighted Average Price
#[napi]
pub fn volume_weighted_average_price(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	vwap_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}
