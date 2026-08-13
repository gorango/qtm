#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Base strategy configuration with flexible parameters
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StrategyConfig {
	/// Strategy-specific parameters as JSON
	pub params: serde_json::Value,
}

/// RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RSIConfig {
	pub period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for RSIConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			oversold: Some(30.0),
			overbought: Some(70.0),
		}
	}
}

/// MACD Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MACDConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
}

impl Default for MACDConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(12),
			slow_period: Some(26),
			signal_period: Some(9),
		}
	}
}

/// MACD Crossover Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MacdCrossoverConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
}

impl Default for MacdCrossoverConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(12),
			slow_period: Some(26),
			signal_period: Some(9),
		}
	}
}

/// Moving Average Crossover configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaCrossoverConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
}

impl Default for MaCrossoverConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(5),
			slow_period: Some(20),
		}
	}
}

/// Stochastic Oscillator configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StochasticConfig {
	pub k_period: Option<u32>,
	pub d_period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for StochasticConfig {
	fn default() -> Self {
		Self {
			k_period: Some(14),
			d_period: Some(3),
			overbought: Some(80.0),
			oversold: Some(20.0),
		}
	}
}

/// VWAP configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VWAPConfig {
	pub anchor_period: Option<String>, // e.g., "daily", "weekly"
}

impl Default for VWAPConfig {
	fn default() -> Self {
		Self {
			anchor_period: Some("daily".to_string()),
		}
	}
}

/// Volume Weighted Average Price Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeWeightedAveragePriceConfig {
	pub period: Option<u32>,
}

impl Default for VolumeWeightedAveragePriceConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// SMA-VWAP Crossover configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SmaVwapCrossoverConfig {
	pub sma_period: Option<u32>,
	pub vwap_period: Option<u32>,
	pub price_source: Option<String>,
	pub anchored: Option<bool>,
	pub session_length: Option<u32>,
}

impl Default for SmaVwapCrossoverConfig {
	fn default() -> Self {
		Self {
			sma_period: Some(3),
			vwap_period: Some(14),
			price_source: Some("hlc3".to_string()),
			anchored: Some(true),
			session_length: Some(1440),
		}
	}
}

/// OBV Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OBVConfig {
	pub lookback_period: Option<u32>,
}

impl Default for OBVConfig {
	fn default() -> Self {
		Self {
			lookback_period: Some(20),
		}
	}
}

/// Volume Price Trend Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumePriceTrendConfig {
	pub min_criteria_met: Option<u32>,
	pub vpt_threshold: Option<f64>,
}

impl Default for VolumePriceTrendConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(1),
			vpt_threshold: Some(0.1),
		}
	}
}

/// Volume Profile RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeProfileRsiConfig {
	pub rsi_period: Option<u32>,
	pub rsi_oversold: Option<f64>,
	pub rsi_overbought: Option<f64>,
	pub volume_profile_bins: Option<u32>,
}

impl Default for VolumeProfileRsiConfig {
	fn default() -> Self {
		Self {
			rsi_period: Some(14),
			rsi_oversold: Some(30.0),
			rsi_overbought: Some(70.0),
			volume_profile_bins: Some(50),
		}
	}
}

/// Cup and Handle Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CupAndHandleConfig {
	pub cup_depth: Option<f64>,
	pub handle_retracement: Option<f64>,
	pub min_duration: Option<u32>,
}

impl Default for CupAndHandleConfig {
	fn default() -> Self {
		Self {
			cup_depth: Some(0.15),
			handle_retracement: Some(0.3),
			min_duration: Some(20),
		}
	}
}

/// Double Top/Bottom Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DoubleTopBottomConfig {
	pub min_distance: Option<u32>,
	pub tolerance: Option<f64>,
	pub lookaround: Option<u32>,
}

impl Default for DoubleTopBottomConfig {
	fn default() -> Self {
		Self {
			min_distance: Some(10),
			tolerance: Some(0.03),
			lookaround: Some(2),
		}
	}
}

