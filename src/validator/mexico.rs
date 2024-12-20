use crate::country::Code;
use crate::validator::date::get_year_of_birth;
use crate::validator::words::get_inappropriate_words_mexico;
use crate::{validator, Citizen};
use chrono::NaiveDate;

pub(crate) struct MexicoValidator;

/**
 * National Id for Mexico.
 *
 * Expected format:
 *  [surname initial] [surname inside vowel] [second surname initial] [first name initial]
 *  [last 2 digits of year of birth] [2 digits of month of birth] [2 digits of day of birth]
 *  [gender letter: M, H, X] [2 letter indicator of the state of birth]
 *  [surname inside consonant] [second surname inside consonant] [first name inside consonant]
 *  [single char 0-9 or A-Z for people born before or after 2000, respectively]
 *
 * Example: Gloria Hernández García, a female, born on 27 April 1956 in the state of Veracruz
 * would be HEGG560427
 *
 * Information about this national id can be found at:
 *  - https://en.wikipedia.org/wiki/Unique_Population_Registry_Code
 *  - http://sistemas.uaeh.edu.mx/dce/admisiones/docs/guia_CURP.pdf
 *  - (Inappropriate words) https://solucionfactible.com/sfic/resources/files/palabrasInconvenientes-rfc.pdf
**/
impl validator::CountryValidator for MexicoValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = id.replace(" ", "").replace("-", "");

        if standard_id.len() != 18 || !standard_id[0..4].chars().all(|c| c.is_alphabetic()) {
            return false;
        }

        let month_of_birth = standard_id[6..8].parse::<u32>();
        let day_of_birth = standard_id[8..10].parse::<u32>();

        if !standard_id[4..6].chars().all(|c| c.is_numeric())
            || month_of_birth.is_err()
            || day_of_birth.is_err()
        {
            return false;
        }
        let year_of_birth = get_year_of_birth(standard_id[4..6].chars().as_str());
        let date_of_birth = NaiveDate::from_ymd_opt(
            year_of_birth,
            month_of_birth.unwrap(),
            day_of_birth.unwrap(),
        );
        if date_of_birth.is_none() {
            return false;
        }

        let gender = standard_id[10..11].chars().nth(0).unwrap();
        if gender != 'M' && gender != 'H' && gender != 'X' {
            return false;
        }

        if !standard_id[11..16].chars().all(|c| c.is_alphabetic()) {
            return false;
        }

        let assigned_digit = standard_id[16..17].chars().nth(0).unwrap();
        if (year_of_birth > 2000 && assigned_digit.is_digit(10))
            || (year_of_birth <= 2000 && assigned_digit.is_alphabetic())
        {
            return false;
        }

        if get_inappropriate_words_mexico()
            .iter()
            .find(|word| standard_id.contains(*word))
            .is_some()
        {
            return false;
        };

        const DICTIONARY: &str = "0123456789ABCDEFGHIJKLMN&OPQRSTUVWXYZ";
        let mut sum = 0;
        let l = standard_id.len();
        standard_id[0..17].chars().enumerate().for_each(|(i, c)| {
            sum += DICTIONARY.find(c).unwrap() * (l - i);
        });
        let check_digit = standard_id[17..18].parse::<u32>();

        !check_digit.is_err() && check_digit.unwrap() == ((10 - (sum % 10)) % 10) as u32
    }

    fn country_code(&self) -> Code {
        Code::MX
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self.validate_id(id) {
            return None;
        }
        let standard_id = id.replace(" ", "").replace("-", "");
        Some(Citizen {
            gender: match standard_id[10..11].chars().nth(0).unwrap() {
                'M' => 'F',
                'H' => 'M',
                _ => 'X',
            },
            year_of_birth: get_year_of_birth(standard_id[4..6].chars().as_str()),
            month_of_birth: Some(standard_id[6..8].parse::<u32>().unwrap() as u8),
            day_of_birth: Some(standard_id[8..10].parse::<u32>().unwrap() as u8),
            place_of_birth: extract_place_of_birth(standard_id[11..13].to_string()),
        })
    }
}

