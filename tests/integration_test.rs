use socrates_rs;
use socrates_rs::Citizen;

#[test]
fn validate_id_portugal_valid_test() {
    assert!(socrates_rs::validate_id("14349483 0 ZV3", socrates_rs::country::Code::PT));
}

#[test]
fn validate_id_portugal_invalid_test() {
    assert!(!socrates_rs::validate_id("11234455677890", socrates_rs::country::Code::PT));
}

#[test]
fn validate_id_france_valid_test() {
    assert!(socrates_rs::validate_id("2820819398814 09", socrates_rs::country::Code::FR));
}

#[test]
fn validate_id_france_invalid_test() {
    assert!(!socrates_rs::validate_id("123X123X123dDAS", socrates_rs::country::Code::FR));
}

#[test]
fn extract_france_test() {
    let citizen = socrates_rs::extract_information("2820819398814 09", socrates_rs::country::Code::FR).unwrap();
    assert_eq!(citizen.gender, 'F');
    assert_eq!(citizen.year_of_birth, 1982);
    assert_eq!(citizen.month_of_birth.unwrap(), 8);
    assert_eq!(citizen.place_of_birth.unwrap(), "Corrèze");
}

#[test]
fn generator_denmark() {
    let id = socrates_rs::generate_id(&Citizen {
        gender: 'M',
        year_of_birth: 1991,
        month_of_birth: Some(6),
        day_of_birth: Some(16),
        place_of_birth: None
    }, socrates_rs::country::Code::DK).unwrap();
    assert_eq!(id, "160691-3113");
}