/// Flags and Pennants Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlagsPennantsConfig {
	pub pole_length: Option<u32>,
	pub consolidation_bars: Option<u32>,
	pub breakout_threshold: Option<f64>,
	pub additional_buffer: Option<u32>,
}

impl Default for FlagsPennantsConfig {
	fn default() -> Self {
		Self {
			pole_length: Some(10),
			consolidation_bars: Some(10),
			breakout_threshold: Some(0.02),
			additional_buffer: Some(5),
		}
	}
}

/// Head and Shoulders Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HeadAndShouldersConfig {
	pub min_distance: Option<u32>,
	pub tolerance: Option<f64>,
	pub deviation: Option<f64>,
	pub min_data_length: Option<u32>,
}

impl Default for HeadAndShouldersConfig {
	fn default() -> Self {
		Self {
			min_distance: Some(5),
			tolerance: Some(0.02),
			deviation: Some(0.005),
			min_data_length: Some(15),
		}
	}
}

/// Triangle Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriangleConfig {
	pub min_points: Option<u32>,
	pub slope_tolerance: Option<f64>,
	pub min_data_length: Option<u32>,
	pub angle_tolerance: Option<f64>,
}

impl Default for TriangleConfig {
	fn default() -> Self {
		Self {
			min_points: Some(4),
			slope_tolerance: Some(0.01),
			min_data_length: Some(20),
			angle_tolerance: Some(0.001),
		}
	}
}

/// Wedge Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WedgeConfig {
	pub min_points: Option<u32>,
	pub slope_tolerance: Option<f64>,
	pub min_data_length: Option<u32>,
}

impl Default for WedgeConfig {
	fn default() -> Self {
		Self {
			min_points: Some(4),
			slope_tolerance: Some(0.0001),
			min_data_length: Some(20),
		}
	}
}

/// Z-Score Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ZScoreConfig {
	pub mean_period: Option<u32>,
	pub std_period: Option<u32>,
	pub threshold: Option<f64>,
}

impl Default for ZScoreConfig {
	fn default() -> Self {
		Self {
			mean_period: Some(20),
			std_period: Some(20),
			threshold: Some(2.0),
		}
	}
}

/// Percent Rank Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PercentRankConfig {
	pub period: Option<u32>,
	pub entry_percentile: Option<f64>,
	pub exit_percentile: Option<f64>,
}

impl Default for PercentRankConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			entry_percentile: Some(80.0),
			exit_percentile: Some(50.0),
		}
	}
}

/// Correlation Pair Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CorrelationPairConfig {
	pub period: Option<u32>,
	pub entry_threshold: Option<f64>,
	pub exit_threshold: Option<f64>,
	pub spread_method: Option<String>,
	pub second_closes: Option<Vec<f64>>,
}

impl Default for CorrelationPairConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			entry_threshold: Some(0.7),
			exit_threshold: Some(0.3),
			spread_method: Some("ratio".to_string()),
			second_closes: Some(Vec::new()),
		}
	}
}

/// Correlation Reversion Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CorrelationReversionConfig {
	pub period: Option<u32>,
	pub reversion_threshold: Option<f64>,
	pub holding_period: Option<u32>,
	pub second_closes: Option<Vec<f64>>,
}

impl Default for CorrelationReversionConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			reversion_threshold: Some(0.2),
			holding_period: Some(5),
			second_closes: Some(Vec::new()),
		}
	}
}

/// Cointegration Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CointegrationConfig {
	pub period: Option<u32>,
	pub beta_period: Option<u32>,
	pub entry_threshold: Option<f64>,
	pub second_closes: Option<Vec<f64>>,
}

impl Default for CointegrationConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			beta_period: Some(60),
			entry_threshold: Some(2.0),
			second_closes: Some(Vec::new()),
		}
	}
}

/// Accumulation Distribution Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccumulationDistributionConfig {
	pub period: Option<u32>,
}

impl Default for AccumulationDistributionConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// Chaikin Money Flow Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChaikinMoneyFlowConfig {
	pub period: Option<u32>,
}

impl Default for ChaikinMoneyFlowConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// Ease of Movement Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EaseOfMovementConfig {
	pub period: Option<u32>,
}

