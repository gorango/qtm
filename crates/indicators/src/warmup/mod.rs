use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct IndicatorWarmupResult {
	pub warmup_period: u32,
}

pub fn calculate_rsi_warmup(period: u32) -> u32 {
	period * 3
}

pub fn calculate_ema_warmup(period: u32) -> u32 {
	period * 2
}

pub fn calculate_macd_warmup(fast_period: u32, slow_period: u32, signal_period: u32) -> u32 {
	let slow_ema_warmup = calculate_ema_warmup(slow_period);
	let signal_ema_warmup = calculate_ema_warmup(signal_period);
	slow_ema_warmup.max(fast_period + signal_ema_warmup)
}

pub fn calculate_stochastic_warmup(k_period: u32, d_period: u32) -> u32 {
	k_period * 2 + d_period
}

pub fn calculate_williams_rwarmup(period: u32) -> u32 {
	period * 2 + 1
}

pub fn calculate_ichimoku_warmup(short: u32, medium: u32, long: u32) -> u32 {
	let tenkan_warmup = calculate_ema_warmup(short);
	let kijun_warmup = calculate_ema_warmup(medium);
	let ssb_warmup = calculate_ema_warmup(long);
	tenkan_warmup.max(kijun_warmup).max(ssb_warmup)
}

pub fn calculate_adx_warmup(period: u32) -> u32 {
	period * 3
}

pub fn calculate_supertrend_warmup(period: u32) -> u32 {
	calculate_atr_warmup(period)
}

pub fn calculate_chaikin_oscillator_warmup(fast_period: u32, slow_period: u32) -> u32 {
	calculate_macd_warmup(fast_period, slow_period, 1) + 5
}

pub fn calculate_vwap_warmup(period: u32) -> u32 {
	(period as f64 * 1.5).ceil() as u32
}

pub fn calculate_obv_warmup(period: u32) -> u32 {
	(period as f64 * 1.5).ceil() as u32
}

pub fn calculate_accumulation_distribution_warmup() -> u32 {
	15
}

#[allow(clippy::too_many_arguments)]
pub fn calculate_kst_warmup(
	roc1: u32,
	roc2: u32,
	roc3: u32,
	roc4: u32,
	sma1: u32,
	sma2: u32,
	sma3: u32,
	sma4: u32,
	signal_period: u32,
) -> u32 {
	let max_roc = roc1.max(roc2).max(roc3).max(roc4);
	let max_sma = sma1.max(sma2).max(sma3).max(sma4);
	(max_roc + max_sma).max(signal_period)
}

pub fn calculate_mfi_warmup(period: u32) -> u32 {
	(period as f64 * 1.5).ceil() as u32
}

pub fn calculate_keltner_channel_warmup(period: u32) -> u32 {
	calculate_atr_warmup(period)
}

pub fn calculate_projection_oscillator_warmup(period: u32, smooth: u32) -> u32 {
	((period + smooth) as f64 * 1.5).ceil() as u32
}

pub fn calculate_chandelier_exit_warmup(period: u32) -> u32 {
	calculate_atr_warmup(period)
}

pub fn calculate_parabolic_sar_warmup() -> u32 {
	50
}

pub fn calculate_bollinger_bands_warmup(period: u32) -> u32 {
	(period as f64 * 1.5).ceil() as u32
}

pub fn calculate_atr_warmup(period: u32) -> u32 {
	(period as f64 * 2.5).ceil() as u32
}

pub fn calculate_alma_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_hma_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_wma_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_linreg_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_mad_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_variance_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_correlation_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_percent_rank_warmup(period: u32) -> u32 {
	period
}

