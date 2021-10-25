use crate::country::Code;
use crate::{validator, Citizen};
use regex::Regex;

pub(crate) struct DenmarkValidator;

const MULTIPLIER: [u32; 10] = [4, 3, 2, 7, 6, 5, 4, 3, 2, 1];

/**
 * National Id for Denmark.
 *
 * Information about this national id can be found at:
 *  - https://en.wikipedia.org/wiki/Personal_identification_number_(Denmark)
 */
impl validator::CountryValidator for DenmarkValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if !Regex::new(r"^\d{10}$").unwrap().is_match(&standard_id) {
            return false;
        }

        let year = extract_year(&standard_id);
        let month = standard_id[2..4].parse::<u32>().unwrap();
        let day = standard_id[0..2].parse::<u32>().unwrap();

        return self.is_date_valid(year, month, day) && validate_checksum(&standard_id);
    }

    fn country_code(&self) -> Code {
        crate::country::Code::DK
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        let standard_id = self.sanitize_id(id);
        let gender = if standard_id[9..].parse::<u32>().unwrap() % 2 == 0 {
            'M'
        } else {
            'F'
        };
        return Some(Citizen {
            gender,
            year_of_birth: extract_year(&standard_id) as i32,
            month_of_birth: Some(standard_id[2..4].parse::<u8>().unwrap()),
            day_of_birth: Some(standard_id[0..2].parse::<u8>().unwrap()),
            place_of_birth: None,
        });
    }
}

fn extract_year(id: &str) -> u32 {
    let century_code = &id[6..7].parse::<u32>().unwrap();
    let year_code = &id[4..6].parse::<u32>().unwrap();
    let century: u32 = match century_code {
        0..=3 => 1900,
        4 | 9 => {
            if year_code <= &36 {
                2000
            } else {
                1900
            }
        }
        &_ => {
            if year_code >= &58 {
                1800
            } else {
                2000
            }
        }
    };
    return century + year_code;
}

fn validate_checksum(id: &str) -> bool {
    let mut sum: u32 = 0;
    for (idx, digit) in id.chars().enumerate() {
        sum += digit.to_digit(36).unwrap() * MULTIPLIER[idx];
    }

    return sum % 11 == 0;
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn dk_validator_requires_10_digits() {
        let validator = super::validator::denmark::DenmarkValidator;
        assert_eq!(false, validator.validate_id(""));
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-78"));
        assert_eq!(false, validator.validate_id("123-456-788-15"));
    }

    #[test]
    fn dk_validator_invalid_ids() {
        let validator = super::validator::denmark::DenmarkValidator;
        assert_eq!(validator.validate_id("161301-0001"), false); // month too high
        assert_eq!(validator.validate_id("311101-0001"), false); // November has 30 days
        assert_eq!(validator.validate_id("321201-0001"), false); // day too high
        assert_eq!(validator.validate_id("290201-0001"), false); // 29 February only exists in leap year
        assert_eq!(validator.validate_id("230321-2454"), false); // bad checksum
    }

    #[test]
    fn dk_validator_valid_ids() {
        let validator = super::validator::denmark::DenmarkValidator;
        assert_eq!(validator.validate_id(" 090792-1395 "), true);
        assert_eq!(validator.validate_id("0705930600"), true);
        assert_eq!(validator.validate_id("1504373068"), true);
        assert_eq!(validator.validate_id("1608881995"), true);
        assert_eq!(validator.validate_id("0404047094"), true);
    }

    #[test]
    fn dk_extractor() {
        let extractor = super::validator::denmark::DenmarkValidator;
        let mut citizen = extractor.extract_citizen("090792-1395").unwrap();
        assert_eq!(citizen.gender, 'F');
        assert_eq!(citizen.year_of_birth, 1992);
        assert_eq!(citizen.month_of_birth.unwrap(), 7);
        assert_eq!(citizen.day_of_birth.unwrap(), 9);

        citizen = extractor.extract_citizen("070593-0600").unwrap();
        assert_eq!(citizen.gender, 'M');
        assert_eq!(citizen.year_of_birth, 1993);
        assert_eq!(citizen.month_of_birth.unwrap(), 5);
        assert_eq!(citizen.day_of_birth.unwrap(), 7);
    }
}
