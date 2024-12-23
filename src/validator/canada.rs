use crate::country::Code;
use crate::validator::algorithms;
use crate::{validator, Citizen};

pub(crate) struct CanadaValidator;

/**
 Canada Social Insurance Number code validation.

 TODO - Add Official Source

 Another (english) version can be viewed in wikipedia: https://en.wikipedia.org/wiki/Social_Insurance_Number
**/
impl validator::CountryValidator for CanadaValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);

        if standard_id.len() != 9 {
            return false;
        }

        algorithms::validate_luhn_10(&standard_id)
    }

    fn country_code(&self) -> Code {
        Code::CA
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn ca_validator_requires_len_of_9() {
        let validator = super::validator::canada::CanadaValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-7"));
    }

    #[test]
    fn ca_validator_invalid_ids() {
        let validator = super::validator::canada::CanadaValidator;
        assert_eq!(validator.validate_id("512 917 638"), false);
        assert_eq!(validator.validate_id("322 710 094"), false);
        assert_eq!(validator.validate_id("761 999 512"), false);
        assert_eq!(validator.validate_id("061 003 528"), false);
    }

    #[test]
    fn ca_validator_valid_ids() {
        let validator = super::validator::canada::CanadaValidator;
        assert_eq!(validator.validate_id("046 454 286"), true);
        assert_eq!(validator.validate_id("671 143 899"), true);
        assert_eq!(validator.validate_id("002 371 920"), true);
        assert_eq!(validator.validate_id("501 343 719"), true);
        assert_eq!(validator.validate_id("912 046 737"), true);
    }
}