impl Default for EaseOfMovementConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Force Index Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ForceIndexConfig {
	pub period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for ForceIndexConfig {
	fn default() -> Self {
		Self {
			period: Some(13),
			oversold: Some(-0.1),
			overbought: Some(0.1),
		}
	}
}

/// Money Flow Index Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MoneyFlowIndexConfig {
	pub period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for MoneyFlowIndexConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			oversold: Some(20.0),
			overbought: Some(80.0),
		}
	}
}

/// Negative Volume Index Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NegativeVolumeIndexConfig {
	pub period: Option<u32>,
	pub start: Option<f64>,
}

impl Default for NegativeVolumeIndexConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			start: Some(1000.0),
		}
	}
}

/// OBV Confirmation Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObvConfirmationConfig {
	pub obv_period: Option<u32>,
	pub price_period: Option<u32>,
}

impl Default for ObvConfirmationConfig {
	fn default() -> Self {
		Self {
			obv_period: Some(10),
			price_period: Some(10),
		}
	}
}

/// VWAP Breakout Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapBreakoutConfig {
	pub period: Option<u32>,
	pub breakout_threshold: Option<f64>,
}

impl Default for VwapBreakoutConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			breakout_threshold: Some(0.01),
		}
	}
}

/// VWAP Reversion Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapReversionConfig {
	pub period: Option<u32>,
	pub deviation_threshold: Option<f64>,
}

impl Default for VwapReversionConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			deviation_threshold: Some(0.02),
		}
	}
}
/// MFI OBV Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MfiObvConfig {
	pub mfi_period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for MfiObvConfig {
	fn default() -> Self {
		Self {
			mfi_period: Some(14),
			overbought: Some(80.0),
			oversold: Some(20.0),
		}
	}
}

/// VWAP RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapRsiConfig {
	pub vwap_period: Option<u32>,
	pub rsi_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for VwapRsiConfig {
	fn default() -> Self {
		Self {
			vwap_period: Some(14),
			rsi_period: Some(14),
			oversold: Some(30.0),
			overbought: Some(70.0),
		}
	}
}

/// ADX RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdxRsiConfig {
	pub adx_period: Option<u32>,
	pub trend_threshold: Option<f64>,
	pub rsi_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for AdxRsiConfig {
	fn default() -> Self {
		Self {
			adx_period: Some(14),
			trend_threshold: Some(25.0),
			rsi_period: Some(14),
			oversold: Some(30.0),
			overbought: Some(70.0),
		}
	}
}

/// MA RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaRsiConfig {
	pub ma_period: Option<u32>,
	pub rsi_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for MaRsiConfig {
	fn default() -> Self {
		Self {
			ma_period: Some(20),
			rsi_period: Some(14),
			oversold: Some(30.0),
			overbought: Some(70.0),
		}
	}
}

/// BB RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BbRsiConfig {
	pub bb_period: Option<u32>,
	pub bb_std_dev: Option<f64>,
	pub rsi_period: Option<u32>,
	pub rsi_oversold: Option<f64>,
	pub rsi_overbought: Option<f64>,
}

impl Default for BbRsiConfig {
	fn default() -> Self {
		Self {
			bb_period: Some(20),
			bb_std_dev: Some(2.0),
			rsi_period: Some(14),
			rsi_oversold: Some(30.0),
			rsi_overbought: Some(70.0),
		}
	}
}

/// VWAP MACD Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapMacdConfig {
	pub macd_fast_period: Option<u32>,
	pub macd_slow_period: Option<u32>,
	pub macd_signal_period: Option<u32>,
}

impl Default for VwapMacdConfig {
	fn default() -> Self {
		Self {
			macd_fast_period: Some(12),
			macd_slow_period: Some(26),
			macd_signal_period: Some(9),
		}
	}
}

/// MACD Stochastic Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MacdStochasticConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
	pub k_period: Option<u32>,
	pub d_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for MacdStochasticConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(12),
			slow_period: Some(26),
			signal_period: Some(9),
			k_period: Some(14),
			d_period: Some(3),
			oversold: Some(20.0),
			overbought: Some(80.0),
		}
	}
}