pub fn calculate_indicator_warmup(
	indicator_type: String,
	params: serde_json::Value,
) -> Result<IndicatorWarmupResult, String> {
	let period = params.get("period").and_then(|v| v.as_u64()).unwrap_or(14) as u32;

	let warmup = match indicator_type.to_lowercase().as_str() {
		"rsi" => calculate_rsi_warmup(period),
		"ema" => calculate_ema_warmup(period),
		"sma" | "stddev" => period,
		"macd" => {
			let fast = params
				.get("fastPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(12) as u32;
			let slow = params
				.get("slowPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(26) as u32;
			let signal = params
				.get("signalPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(9) as u32;
			calculate_macd_warmup(fast, slow, signal)
		}
		"stochastic" => {
			let k_period = params.get("kPeriod").and_then(|v| v.as_u64()).unwrap_or(14) as u32;
			let d_period = params.get("dPeriod").and_then(|v| v.as_u64()).unwrap_or(3) as u32;
			calculate_stochastic_warmup(k_period, d_period)
		}
		"williams" | "williamsr" | "williams-r" => calculate_williams_rwarmup(period),
		"ichimoku" => {
			let short = params.get("short").and_then(|v| v.as_u64()).unwrap_or(9) as u32;
			let medium = params.get("medium").and_then(|v| v.as_u64()).unwrap_or(26) as u32;
			let long = params.get("long").and_then(|v| v.as_u64()).unwrap_or(52) as u32;
			calculate_ichimoku_warmup(short, medium, long)
		}
		"adx" => calculate_adx_warmup(period),
		"supertrend" => calculate_supertrend_warmup(period),
		"chaikin" => {
			let fast_period = params
				.get("fastPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(3) as u32;
			let slow_period = params
				.get("slowPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(10) as u32;
			calculate_chaikin_oscillator_warmup(fast_period, slow_period)
		}
		"vwap" => calculate_vwap_warmup(period),
		"obv" => calculate_obv_warmup(period),
		"accumulation" | "ad" => calculate_accumulation_distribution_warmup(),
		"kst" => {
			let roc1 = params.get("roc1").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
			let roc2 = params.get("roc2").and_then(|v| v.as_u64()).unwrap_or(15) as u32;
			let roc3 = params.get("roc3").and_then(|v| v.as_u64()).unwrap_or(20) as u32;
			let roc4 = params.get("roc4").and_then(|v| v.as_u64()).unwrap_or(30) as u32;
			let sma1 = params.get("sma1").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
			let sma2 = params.get("sma2").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
			let sma3 = params.get("sma3").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
			let sma4 = params.get("sma4").and_then(|v| v.as_u64()).unwrap_or(15) as u32;
			let signal_period = params
				.get("signalPeriod")
				.and_then(|v| v.as_u64())
				.unwrap_or(9) as u32;
			calculate_kst_warmup(
				roc1,
				roc2,
				roc3,
				roc4,
				sma1,
				sma2,
				sma3,
				sma4,
				signal_period,
			)
		}
		"mfi" => calculate_mfi_warmup(period),
		"keltner" => calculate_keltner_channel_warmup(period),
		"projection" => {
			let smooth = params.get("smooth").and_then(|v| v.as_u64()).unwrap_or(3) as u32;
			calculate_projection_oscillator_warmup(period, smooth)
		}
		"chandelier" => calculate_chandelier_exit_warmup(period),
		"parabolic" | "psar" => calculate_parabolic_sar_warmup(),
		"bollinger" | "bb" => calculate_bollinger_bands_warmup(period),
		"atr" => calculate_atr_warmup(period),
		"alma" => calculate_alma_warmup(period),
		"hma" => calculate_hma_warmup(period),
		"wma" => calculate_wma_warmup(period),
		"linreg" => calculate_linreg_warmup(period),
		"mad" | "meanabsolutedeviation" => calculate_mad_warmup(period),
		"variance" => calculate_variance_warmup(period),
		"correlation" => calculate_correlation_warmup(period),
		"percentrank" => calculate_percent_rank_warmup(period),
		_ => period * 2,
	};

	Ok(IndicatorWarmupResult {
		warmup_period: warmup,
	})
}
