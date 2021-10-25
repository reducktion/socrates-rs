use crate::country::Code;
use crate::validator::algorithms;
use crate::{validator, Citizen};

pub(crate) struct LuxembourgValidator;

/**
 Luxembourg National Identifier Number code validation.

 TIN validation logic source: https://www.oecd.org/tax/automatic-exchange/crs-implementation-and-assistance/tax-identification-numbers/Luxembourg-TIN.pdf

 Another (english) version for the validation can be viewed in wikipedia: https://en.wikipedia.org/wiki/National_identification_number#Luxembourg
**/
impl validator::CountryValidator for LuxembourgValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = id.replace(" ", "").replace("-", "");
        if standard_id.len() != 13 {
            return false;
        }
        if !algorithms::validate_luhn_10(&standard_id[0..12]) {
            return false;
        }

        return algorithms::validate_verhoeff(&standard_id);
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::LU;
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn lu_validator_requires_len_of_13() {
        let validator = super::validator::luxembourg::LuxembourgValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-7"));
    }

    #[test]
    fn lu_validator_invalid_ids() {
        let validator = super::validator::luxembourg::LuxembourgValidator;
        assert_eq!(validator.validate_id("1994789587182"), false);
        assert_eq!(validator.validate_id("5971654782313"), false);
        assert_eq!(validator.validate_id("2055101054879"), false);
        assert_eq!(validator.validate_id("1997053045687"), false);
    }

    #[test]
    fn lu_validator_valid_ids() {
        let validator = super::validator::luxembourg::LuxembourgValidator;
        assert_eq!(validator.validate_id("1983081246783"), true);
        assert_eq!(validator.validate_id("2003042581931"), true);
        assert_eq!(validator.validate_id("1971110258746"), true);
        assert_eq!(validator.validate_id("2012051469336"), true);
        assert_eq!(validator.validate_id("1994092874551"), true);
    }
}