/// ROC OBV RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RocObvRsiConfig {
	pub obv_roc_period: Option<u32>,
	pub rsi_period: Option<u32>,
	pub rsi_overbought: Option<f64>,
	pub rsi_oversold: Option<f64>,
}

impl Default for RocObvRsiConfig {
	fn default() -> Self {
		Self {
			obv_roc_period: Some(3),
			rsi_period: Some(14),
			rsi_overbought: Some(70.0),
			rsi_oversold: Some(30.0),
		}
	}
}

/// Double Top Stochastic Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DoubleTopStochasticConfig {
	pub min_distance: Option<u32>,
	pub tolerance: Option<f64>,
	pub k_period: Option<u32>,
	pub d_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for DoubleTopStochasticConfig {
	fn default() -> Self {
		Self {
			min_distance: Some(10),
			tolerance: Some(0.03),
			k_period: Some(14),
			d_period: Some(3),
			oversold: Some(20.0),
			overbought: Some(80.0),
		}
	}
}

/// Triangle RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriangleRsiConfig {
	pub min_points: Option<u32>,
	pub slope_tolerance: Option<f64>,
	pub rsi_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for TriangleRsiConfig {
	fn default() -> Self {
		Self {
			min_points: Some(4),
			slope_tolerance: Some(0.01),
			rsi_period: Some(14),
			oversold: Some(30.0),
			overbought: Some(70.0),
		}
	}
}

/// VWAP Stochastic Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapStochasticConfig {
	pub vwap_period: Option<u32>,
	pub k_period: Option<u32>,
	pub d_period: Option<u32>,
	pub oversold: Option<f64>,
	pub overbought: Option<f64>,
}

impl Default for VwapStochasticConfig {
	fn default() -> Self {
		Self {
			vwap_period: Some(14),
			k_period: Some(14),
			d_period: Some(3),
			oversold: Some(20.0),
			overbought: Some(80.0),
		}
	}
}

/// VWAP EMA RSI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwapEmaRsiConfig {
	pub ema_fast_period: Option<u32>,
	pub ema_slow_period: Option<u32>,
	pub rsi_period: Option<u32>,
	pub rsi_oversold: Option<f64>,
	pub rsi_overbought: Option<f64>,
}

impl Default for VwapEmaRsiConfig {
	fn default() -> Self {
		Self {
			ema_fast_period: Some(5),
			ema_slow_period: Some(20),
			rsi_period: Some(14),
			rsi_oversold: Some(30.0),
			rsi_overbought: Some(70.0),
		}
	}
}

/// RSI MACD Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RsiMacdConfig {
	pub rsi_period: Option<u32>,
	pub rsi_oversold: Option<f64>,
	pub rsi_overbought: Option<f64>,
	pub macd_fast_period: Option<u32>,
	pub macd_slow_period: Option<u32>,
	pub macd_signal_period: Option<u32>,
}

impl Default for RsiMacdConfig {
	fn default() -> Self {
		Self {
			rsi_period: Some(14),
			rsi_oversold: Some(30.0),
			rsi_overbought: Some(70.0),
			macd_fast_period: Some(12),
			macd_slow_period: Some(26),
			macd_signal_period: Some(9),
		}
	}
}

/// Flag/Pennant + MACD Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlagsPennantsMacdConfig {
	pub pole_length: Option<u32>,
	pub consolidation_bars: Option<u32>,
	pub breakout_threshold: Option<f64>,
	pub additional_buffer: Option<u32>,
	pub macd_fast_period: Option<u32>,
	pub macd_slow_period: Option<u32>,
	pub macd_signal_period: Option<u32>,
}

impl Default for FlagsPennantsMacdConfig {
	fn default() -> Self {
		Self {
			pole_length: Some(10),
			consolidation_bars: Some(10),
			breakout_threshold: Some(0.02),
			additional_buffer: Some(5),
			macd_fast_period: Some(12),
			macd_slow_period: Some(26),
			macd_signal_period: Some(9),
		}
	}
}

