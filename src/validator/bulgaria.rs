use crate::country::Code;
use crate::validator::CountryValidator;
use crate::Citizen;
use chrono::{Datelike, NaiveDate};
use regex::Regex;

pub(crate) struct BulgariaValidator;

/**
* TODO: Find official docs on validation
*/
impl CountryValidator for BulgariaValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if !Regex::new(r"^\d{10}$").unwrap().is_match(&standard_id) {
            return false;
        }

        let sum = (2 * standard_id[0..1].parse::<u32>().unwrap())
            + (4 * standard_id[1..2].parse::<u32>().unwrap())
            + (8 * standard_id[2..3].parse::<u32>().unwrap())
            + (5 * standard_id[3..4].parse::<u32>().unwrap())
            + (10 * standard_id[4..5].parse::<u32>().unwrap())
            + (9 * standard_id[5..6].parse::<u32>().unwrap())
            + (7 * standard_id[6..7].parse::<u32>().unwrap())
            + (3 * standard_id[7..8].parse::<u32>().unwrap())
            + (6 * standard_id[8..9].parse::<u32>().unwrap());

        let check_digit = standard_id[9..10].parse::<u32>().unwrap();
        check_digit == (sum % 11)
    }

    fn country_code(&self) -> Code {
        Code::BG
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self.validate_id(id) {
            return None;
        }
        let standard_id = self.sanitize_id(id);
        let year = standard_id[0..2].parse::<i32>().unwrap();
        let month = standard_id[2..4].parse::<u32>().unwrap();
        let m;
        let y;
        if month > 40 {
            m = month - 40;
            y = 2000 + year;
        } else if month < 20 {
            m = month;
            y = 1900 + year;
        } else {
            m = month - 20;
            y = 1800 + year;
        }

        let date_of_birth =
            NaiveDate::from_ymd_opt(y, m, standard_id[4..6].parse::<u32>().unwrap());
        if date_of_birth.is_none() {
            return None;
        }
        let dob = date_of_birth.unwrap();

        Some(Citizen {
            gender: if standard_id[9..].parse::<u32>().unwrap() % 2 == 0 {
                'F'
            } else {
                'M'
            },
            year_of_birth: dob.year(),
            month_of_birth: Some(dob.month() as u8),
            day_of_birth: Some(dob.day() as u8),
            place_of_birth: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::bulgaria::BulgariaValidator;
    use crate::validator::CountryValidator;

    #[test]
    fn bg_validator_requires_len_10() {
        let validator = BulgariaValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("12345678901"));
    }

    #[test]
    fn bg_validator_invalid_ids() {
        let validator = BulgariaValidator;
        assert_eq!(false, validator.validate_id("7542021030"));
        assert_eq!(false, validator.validate_id("8002560008"));
        assert_eq!(false, validator.validate_id("3542027033"));
        assert_eq!(false, validator.validate_id("6002567498"));
        assert_eq!(false, validator.validate_id("7542039611"));
    }

    #[test]
    fn bg_validator_valid_ids() {
        let validator = BulgariaValidator;
        assert_eq!(true, validator.validate_id("7523169263"));
        assert_eq!(true, validator.validate_id("8032056031"));
        assert_eq!(true, validator.validate_id("8001010008"));
        assert_eq!(true, validator.validate_id("7501020018"));
        assert_eq!(true, validator.validate_id("7552010005"));
        assert_eq!(true, validator.validate_id("7542011030"));
    }

    #[test]
    fn bg_citizen_extractor() {
        let validator = BulgariaValidator;
        let c1 = validator.extract_citizen("7523169263").unwrap();
        assert_eq!('M', c1.gender);
        assert_eq!(1875, c1.year_of_birth);
        assert_eq!(3, c1.month_of_birth.unwrap());
        assert_eq!(16, c1.day_of_birth.unwrap());

        let c2 = validator.extract_citizen("8001010008").unwrap();
        assert_eq!('F', c2.gender);
        assert_eq!(1980, c2.year_of_birth);
        assert_eq!(1, c2.month_of_birth.unwrap());
        assert_eq!(1, c2.day_of_birth.unwrap());
    }
}
