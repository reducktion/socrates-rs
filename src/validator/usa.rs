use crate::country::Code;
use crate::{validator, Citizen};

pub(crate) struct UsaValidator;

/**
 United States of America Social Security Number code validation.

 The validation algorithm is based on the Social Security website documents
  which can be found online in https://www.ssa.gov/employer/randomization.html

 The blacklisted SSN can be traced back to https://www.ssa.gov/history/ssn/misused.html

 Another (english) version can be viewed in wikipedia: https://en.wikipedia.org/wiki/Social_Security_number#Valid_SSNs

 The list of regions can be checked in https://www.ssa.gov/employer/stateweb.htm
**/
impl validator::CountryValidator for UsaValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);

        if standard_id.len() != 9 {
            return false;
        }

        if &standard_id == "078051120" || &standard_id == "219099999" || &standard_id == "457555462"
        {
            return false;
        }

        let area_code = &standard_id[0..3].parse::<u16>().unwrap();
        if area_code == &0 || area_code == &666 || area_code > &899 {
            return false;
        }

        if &standard_id[3..5].parse::<u8>().unwrap() == &0 {
            return false;
        }

        if &standard_id[5..].parse::<u16>().unwrap() == &0 {
            return false;
        }

        return true;
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::US;
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn us_validator_requires_len_of_9() {
        let validator = super::validator::usa::UsaValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123-456-7"));
    }

    #[test]
    fn us_validator_invalid_ids() {
        let validator = super::validator::usa::UsaValidator;
        assert_eq!(validator.validate_id("078-05-1120"), false);
        assert_eq!(validator.validate_id("219-09-9999"), false);
        assert_eq!(validator.validate_id("457-55-5462"), false);
        assert_eq!(validator.validate_id("666-91-8271"), false);
    }

    #[test]
    fn us_validator_valid_ids() {
        let validator = super::validator::usa::UsaValidator;
        assert_eq!(validator.validate_id("167-38-1265"), true);
        assert_eq!(validator.validate_id("536-22-8726"), true);
        assert_eq!(validator.validate_id("536-22-5232"), true);
        assert_eq!(validator.validate_id("574-22-7664"), true);
        assert_eq!(validator.validate_id("671-26-9121"), true);
    }
}
