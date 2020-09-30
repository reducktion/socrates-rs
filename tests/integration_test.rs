use socrates;

#[test]
fn validate_id_portugal_valid_test() {
    assert!(socrates::validate_id("14349483 0 ZV3", socrates::country::Code::PT));
}

#[test]
fn validate_id_portugal_invalid_test() {
    assert!(!socrates::validate_id("11234455677890", socrates::country::Code::PT));
}

#[test]
fn validate_id_france_valid_test() {
    assert!(socrates::validate_id("2820819398814 09", socrates::country::Code::FR));
}

#[test]
fn validate_id_france_invalid_test() {
    assert!(!socrates::validate_id("123X123X123dDAS", socrates::country::Code::FR));
}

