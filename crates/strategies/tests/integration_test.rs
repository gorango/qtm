use strategies_core::buy_and_hold_strategy;

#[test]
fn integration_buy_and_hold_first_signal_is_one() {
	let closes = vec![100.0, 101.0, 102.0];
	let result = buy_and_hold_strategy(&closes, None).unwrap();
	assert_eq!(result[0], 1);
	assert_eq!(result[1], 0);
	assert_eq!(result[2], 0);
}

#[test]
fn integration_buy_and_hold_empty_errors() {
	let closes: Vec<f64> = vec![];
	let result = buy_and_hold_strategy(&closes, None);
	assert!(result.is_err());
	assert_eq!(result.err().unwrap(), "Input arrays cannot be empty");
}

#[test]
fn integration_buy_and_hold_single_element() {
	let closes = vec![42.0];
	let result = buy_and_hold_strategy(&closes, None).unwrap();
	assert_eq!(result[0], 1);
}

#[test]
fn integration_buy_and_hold_with_default_config() {
	let closes = vec![10.0, 20.0, 30.0];
	let config = strategies_core::BuyAndHoldConfig::default();
	let result = buy_and_hold_strategy(&closes, Some(config)).unwrap();
	assert_eq!(result[0], 1);
	assert_eq!(result[1], 0);
}