/// Acceleration Bands Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccelerationBandsConfig {
	pub period: Option<u32>,
	pub multiplier: Option<f64>,
}

impl Default for AccelerationBandsConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			multiplier: Some(4.0),
		}
	}
}

/// Bollinger Bands Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BollingerBandsConfig {
	pub period: Option<u32>,
	pub std_dev: Option<f64>,
}

impl Default for BollingerBandsConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			std_dev: Some(2.0),
		}
	}
}

/// Donchian Turtle Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DonchianTurtleConfig {
	pub period: Option<u32>,
}

impl Default for DonchianTurtleConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// Keltner Channel Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KeltnerChannelConfig {
	pub period: Option<u32>,
}

impl Default for KeltnerChannelConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// Keltner Volatility Breakout Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KeltnerVolatilityBreakoutConfig {
	pub period: Option<u32>,
}

impl Default for KeltnerVolatilityBreakoutConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// ATR Threshold Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AtrThresholdConfig {
	pub period: Option<u32>,
	pub multiplier: Option<f64>,
}

impl Default for AtrThresholdConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			multiplier: Some(2.0),
		}
	}
}

/// ATR Volatility Threshold Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AtrVolatilityThresholdConfig {
	pub period: Option<u32>,
	pub volatility_threshold: Option<f64>,
}

impl Default for AtrVolatilityThresholdConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			volatility_threshold: Some(1.2),
		}
	}
}

/// Standard Deviation Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StandardDeviationConfig {
	pub period: Option<u32>,
	pub threshold: Option<f64>,
}

impl Default for StandardDeviationConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			threshold: Some(2.0),
		}
	}
}

/// Variance Stop Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VarianceStopConfig {
	pub period: Option<u32>,
	pub multiplier: Option<f64>,
}

impl Default for VarianceStopConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			multiplier: Some(2.0),
		}
	}
}

/// Volatility Adjusted Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolatilityAdjustedConfig {
	pub period: Option<u32>,
	pub target_volatility: Option<f64>,
}

impl Default for VolatilityAdjustedConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			target_volatility: Some(0.15),
		}
	}
}

/// Z-Score Reversion Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ZScoreReversionConfig {
	pub period: Option<u32>,
	pub threshold: Option<f64>,
}

impl Default for ZScoreReversionConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			threshold: Some(2.0),
		}
	}
}

/// MAD Reversion Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MadReversionConfig {
	pub period: Option<u32>,
	pub deviation_multiplier: Option<f64>,
}

impl Default for MadReversionConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			deviation_multiplier: Some(2.0),
		}
	}
}

/// Opening Range Breakout Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OpeningRangeBreakoutConfig {
	pub lookback: Option<u32>,
	pub threshold_pct: Option<f64>,
}

impl Default for OpeningRangeBreakoutConfig {
	fn default() -> Self {
		Self {
			lookback: Some(10),
			threshold_pct: Some(0.02),
		}
	}
}

/// Pairs Trading Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PairsTradingConfig {
	pub period: Option<u32>,
	pub entry_threshold: Option<f64>,
	pub exit_threshold: Option<f64>,
}

impl Default for PairsTradingConfig {
	fn default() -> Self {
		Self {
			period: Some(100),
			entry_threshold: Some(2.0),
			exit_threshold: Some(0.5),
		}
	}
}

/// Projection Oscillator Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectionOscillatorConfig {
	pub period: Option<u32>,
	pub smooth: Option<u32>,
}

impl Default for ProjectionOscillatorConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			smooth: Some(3),
		}
	}
}

/// Fibonacci Retracement Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FibonacciRetracementConfig {
	pub period: Option<u32>,
	pub fib_level: Option<f64>,
}

impl Default for FibonacciRetracementConfig {
	fn default() -> Self {
		Self {
			period: Some(50),
			fib_level: Some(0.618),
		}
	}
}

/// Absolute Price Oscillator Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AbsolutePriceOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
}

impl Default for AbsolutePriceOscillatorConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(10),
			slow_period: Some(20),
		}
	}
}

