use crate::country::Code;
use crate::{validator, Citizen};

pub(crate) struct SpainValidator;

const CONTROL_DIGIT: &str = "TRWAGMYFPDXBNJZSQVHLCKE";

/**
 Spain national citizen card number validation.

 This validation algorithm is based on the official documentation.
 Link: http://www.interior.gob.es/web/servicios-al-ciudadano/dni/calculo-del-digito-de-control-del-nif-nie
**/
impl validator::CountryValidator for SpainValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);

        if standard_id.len() != 9 {
            return false;
        }

        let control = &standard_id[standard_id.len() - 1..standard_id.len()];
        let citizen = &standard_id[0..standard_id.len() - 1]
            .replace("X", "0")
            .replace("Y", "1")
            .replace("Z", "2")
            .parse::<usize>()
            .unwrap();

        let result = citizen % 23;
        let validation = &CONTROL_DIGIT[result..result + 1];
        return validation == control;
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::ES;
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn es_validator_requires_min_len_of_9() {
        let validator = super::validator::spain::SpainValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-7"));
    }

    #[test]
    fn es_validator_invalid_ids() {
        let validator = super::validator::spain::SpainValidator;
        assert_eq!(validator.validate_id("05756786M"), false);
    }

    #[test]
    fn es_validator_valid_ids() {
        let validator = super::validator::spain::SpainValidator;
        assert_eq!(validator.validate_id("84345642L"), true);
        assert_eq!(validator.validate_id("Y3338121F"), true);
        assert_eq!(validator.validate_id("40298386V"), true);
        assert_eq!(validator.validate_id("Y0597591L"), true);
        assert_eq!(validator.validate_id("09730915Y"), true);
    }
}
