use crate::country::Code;
use crate::{validator, Citizen};

pub(crate) struct PortugalValidator;

/**
 Portugal national citizen card number validation.

 This validation algorithm is based on the official documentation released in 26 of January of 2009.
 Link: https://www.autenticacao.gov.pt/documents/20126/115760/Valida%C3%A7%C3%A3o+de+N%C3%BAmero+de+Documento+do+Cart%C3%A3o+de+Cidad%C3%A3o.pdf/bdc4eb37-7316-3ff4-164a-f869382b7053
**/
impl validator::CountryValidator for PortugalValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if standard_id.len() != 12 {
            return false;
        }

        let mut every_other_digit = false;
        let mut sum: u32 = 0;
        for char in standard_id.chars().rev() {
            let potential_char_value = char.to_digit(36);
            if potential_char_value.is_none() {
                return false;
            }

            let mut char_value = potential_char_value.unwrap();
            if every_other_digit {
                char_value *= 2;
                if char_value > 9 {
                    char_value -= 9;
                }
            }

            sum += char_value;
            every_other_digit = !every_other_digit;
        }

        return (sum % 10) == 0;
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::PT;
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn pt_validator_requires_min_len_of_12() {
        let validator = super::validator::portugal::PortugalValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123 456 789 0"));
    }

    #[test]
    fn pt_validator_invalid_ids() {
        let validator = super::validator::portugal::PortugalValidator;
        assert_eq!(validator.validate_id("14897475 4 ZY5"), false);
    }

    #[test]
    fn pt_validator_valid_ids() {
        let validator = super::validator::portugal::PortugalValidator;
        assert_eq!(validator.validate_id("11084129 8 ZX8"), true);
        assert_eq!(validator.validate_id("154203556ZX9"), true);
        assert_eq!(validator.validate_id("17653917 4ZZ5"), true);
        assert_eq!(validator.validate_id("174886721 ZX1"), true);
        assert_eq!(validator.validate_id("14898475 4 ZY5"), true);
    }

    #[test]
    fn pt_extractor() {
        let validator = super::validator::portugal::PortugalValidator;
        assert_eq!(validator.extract_citizen("11084129 8 ZX8").is_none(), true);
    }
}