/// ALMA Crossover Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AlmacrossoverConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub offset: Option<f64>,
}

impl Default for AlmacrossoverConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(9),
			slow_period: Some(21),
			offset: Some(0.85),
		}
	}
}

/// ALMA HMA Divergence Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AlmahmaDivergenceConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub offset: Option<f64>,
	pub divergence_threshold: Option<f64>,
}

impl Default for AlmahmaDivergenceConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(9),
			slow_period: Some(21),
			offset: Some(0.85),
			divergence_threshold: Some(0.01),
		}
	}
}

/// Aroon Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AroonConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for AroonConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(70.0),
			oversold: Some(30.0),
		}
	}
}

/// Balance of Power Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalanceOfPowerConfig {
	pub period: Option<u32>,
}

impl Default for BalanceOfPowerConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Chande Forecast Oscillator Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChandeForecastOscillatorConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for ChandeForecastOscillatorConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(70.0),
			oversold: Some(30.0),
		}
	}
}

/// DMI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DmiConfig {
	pub period_di: Option<u32>,
	pub period_adx: Option<u32>,
	pub adx_threshold: Option<f64>,
}

impl Default for DmiConfig {
	fn default() -> Self {
		Self {
			period_di: Some(14),
			period_adx: Some(14),
			adx_threshold: Some(25.0),
		}
	}
}

/// HMA Trend Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HmaTrendConfig {
	pub period: Option<u32>,
}

impl Default for HmaTrendConfig {
	fn default() -> Self {
		Self { period: Some(21) }
	}
}

/// KDJ Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KdjConfig {
	pub period: Option<u32>,
	pub period1: Option<u32>,
	pub period2: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for KdjConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			period1: Some(3),
			period2: Some(3),
			overbought: Some(80.0),
			oversold: Some(20.0),
		}
	}
}

/// Larsson Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LarssonConfig {
	pub use_consolidating_filter: Option<i32>,
	pub consolidating_lookback: Option<i32>,
	pub consolidating_threshold_pct: Option<f64>,
	pub signal_offset: Option<i32>,
}

impl Default for LarssonConfig {
	fn default() -> Self {
		Self {
			use_consolidating_filter: Some(1),
			consolidating_lookback: Some(10),
			consolidating_threshold_pct: Some(0.02),
			signal_offset: Some(0),
		}
	}
}

/// LinRegChannelConfig is used for Linear Regression Channel trend strategy
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinRegChannelConfig {
	pub period: Option<u32>,
	pub offset: Option<f64>,
}

impl Default for LinRegChannelConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			offset: Some(0.0),
		}
	}
}

/// Linear Regression Channel Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinregChannelConfig {
	pub period: Option<u32>,
	pub std_dev_multiplier: Option<f64>,
}

impl Default for LinregChannelConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			std_dev_multiplier: Some(2.0),
		}
	}
}

/// Linear Regression Slope Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinregSlopeConfig {
	pub period: Option<u32>,
	pub slope_period: Option<u32>,
	pub period_adx: Option<u32>,
	pub adx_threshold: Option<f64>,
}

impl Default for LinregSlopeConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			slope_period: Some(10),
			period_adx: Some(14),
			adx_threshold: Some(25.0),
		}
	}
}

/// Parabolic SAR Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParabolicSarConfig {
	pub step: Option<f64>,
	pub max_step: Option<f64>,
}

impl Default for ParabolicSarConfig {
	fn default() -> Self {
		Self {
			step: Some(0.02),
			max_step: Some(0.02),
		}
	}
}

/// Pivot Points Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PivotPointsConfig {
	pub period: Option<u32>,
	pub period_high: Option<u32>,
	pub period_low: Option<u32>,
}

impl Default for PivotPointsConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			period_high: Some(20),
			period_low: Some(20),
		}
	}
}

/// Super Trend Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuperTrendConfig {
	pub period: Option<u32>,
	pub multiplier: Option<f64>,
}

impl Default for SuperTrendConfig {
	fn default() -> Self {
		Self {
			period: Some(3),
			multiplier: Some(3.0),
		}
	}
}

