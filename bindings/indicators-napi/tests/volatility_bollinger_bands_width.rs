use indicators::bbw;
use indicators_core::BBResult;

#[test]
fn test_bollinger_bands_width_empty_bb() {
	let empty_bb = BBResult {
		upper: vec![],
		middle: vec![],
		lower: vec![],
	};

	let result = bbw(empty_bb, Some(14));

	assert!(result.is_err());
}
