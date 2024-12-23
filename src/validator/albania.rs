use crate::country::Code;
use crate::{validator, Citizen};
use chrono::NaiveDate;
use regex::Regex;

pub(crate) struct AlbaniaValidator;

/**
Albania National Identifier Number code validation.

TODO: Add official source with checksum calculation logic

Validation logic source: https://lookuptax.com/docs/tax-identification-number/albania-tax-id-guide

**/
impl validator::CountryValidator for AlbaniaValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if !Regex::new(r"^[A-T]\d[0156]\d{6}[A-W]$")
            .unwrap()
            .is_match(&standard_id)
        {
            return false;
        }

        let first_letter = standard_id[0..1].chars().nth(0).unwrap();

        let year_or_birth = extract_year_of_birth(standard_id.to_string());
        if year_or_birth.is_none() {
            return false;
        }

        let gender = convert_gender(standard_id.to_string()).unwrap();
        let month_of_birth = convert_month(&standard_id[2..4], gender);
        let day_of_birth = standard_id[4..6].parse::<u32>().unwrap();

        let date_of_birth =
            NaiveDate::from_ymd_opt(year_or_birth.unwrap(), month_of_birth, day_of_birth);
        if date_of_birth.is_none() {
            return false;
        }

        let mut sum = 0;
        standard_id[1..9]
            .chars()
            .enumerate()
            .for_each(|(idx, c)| sum += (c.to_digit(10).unwrap() as usize) * (idx + 1));
        sum += convert_letter_table(first_letter).unwrap() as usize;
        let expected_check_letter = convert_number_table(sum % 23);

        expected_check_letter.is_some()
            && expected_check_letter.unwrap() == standard_id[9..].chars().nth(0).unwrap()
    }

    fn country_code(&self) -> Code {
        Code::AL
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self.validate_id(id) {
            return None;
        }

        let standard_id = self.sanitize_id(id);
        let g = convert_gender(standard_id.to_string()).unwrap();
        Some(Citizen {
            gender: g,
            year_of_birth: extract_year_of_birth(standard_id.to_string()).unwrap(),
            month_of_birth: Some(convert_month(&standard_id[2..4], g) as u8),
            day_of_birth: Some(standard_id[4..6].parse::<u8>().unwrap()),
            place_of_birth: None,
        })
    }
}

fn extract_year_of_birth(standard_id: String) -> Option<i32> {
    let first_letter = standard_id[0..1].chars().nth(0).unwrap();

    let birth_decade = match first_letter {
        'A' => Some(1900),
        'B' => Some(1910),
        'C' => Some(1920),
        'D' => Some(1930),
        'E' => Some(1940),
        'F' => Some(1950),
        'G' => Some(1960),
        'H' => Some(1970),
        'I' => Some(1980),
        'J' => Some(1990),
        'K' => Some(2000),
        'L' => Some(2010),
        'M' => Some(2020),
        'N' => Some(2030),
        'O' => Some(2040),
        'P' => Some(2050),
        'Q' => Some(2060),
        'R' => Some(2070),
        'S' => Some(2080),
        'T' => Some(2090),
        _ => None,
    };

    let year_of_decade = standard_id[1..2].to_string().parse::<i32>();

    if birth_decade.is_none() || year_of_decade.is_err() {
        return None;
    }

    Some(birth_decade.unwrap() + year_of_decade.unwrap())
}

fn convert_gender(id: String) -> Option<char> {
    let gender = id[2..3].chars().nth(0).unwrap();

    if gender == '0' || gender == '1' {
        return Some('M');
    } else if gender == '5' || gender == '6' {
        return Some('F');
    }
    None
}

fn convert_letter_table(letter: char) -> Option<u32> {
    match letter {
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'F' => Some(6),
        'G' => Some(7),
        'H' => Some(8),
        'I' => Some(9),
        'J' => Some(10),
        'K' => Some(11),
        'L' => Some(12),
        'M' => Some(13),
        'N' => Some(14),
        'O' => Some(15),
        'P' => Some(16),
        'Q' => Some(17),
        'R' => Some(18),
        'S' => Some(19),
        'T' => Some(20),
        'U' => Some(21),
        'V' => Some(22),
        'W' => Some(0),
        _ => None,
    }
}

fn convert_number_table(n: usize) -> Option<char> {
    match n {
        1 => Some('A'),
        2 => Some('B'),
        3 => Some('C'),
        4 => Some('D'),
        5 => Some('E'),
        6 => Some('F'),
        7 => Some('G'),
        8 => Some('H'),
        9 => Some('I'),
        10 => Some('J'),
        11 => Some('K'),
        12 => Some('L'),
        13 => Some('M'),
        14 => Some('N'),
        15 => Some('O'),
        16 => Some('P'),
        17 => Some('Q'),
        18 => Some('R'),
        19 => Some('S'),
        20 => Some('T'),
        21 => Some('U'),
        22 => Some('V'),
        0 => Some('W'),
        _ => None,
    }
}

fn convert_month(month: &str, gender: char) -> u32 {
    let m = month.parse::<u32>().unwrap();
    if m < 40 && gender == 'F' {
        return 0;
    }
    if gender == 'F' {
        return m - 50;
    }
    m
}

#[cfg(test)]
mod tests {
    use crate::validator::albania::AlbaniaValidator;
    use crate::validator::CountryValidator;

    #[test]
    fn al_validation_requires_len_10() {
        let validator = AlbaniaValidator;
        assert_eq!(false, validator.validate_id("I05199Q"));
    }

    #[test]
    fn al_validation_requires_gender() {
        let validator = AlbaniaValidator;
        assert_eq!(false, validator.validate_id("H73211672R"));
    }

    #[test]
    fn al_validation_starts_with_valid_letter() {
        let validator = AlbaniaValidator;
        assert_eq!(false, validator.validate_id("Z71211672R"));
    }

    #[test]
    fn al_validation_requires_valid_date_of_birth() {
        let validator = AlbaniaValidator;
        assert_eq!(false, validator.validate_id("H71311672R"));
    }

    #[test]
    fn al_validation_invalid_ids() {
        let validator = AlbaniaValidator;
        assert_eq!(false, validator.validate_id("H71211672A"));
        assert_eq!(false, validator.validate_id("I90201535M"));
    }

    #[test]
    fn al_validation_valid_ids() {
        let validator = AlbaniaValidator;
        assert_eq!(true, validator.validate_id("I05101999I"));
        assert_eq!(true, validator.validate_id("I90201535E"));
        assert_eq!(true, validator.validate_id("J45423004V"));
        assert_eq!(true, validator.validate_id("H71211672R"));
        assert_eq!(true, validator.validate_id("I85413200A"));
    }

    #[test]
    fn al_extract_valid_ids() {
        let validator = AlbaniaValidator;
        let c1 = validator.extract_citizen("I05101999I").unwrap();
        assert_eq!('F', c1.gender);
        assert_eq!(1980, c1.year_of_birth);
        assert_eq!(1, c1.month_of_birth.unwrap());
        assert_eq!(1, c1.day_of_birth.unwrap());

        let c2 = validator.extract_citizen("H71211672R").unwrap();
        assert_eq!('M', c2.gender);
        assert_eq!(1977, c2.year_of_birth);
        assert_eq!(12, c2.month_of_birth.unwrap());
        assert_eq!(11, c2.day_of_birth.unwrap());
    }
}