/// Typical Price Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypicalPriceConfig {}

/// Buy and Hold Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BuyAndHoldConfig {}

/// Vortex Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VortexConfig {
	pub period: Option<u32>,
}

impl Default for VortexConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// VWMA Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VwmaConfig {
	pub period: Option<u32>,
}

impl Default for VwmaConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// WMA Confirmation Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WmaConfirmationConfig {
	pub period: Option<u32>,
	pub threshold: Option<f64>,
}

impl Default for WmaConfirmationConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			threshold: Some(0.02),
		}
	}
}

/// WMA Momentum Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WmaMomentumConfig {
	pub period: Option<u32>,
}

impl Default for WmaMomentumConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Williams R Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WilliamsRConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for WilliamsRConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(-20.0),
			oversold: Some(-80.0),
		}
	}
}

/// KST Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KSTConfig {
	pub roc1_period: Option<u32>,
	pub roc2_period: Option<u32>,
	pub roc3_period: Option<u32>,
	pub roc4_period: Option<u32>,
	pub signal_period: Option<u32>,
}

impl Default for KSTConfig {
	fn default() -> Self {
		Self {
			roc1_period: Some(10),
			roc2_period: Some(15),
			roc3_period: Some(20),
			roc4_period: Some(30),
			signal_period: Some(9),
		}
	}
}

/// ADX Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ADXConfig {
	pub period: Option<u32>,
	pub trend_threshold: Option<f64>,
}

impl Default for ADXConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			trend_threshold: Some(25.0),
		}
	}
}

/// Momentum Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MomentumConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for MomentumConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(70.0),
			oversold: Some(30.0),
		}
	}
}

/// Awesome Oscillator Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AwesomeOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
}

impl Default for AwesomeOscillatorConfig {
	fn default() -> Self {
		Self {
			fast_period: Some(5),
			slow_period: Some(34),
		}
	}
}

/// CCI Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CciConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for CciConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			overbought: Some(100.0),
			oversold: Some(-100.0),
		}
	}
}

/// ROC Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RocConfig {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for RocConfig {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(10.0),
			oversold: Some(-10.0),
		}
	}
}

/// RSI2 Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rsi2Config {
	pub period: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for Rsi2Config {
	fn default() -> Self {
		Self {
			period: Some(14),
			overbought: Some(70.0),
			oversold: Some(30.0),
		}
	}
}

/// Ultimate Oscillator Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UltimateOscillatorConfig {
	pub period1: Option<u32>,
	pub period2: Option<u32>,
	pub period3: Option<u32>,
	pub overbought: Option<f64>,
	pub oversold: Option<f64>,
}

impl Default for UltimateOscillatorConfig {
	fn default() -> Self {
		Self {
			period1: Some(7),
			period2: Some(14),
			period3: Some(28),
			overbought: Some(70.0),
			oversold: Some(30.0),
		}
	}
}

/// Ichimoku Cloud Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IchimokuCloudConfig {
	pub short: Option<u32>,
	pub medium: Option<u32>,
	pub long: Option<u32>,
	pub close: Option<u32>,
}

impl Default for IchimokuCloudConfig {
	fn default() -> Self {
		Self {
			short: Some(9),
			medium: Some(26),
			long: Some(52),
			close: Some(26),
		}
	}
}

/// Elliott Wave Strategy configuration
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ElliottWaveConfig {
	pub wave2_retracement: Option<f64>,
	pub wave4_retracement: Option<f64>,
	pub wave3_min_extension: Option<f64>,
	pub min_wave_separation: Option<u32>,
	pub lookaround: Option<u32>,
	pub retracement_tolerance: Option<f64>,
}

impl Default for ElliottWaveConfig {
	fn default() -> Self {
		Self {
			wave2_retracement: Some(0.618),
			wave4_retracement: Some(0.382),
			wave3_min_extension: Some(1.618),
			min_wave_separation: Some(5),
			lookaround: Some(2),
			retracement_tolerance: Some(0.1),
		}
	}
}
