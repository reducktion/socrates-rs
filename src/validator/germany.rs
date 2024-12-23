use crate::country::Code;
use crate::{validator, Citizen};

pub(crate) struct GermanyValidator;

/**
Germany Social Security Number code validation.

A national identification number does not really exist within Germany.
But there's a tax ident number, which is unique for each citizen and assigned by date of birth.
This one is restricted by law to be used as a marker/attribute for tax/finance relevant purposes only,
and doest not to identify people in general.

It's called 'steuerliche Identifikationsnummer' or abbreviated 'Steuer-IdNr.'.
There's no metadata, which could be extracted from this number.

See https://de.wikipedia.org/wiki/Steuerliche_Identifikationsnummer

The algorithm is published in an official document
from: Informations Technik Zentrum Bund
title: Steueridentifikationsnummer (IdNr) nach § 139b AO; Informationen zur Berechnung gültiger Prüfziffern
**/
impl validator::CountryValidator for GermanyValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);

        if standard_id.len() != 11 || &standard_id[0..1] == "0" {
            return false;
        }

        let mut r11 = 10;
        let mut r10;
        let mut repeated = vec![0; 10];
        for (idx, char) in standard_id[0..10].chars().enumerate() {
            if idx > 1 {
                let first = standard_id.chars().nth(idx - 2).unwrap();
                let second = standard_id.chars().nth(idx - 1).unwrap();
                if first == second && first == char {
                    return false;
                }
            }
            repeated[char.to_digit(10).unwrap() as usize] += 1;
            r10 = (char.to_digit(10).unwrap() + r11) % 10;
            if r10 == 0 {
                r10 = 10;
            }

            r11 = (2 * r10) % 11;
        }

        if repeated.iter().max().unwrap() > &3 {
            return false;
        }

        let mut expected_check_sum = 11 - r11;
        if expected_check_sum == 10 {
            expected_check_sum = 0;
        }

        expected_check_sum == standard_id.chars().nth(10).unwrap().to_digit(10).unwrap()
    }

    fn country_code(&self) -> Code {
        Code::DE
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn de_validator_requires_11_numeric_chars() {
        let validator = super::validator::germany::GermanyValidator;
        assert_eq!(false, validator.validate_id(""));
        assert_eq!(false, validator.validate_id("1234567890123"));
        assert_eq!(false, validator.validate_id("12345678901"));
        assert_eq!(false, validator.validate_id("12345678901A"));
    }

    #[test]
    fn de_validator_invalid_ids() {
        let validator = super::validator::germany::GermanyValidator;
        // test id (leading 0)
        assert_eq!(false, validator.validate_id("02476291358"));
        // 4 digits or more are equal
        assert_eq!(false, validator.validate_id("44491234560"));
        // 3 consecutive digits
        assert_eq!(false, validator.validate_id("11145678908"));
    }

    #[test]
    fn de_validator_valid_ids() {
        let validator = super::validator::germany::GermanyValidator;
        assert_eq!(true, validator.validate_id("86095742719"));
        assert_eq!(true, validator.validate_id("47036892816"));
        assert_eq!(true, validator.validate_id("65929970489"));
        assert_eq!(true, validator.validate_id("57549285017"));
        assert_eq!(true, validator.validate_id("25768131411"));
    }

    #[test]
    fn de_validator_empty_citizen() {
        let validator = super::validator::germany::GermanyValidator;
        assert!(validator.extract_citizen("65929970489").is_none())
    }
}
