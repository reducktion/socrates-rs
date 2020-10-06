use crate::{Citizen, validator};
use crate::country::Code;
use std::borrow::Borrow;
use chrono::{NaiveDate, Utc};

pub(crate) struct BelgiumValidator;

/**
 Belgium National Identifier Number code validation.

 Validation logic source: https://www.ibz.rrn.fgov.be/fileadmin/user_upload/nl/rr/instructies/IT-lijst/IT000_Rijksregisternummer.pdf

 Another (english) version for the validation can be viewed in wikipedia: https://en.wikipedia.org/wiki/National_identification_number#Belgium
**/
impl validator::CountryValidator for BelgiumValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = id.replace(" ", "").replace(".", "").replace("-","").to_uppercase();
        if standard_id.len() != 11 {
            return false;
        }

        let checksum = standard_id.get(9..11).unwrap();
        let partial_id = standard_id.get(0..9).unwrap();
        if !validate_checksum(partial_id, checksum) {
            let id_for_2000 = "2".to_owned() + partial_id;

            if !validate_checksum(id_for_2000.borrow(), checksum) {
                return false;
            }
            return validate_date(("20".to_owned() + partial_id.get(0..6).unwrap()).borrow());
        }

        return validate_date(("19".to_owned() + partial_id.get(0..6).unwrap()).borrow());
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::BE;
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self::BelgiumValidator::validate_id(&self, id) {
            return None;
        }

        let standard_id = id.replace(" ", "").replace(".", "").replace("-","").to_uppercase();
        let before2000 = validate_checksum(standard_id.get(0..9).unwrap(), standard_id.get(9..11).unwrap());

        let gender = if (standard_id.get(6..9).unwrap().parse::<u64>().unwrap() % 2) == 0 { 'F' } else { 'M' };
        let year = if before2000 {
            ("19".to_owned() + standard_id.get(0..2).unwrap()).parse().unwrap()
        } else {
            ("20".to_owned() + standard_id.get(0..2).unwrap()).parse().unwrap()
        };

        return Some(Citizen {
            gender,
            year_of_birth: year,
            month_of_birth: Some(standard_id.get(2..4).unwrap().parse::<u8>().unwrap()),
            day_of_birth: Some(standard_id.get(4..6).unwrap().parse::<u8>().unwrap()),
            place_of_birth: None,
        });
    }
}

fn validate_checksum(id: &str, checksum: &str) -> bool {
    let calculated_checksum = 97 - (id.parse::<u64>().unwrap() % 97);
    return checksum.parse::<u64>().unwrap() == calculated_checksum;
}

fn validate_date(date: &str) -> bool {
    let date_of_birth = NaiveDate::from_ymd(date.get(0..4).unwrap().parse().unwrap(), date.get(4..6).unwrap().parse().unwrap(), date.get(6..8).unwrap().parse().unwrap());
    return date_of_birth <= Utc::now().naive_local().date();
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn be_validator_requires_min_len_of_11() {
        let validator = super::validator::belgium::BelgiumValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123 456 789 0"));
    }

    #[test]
    fn be_validator_invalid_ids() {
        let validator = super::validator::belgium::BelgiumValidator;
        assert_eq!(validator.validate_id("12.12.12-132.32"), false);
        assert_eq!(validator.validate_id("97.12.03-123.12"), false);
        assert_eq!(validator.validate_id("01.06.18-468.99"), false);
        assert_eq!(validator.validate_id("64.04.09-874.43"), false);
        assert_eq!(validator.validate_id("12.10.23-954.11"), false);
    }

    #[test]
    fn be_validator_valid_ids() {
        let validator = super::validator::belgium::BelgiumValidator;
        assert_eq!(validator.validate_id("93.05.18-223.61"), true);
        assert_eq!(validator.validate_id("730111-361-73"), true);
        assert_eq!(validator.validate_id("75.12.05-137.14"), true);
        assert_eq!(validator.validate_id("09.08.24-282.48"), true);
        assert_eq!(validator.validate_id("71.09.07-213.64"), true);
    }

    #[test]
    fn be_extractor() {
        let validator = super::validator::belgium::BelgiumValidator;
        let citizen_shahin = validator.extract_citizen("93.05.18-223.61").unwrap();
        assert_eq!(citizen_shahin.gender, 'M');
        assert_eq!(citizen_shahin.year_of_birth, 1993);
        assert_eq!(citizen_shahin.month_of_birth.unwrap(), 5);
        assert_eq!(citizen_shahin.day_of_birth.unwrap(), 18);

        let citizen_naoual = validator.extract_citizen("730111-361-73").unwrap();
        assert_eq!(citizen_naoual.gender, 'M');
        assert_eq!(citizen_naoual.year_of_birth, 1973);
        assert_eq!(citizen_naoual.month_of_birth.unwrap(), 1);
        assert_eq!(citizen_naoual.day_of_birth.unwrap(), 11);

        let citizen_xavi = validator.extract_citizen("75.12.05-137.14").unwrap();
        assert_eq!(citizen_xavi.gender, 'M');
        assert_eq!(citizen_xavi.year_of_birth, 1975);
        assert_eq!(citizen_xavi.month_of_birth.unwrap(), 12);
        assert_eq!(citizen_xavi.day_of_birth.unwrap(), 5);

        let citizen_xavi = validator.extract_citizen("09.08.24-282.48").unwrap();
        assert_eq!(citizen_xavi.gender, 'F');
        assert_eq!(citizen_xavi.year_of_birth, 2009);
        assert_eq!(citizen_xavi.month_of_birth.unwrap(), 8);
        assert_eq!(citizen_xavi.day_of_birth.unwrap(), 24);

        let citizen_kurt = validator.extract_citizen("71.09.07-213.64").unwrap();
        assert_eq!(citizen_kurt.gender, 'M');
        assert_eq!(citizen_kurt.year_of_birth, 1971);
        assert_eq!(citizen_kurt.month_of_birth.unwrap(), 9);
        assert_eq!(citizen_kurt.day_of_birth.unwrap(), 7);
    }
}