use crate::country;
use crate::Citizen;
use chrono::NaiveDate;

mod algorithms;
mod date;
mod regions;

pub trait CountryValidator {
    fn validate_id(&self, id: &str) -> bool;
    fn country_code(&self) -> country::Code;
    fn extract_citizen(&self, id: &str) -> Option<Citizen>;

    fn sanitize_id(&self, id: &str) -> String {
        id.replace(" ", "")
            .replace("-", "")
            .replace(".", "")
            .to_uppercase()
    }

    fn is_date_valid(&self, year: u32, month: u32, day: u32) -> bool {
        NaiveDate::from_ymd_opt(year as i32, month, day).is_some()
    }
}

mod albania;
mod belgium;
mod bosniaherzegovina;
mod brazil;
mod bulgaria;
mod canada;
mod denmark;
mod france;
mod germany;
mod italy;
mod luxembourg;
mod mexico;
mod portugal;
mod spain;
mod usa;
mod words;

pub fn get_validator(country: &country::Code) -> Box<dyn CountryValidator> {
    match country {
        country::Code::BE => Box::new(belgium::BelgiumValidator),
        country::Code::BR => Box::new(brazil::BrazilValidator),
        country::Code::CA => Box::new(canada::CanadaValidator),
        country::Code::DK => Box::new(denmark::DenmarkValidator),
        country::Code::ES => Box::new(spain::SpainValidator),
        country::Code::FR => Box::new(france::FranceValidator),
        country::Code::IT => Box::new(italy::ItalyValidator),
        country::Code::LU => Box::new(luxembourg::LuxembourgValidator),
        country::Code::PT => Box::new(portugal::PortugalValidator),
        country::Code::US => Box::new(usa::UsaValidator),
        country::Code::DE => Box::new(germany::GermanyValidator),
        country::Code::MX => Box::new(mexico::MexicoValidator),
        country::Code::AL => Box::new(albania::AlbaniaValidator),
        country::Code::BA => Box::new(bosniaherzegovina::BosniaHerzegovinaValidator),
        country::Code::BG => Box::new(bulgaria::BulgariaValidator),
    }
}

#[cfg(test)]
mod tests {
    use crate::country::Code;
    use crate::validator::CountryValidator;
    use crate::Citizen;
    use std::mem;
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
        let validator = TestValidator {};
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
            assert_eq!(
                mem::discriminant(&country),
                mem::discriminant(&validator.country_code())
            );
        }
    }
}
