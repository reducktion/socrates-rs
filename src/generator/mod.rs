mod denmark;

use crate::country::Code;
use crate::{country, Citizen};

pub trait CountryIdGenerator {
    fn generate(&self, citizen: &Citizen) -> String;
    fn country_code(&self) -> country::Code;
}

pub fn get_generator(country: country::Code) -> Option<Box<dyn CountryIdGenerator>> {
    return match country {
        Code::DK => Some(Box::new(denmark::DenmarkGenerator)),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use crate::country::Code;
    use std::mem;

    #[test]
    fn generator_selection() {
        let validator = super::get_generator(Code::DK).unwrap();
        assert_eq!(
            mem::discriminant(&Code::DK),
            mem::discriminant(&validator.country_code())
        );
    }
}
