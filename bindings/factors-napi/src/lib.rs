use napi_derive::napi;

pub mod composite;
pub mod event;
pub mod expectations;
pub mod growth;
pub mod onchain;
pub mod prediction;
pub mod quality;
pub mod reit;
pub mod shareholder;
pub mod solvency;
pub mod value;

pub use composite::*;
pub use event::*;
pub use expectations::*;
pub use factors_core::{
	Bar, EarningsReportPoint, FactorPoint, FundamentalPoint, FundamentalPointData,
	OnChainDataPoint, PredictionMarketPoint,
};
pub use growth::*;
pub use onchain::*;
pub use prediction::*;
pub use quality::*;
pub use reit::*;
pub use shareholder::*;
pub use solvency::*;
pub use value::*;

/// Utility to verify the module loads and returns the version.
#[napi]
pub fn init() -> f64 {
	0.1
}
