use factors_core;
use napi_derive::napi;

#[napi]
pub fn prediction_market_odds(
	prediction_data: Vec<factors_core::PredictionMarketPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::prediction_market_odds(prediction_data)
}

#[napi]
pub fn odds_momentum(
	prediction_data: Vec<factors_core::PredictionMarketPoint>,
	period: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::odds_momentum(prediction_data, period)
}
