use crate::Citizen;
use crate::country;

mod date;
mod regions;
mod algorithms;

pub trait CountryValidator {
    fn validate_id(&self, id: &str) -> bool;
    fn country_code(&self) -> country::Code;
    fn extract_citizen(&self, id: &str) -> Option<Citizen>;
}

mod portugal;
mod france;
mod spain;
mod italy;
mod usa;
mod canada;
mod luxembourg;


pub fn get_validator(country: country::Code) -> Box<dyn CountryValidator> {
    return match country {
        country::Code::PT => Box::new(portugal::PortugalValidator),
        country::Code::FR => Box::new(france::FranceValidator),
        country::Code::ES => Box::new(spain::SpainValidator),
        country::Code::IT => Box::new(italy::ItalyValidator),
        country::Code::US => Box::new(usa::UsaValidator),
        country::Code::CA => Box::new(canada::CanadaValidator),
        country::Code::LU => Box::new(luxembourg::LuxembourgValidator)
    };
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn pt_validator_selected_for_portugal_country_code() {
        let validator = super::get_validator(super::country::Code::PT);
        assert_eq!(mem::discriminant(&crate::country::Code::PT), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn fr_validator_selected_for_france_country_code() {
        let validator = super::get_validator(super::country::Code::FR);
        assert_eq!(mem::discriminant(&crate::country::Code::FR), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn es_validator_selected_for_spain_country_code() {
        let validator = super::get_validator(super::country::Code::ES);
        assert_eq!(mem::discriminant(&crate::country::Code::ES), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn it_validator_selected_for_italy_country_code() {
        let validator = super::get_validator(super::country::Code::IT);
        assert_eq!(mem::discriminant(&crate::country::Code::IT), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn us_validator_selected_for_usa_country_code() {
        let validator = super::get_validator(super::country::Code::US);
        assert_eq!(mem::discriminant(&crate::country::Code::US), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn ca_validator_selected_for_canada_country_code() {
        let validator = super::get_validator(super::country::Code::CA);
        assert_eq!(mem::discriminant(&crate::country::Code::CA), mem::discriminant(&validator.country_code()));
    }

    #[test]
    fn lu_validator_selected_for_luxembourg_country_code() {
        let validator = super::get_validator(super::country::Code::LU);
        assert_eq!(mem::discriminant(&crate::country::Code::LU), mem::discriminant(&validator.country_code()));
    }
}