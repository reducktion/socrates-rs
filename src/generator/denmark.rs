use crate::country::Code;
use crate::{generator::CountryIdGenerator, Citizen};

pub(crate) struct DenmarkGenerator;

const MULTIPLIER: [u32; 10] = [4, 3, 2, 7, 6, 5, 4, 3, 2, 1];

/**
 * National Id for Denmark.
 *
 * Information about this national id can be found at:
 *  - https://en.wikipedia.org/wiki/Personal_identification_number_(Denmark)
 */
impl CountryIdGenerator for DenmarkGenerator {
    fn generate(&self, citizen: &Citizen) -> String {
        if !is_required_data_present(citizen) {
            panic!("Gender and valid date of birth are required to generate an id");
        }

        let dob = format!(
            "{:02}{:02}{:02}",
            citizen.day_of_birth.unwrap(),
            citizen.month_of_birth.unwrap(),
            &citizen.year_of_birth.to_string()[2..]
        );

        let century = match citizen.year_of_birth {
            0..=1998 => "3".to_string(),
            1999..=2035 => "4".to_string(),
            _ => "5".to_string(),
        };

        let check_digit = if citizen.gender == 'F' {
            "2".to_string()
        } else {
            "3".to_string()
        };
        let sum = calculate_checksum(&format!("{}{}00{}", dob, century, check_digit));
        let ceiling = ((sum as f32) / 11.0).ceil();
        let remainder = (ceiling * 11.0) as u32 - sum;
        let generated_digits = calculate_final_digits(remainder);

        return format!("{}-{}{}{}", dob, century, generated_digits, check_digit);
    }

    fn country_code(&self) -> Code {
        Code::DK
    }
}

fn is_required_data_present(citizen: &Citizen) -> bool {
    (citizen.gender == 'M' || citizen.gender == 'F')
        && citizen.year_of_birth > 1800
        && citizen.month_of_birth.is_some()
        && citizen.day_of_birth.is_some()
}

fn calculate_checksum(id: &str) -> u32 {
    let mut sum: u32 = 0;
    for (idx, digit) in id.chars().enumerate() {
        sum += digit.to_digit(36).unwrap() * MULTIPLIER[idx];
    }
    return sum;
}

fn calculate_final_digits(target_sum: u32) -> String {
    if target_sum / MULTIPLIER[7] < 1 && target_sum / MULTIPLIER[8] < 1 {
        return calculate_final_digits(target_sum + 11);
    }

    if target_sum % MULTIPLIER[7] == 0 {
        return format!("{}0", target_sum / MULTIPLIER[7]);
    }

    if target_sum % MULTIPLIER[8] == 0 {
        return format!("0{}", target_sum / MULTIPLIER[8]);
    }

    for i in 1..10 {
        for j in 1..10 {
            if target_sum == MULTIPLIER[7] * i + MULTIPLIER[8] * j {
                return format!("{}{}", i, j);
            }
        }
    }

    panic!("Could not generate a valid cpr for this data. Please open an issue in https://github.com/reducktion/socrates-rs/issues");
}

#[cfg(test)]
mod tests {
    use crate::generator::denmark::DenmarkGenerator;
    use crate::generator::CountryIdGenerator;
    use crate::Citizen;

    #[test]
    fn generate_citizen() {
        let generator = DenmarkGenerator;
        assert_eq!(
            "160691-3113",
            generator.generate(&Citizen {
                gender: 'M',
                year_of_birth: 1991,
                month_of_birth: Some(6),
                day_of_birth: Some(16),
                place_of_birth: None
            })
        );

        assert_eq!(
            "081084-3012",
            generator.generate(&Citizen {
                gender: 'F',
                year_of_birth: 1984,
                month_of_birth: Some(10),
                day_of_birth: Some(8),
                place_of_birth: None
            })
        )
    }
}