fn extract_place_of_birth(code: String) -> Option<String> {
    match code.as_str() {
        "AS" => Some("AGUASCALIENTES".to_string()),
        "BS" => Some("BAJA CALIFORNIA SUR".to_string()),
        "CL" => Some("COAHUILA".to_string()),
        "CS" => Some("CHIAPAS".to_string()),
        "DF" => Some("DISTRITO FEDERAL".to_string()),
        "GT" => Some("GUANAJUATO".to_string()),
        "HG" => Some("HIDALGO".to_string()),
        "MC" => Some("MÉXICO".to_string()),
        "MS" => Some("MORELOS".to_string()),
        "NL" => Some("NUEVO LEÓN".to_string()),
        "PL" => Some("PUEBLA".to_string()),
        "QR" => Some("QUINTANA ROO".to_string()),
        "SL" => Some("SINALOA".to_string()),
        "TC" => Some("TABASCO".to_string()),
        "TL" => Some("TLAXCALA".to_string()),
        "YN" => Some("YUCATÁN".to_string()),
        "NE" => Some("NACIDO EN EL EXTRANJERO".to_string()),
        "BC" => Some("BAJA CALIFORNIA".to_string()),
        "CC" => Some("CAMPECHE".to_string()),
        "CM" => Some("COLIMA".to_string()),
        "CH" => Some("CHIHUAHUA".to_string()),
        "DG" => Some("DURANGO".to_string()),
        "GR" => Some("GUERRERO".to_string()),
        "JC" => Some("JALISCO".to_string()),
        "MN" => Some("MICHOACÁN".to_string()),
        "NT" => Some("NAYARIT".to_string()),
        "OC" => Some("OAXACA".to_string()),
        "QT" => Some("QUERÉTARO".to_string()),
        "SP" => Some("SAN LUIS POTOSÍ".to_string()),
        "SR" => Some("SONORA".to_string()),
        "TS" => Some("TAMAULIPAS".to_string()),
        "VZ" => Some("VERACRUZ".to_string()),
        "ZS" => Some("ZACATECAS".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn mx_validator_requires_18_chars() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id(""));
        assert_eq!(false, validator.validate_id("1234567890123456789"));
        assert_eq!(false, validator.validate_id("12345678901234567"));
    }

    #[test]
    fn mx_validator_requires_name_initials() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("0000101109MHGNMN01"));
    }

    #[test]
    fn mx_validator_requires_date_of_birth() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("JOIMAAAAAAHHGSMN08"));
        assert_eq!(false, validator.validate_id("AAIM901312MBCNMN08"));
        assert_eq!(false, validator.validate_id("AAIM901131MBCNMN08"));
        assert_eq!(false, validator.validate_id("AAIM901232MBCNMN08"));
        assert_eq!(false, validator.validate_id("AAIM010229MBCNMN08"));
    }

    #[test]
    fn mx_validator_requires_gender() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("AAIM901112VBCNMN08"));
    }

    #[test]
    fn mx_validator_requires_state_of_birth() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("AAIM901112H99NMN08"));
    }

    #[test]
    fn mx_validator_requires_name_consonants() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("AAIT101109MVZ11101"));
    }

    #[test]
    fn mx_validator_requires_assigned_char() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("HEGG560427MVZRRLA4"));
        assert_eq!(false, validator.validate_id("HEGG040427MVZRRL04"));
    }

    #[test]
    fn mx_validator_blocks_innappropriate_words() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("BUEI150102HAAAAAC1"));
        assert_eq!(false, validator.validate_id("KAGO010101XAAAAAD1"));
        assert_eq!(false, validator.validate_id("PEDA560102MAAAAA01"));
        assert_eq!(false, validator.validate_id("RATA001001HAAAAA21"));
    }

    #[test]
    fn mx_validator_invalid_control_digit() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(false, validator.validate_id("BMHM260906HCHQAN04"));
    }

    #[test]
    fn mx_validator_valid_ids() {
        let validator = super::validator::mexico::MexicoValidator;
        assert_eq!(true, validator.validate_id(" AAIM901112MBCNMN08 "));
        assert_eq!(true, validator.validate_id("JOIM890106HHGSMN08"));
        assert_eq!(true, validator.validate_id("JOTA950616HBCSWS03"));
    }

    #[test]
    fn mx_validator_extract_citizen() {
        let validator = super::validator::mexico::MexicoValidator;
        let citizen = validator.extract_citizen("AAIM901112MBCNMN08");
        assert_eq!(true, citizen.is_some());
        let c = citizen.unwrap();
        assert_eq!('F', c.gender);
        assert_eq!(1990, c.year_of_birth);
        assert_eq!(11, c.month_of_birth.unwrap());
        assert_eq!(12, c.day_of_birth.unwrap());
        assert_eq!("BAJA CALIFORNIA", c.place_of_birth.unwrap());

        let citizen = validator.extract_citizen("AAJM900827MGTDPS05");
        assert_eq!(true, citizen.is_some());
        let c = citizen.unwrap();
        assert_eq!('F', c.gender);
        assert_eq!(1990, c.year_of_birth);
        assert_eq!(8, c.month_of_birth.unwrap());
        assert_eq!(27, c.day_of_birth.unwrap());
        assert_eq!("GUANAJUATO", c.place_of_birth.unwrap());

        let citizen = validator.extract_citizen("JOIM890106HHGSMN08");
        assert_eq!(true, citizen.is_some());
        let c = citizen.unwrap();
        assert_eq!('M', c.gender);
        assert_eq!(1989, c.year_of_birth);
        assert_eq!(1, c.month_of_birth.unwrap());
        assert_eq!(6, c.day_of_birth.unwrap());
        assert_eq!("HIDALGO", c.place_of_birth.unwrap());
    }
}
