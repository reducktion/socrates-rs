use crate::country::Code;
use crate::validator::date;
use crate::validator::regions;
use crate::{validator, Citizen};

pub(crate) struct ItalyValidator;

/**
 Italy fiscal code validation.

 The validation algorithm is based on the Italy's Ministry of Finance Decree of 12/03/1974 n. 2227,
  which can be found online in http://www.dossier.net/utilities/codice-fiscale/decreto1974_2227.html

 Another (english) version can be viewed in wikipedia: https://en.wikipedia.org/wiki/Italian_fiscal_code

 The list of regions can be checked in this web archive of the Italian government:
 https://web.archive.org/web/20160819012136/http://www.agenziaentrate.gov.it/wps/wcm/connect/321b0500426a5e2492629bc065cef0e8/codicicatastali_comuni_29_11_2010.pdf?MOD=AJPERES&CACHEID=321b500426a5e2492629bc065cef0e8
**/
impl validator::CountryValidator for ItalyValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if standard_id.len() != 16 {
            return false;
        }

        let mut is_odd = true;
        let mut sum: u32 = 0;
        for char in standard_id[0..15].chars() {
            if is_odd {
                sum += get_odd_char_value(char)
            } else {
                sum += get_even_char_value(char) as u32
            }

            is_odd = !is_odd;
        }

        let control_letter = get_remainder_char(sum % 26).to_string();
        return (standard_id[15..].parse::<String>().unwrap()) == control_letter;
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::IT;
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self::ItalyValidator::validate_id(&self, id) {
            return None;
        }

        return Some(Citizen {
            gender: get_gender(&id[9..11]),
            year_of_birth: date::get_year_of_birth(&id[6..8]),
            month_of_birth: get_month_of_birth(id[8..9].parse::<char>().unwrap()),
            day_of_birth: Some(get_day_of_birth(&id[9..11])),
            place_of_birth: regions::get_region_from_csv(
                &id[11..15],
                "./src/validator/regions/italy_regions.csv",
            ), //italy_regions::get_region(&id[11..15]),
        });
    }
}

fn get_odd_char_value(character: char) -> u32 {
    return match character {
        '0' | 'A' => 1,
        '1' | 'B' => 0,
        '2' | 'C' => 5,
        '3' | 'D' => 7,
        '4' | 'F' => 9,
        '5' | 'E' => 13,
        '6' | 'G' => 15,
        '7' | 'H' => 17,
        '8' | 'I' => 19,
        '9' | 'J' => 21,
        'K' => 2,
        'L' => 4,
        'M' => 18,
        'N' => 20,
        'O' => 11,
        'P' => 3,
        'Q' => 6,
        'R' => 8,
        'S' => 12,
        'T' => 14,
        'U' => 16,
        'V' => 10,
        'W' => 22,
        'X' => 25,
        'Y' => 24,
        'Z' => 23,
        _ => panic!("Unrecognized letter"),
    };
}

fn get_remainder_char(digit: u32) -> char {
    return match digit {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ => panic!("out of range for digit conversion"),
    };
}

fn get_even_char_value(character: char) -> u8 {
    let digit = character.to_digit(36).unwrap() as u8;
    return if digit > 9 { digit - 10_u8 } else { digit };
}

fn get_month_of_birth(letter: char) -> Option<u8> {
    return match letter {
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'H' => Some(6),
        'L' => Some(7),
        'M' => Some(8),
        'P' => Some(9),
        'R' => Some(10),
        'S' => Some(11),
        'T' => Some(12),
        _ => None,
    };
}

fn get_day_of_birth(day_of_birth: &str) -> u8 {
    let day = day_of_birth.parse::<u8>().unwrap();
    return if day > 40 { day - 40_u8 } else { day };
}

fn get_gender(day_of_birth: &str) -> char {
    let day = day_of_birth.parse::<u8>().unwrap();
    return if day > 40 { 'F' } else { 'M' };
}

#[cfg(test)]
mod tests {
    use crate::validator::italy::{get_even_char_value, get_odd_char_value};
    use crate::validator::CountryValidator;

    #[test]
    fn it_validator_requires_len_of_16() {
        let validator = super::validator::italy::ItalyValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-7"));
    }

    #[test]
    fn even_char_converter() {
        assert_eq!(0, get_even_char_value('A'));
        assert_eq!(0, get_even_char_value('0'));
        assert_eq!(3, get_even_char_value('3'));
        assert_eq!(25, get_even_char_value('Z'));
    }

    #[test]
    fn odd_char_converter() {
        assert_eq!(1, get_odd_char_value('A'));
        assert_eq!(1, get_odd_char_value('0'));
        assert_eq!(7, get_odd_char_value('3'));
        assert_eq!(23, get_odd_char_value('Z'));
    }

    #[test]
    fn it_validator_invalid_ids() {
        let validator = super::validator::italy::ItalyValidator;
        assert_eq!(validator.validate_id("MECDRE01A11A025E"), false);
        assert_eq!(validator.validate_id("ARSLGE02D50H987A"), false);
        assert_eq!(validator.validate_id("CSTDAM75B06C215T"), false);
        assert_eq!(validator.validate_id("ARLSNT66P65Z404R"), false);
    }

    #[test]
    fn it_validator_valid_ids() {
        let validator = super::validator::italy::ItalyValidator;
        assert_eq!(validator.validate_id("MRTMTT25D09F205Z"), true);
        assert_eq!(validator.validate_id("MLLSNT82P65Z404U"), true);
        assert_eq!(validator.validate_id("DLMCTG75B07H227Y"), true);
        assert_eq!(validator.validate_id("BRSLSE08D50H987B"), true);
        assert_eq!(validator.validate_id("MRCDRA01A13A065E"), true);
    }

    #[test]
    fn it_extractor_returns_none_for_invalid_id() {
        let validator = super::validator::italy::ItalyValidator;
        assert_eq!(
            validator.extract_citizen("ARLSNT66P65Z404R 01").is_none(),
            true
        );
    }

    #[test]
    fn it_extractor_returns_citizen_for_valid_ids() {
        let validator = super::validator::italy::ItalyValidator;

        let citizen_annette = validator.extract_citizen("MRTMTT25D09F205Z").unwrap();
        assert_eq!(citizen_annette.gender, 'M');
        assert_eq!(citizen_annette.year_of_birth, 1925);
        assert_eq!(citizen_annette.month_of_birth.unwrap(), 4);
        assert_eq!(citizen_annette.day_of_birth.unwrap(), 9);
        assert_eq!(citizen_annette.place_of_birth.unwrap(), "MILANO (MI)");

        let citizen_lothair = validator.extract_citizen("MLLSNT82P65Z404U").unwrap();
        assert_eq!(citizen_lothair.gender, 'F');
        assert_eq!(citizen_lothair.year_of_birth, 1982);
        assert_eq!(citizen_lothair.month_of_birth.unwrap(), 9);
        assert_eq!(citizen_lothair.day_of_birth.unwrap(), 25);
        assert_eq!(
            citizen_lothair.place_of_birth.unwrap(),
            "STATI UNITI D'AMERICA"
        );
    }
}
