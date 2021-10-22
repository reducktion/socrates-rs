use chrono::NaiveDate;
use crate::Citizen;
use crate::country;

mod date;
mod regions;
mod algorithms;

pub trait CountryValidator {
    fn validate_id(&self, id: &str) -> bool;
    fn country_code(&self) -> country::Code;
    fn extract_citizen(&self, id: &str) -> Option<Citizen>;

    fn sanitize_id(&self, id: &str) -> String {
        id.replace(" ", "").replace("-", "").to_uppercase()
    }

    fn is_date_valid(&self, year: u32, month: u32, day: u32) -> bool {
        NaiveDate::from_ymd_opt(year as i32, month, day).is_some()
    }
}

mod portugal;
mod france;
mod spain;
mod italy;
mod usa;
mod canada;
mod luxembourg;
mod belgium;
mod denmark;


pub fn get_validator(country: &country::Code) -> Box<dyn CountryValidator> {
    return match country {
        country::Code::BE => Box::new(belgium::BelgiumValidator),
        country::Code::CA => Box::new(canada::CanadaValidator),
        country::Code::DK => Box::new(denmark::DenmarkValidator),
        country::Code::ES => Box::new(spain::SpainValidator),
        country::Code::FR => Box::new(france::FranceValidator),
        country::Code::IT => Box::new(italy::ItalyValidator),
        country::Code::LU => Box::new(luxembourg::LuxembourgValidator),
        country::Code::PT => Box::new(portugal::PortugalValidator),
        country::Code::US => Box::new(usa::UsaValidator)
    };
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::Citizen;
    use crate::country::Code;
    use crate::validator::CountryValidator;
    use strum::IntoEnumIterator;

    struct TestValidator {}

    impl CountryValidator for TestValidator {
        fn validate_id(&self, _id: &str) -> bool {
            panic!()
        }

        fn country_code(&self) -> Code {
            panic!()
        }

        fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
            panic!()
        }
    }

    #[test]
    fn validator_trait() {
        let validator = TestValidator{};
        assert_eq!(validator.sanitize_id("1"), "1");
        assert_eq!(validator.sanitize_id("1-"), "1");
        assert_eq!(validator.sanitize_id("1 "), "1");
        assert_eq!(validator.sanitize_id(" 1 - 2"), "12");

        assert!(validator.is_date_valid(2021, 1, 10));
        assert!(!validator.is_date_valid(2021, 15, 1));
    }

    #[test]
    fn validator_selector() {
        for country in Code::iter() {
            let validator = super::get_validator(&country);
            assert_eq!(mem::discriminant(&country), mem::discriminant(&validator.country_code()));
        }
    }
}