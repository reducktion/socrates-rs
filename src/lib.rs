pub mod country;
mod generator;
mod validator;

pub struct Citizen {
    pub gender: char,
    pub year_of_birth: i32,
    pub month_of_birth: Option<u8>,
    pub day_of_birth: Option<u8>,
    pub place_of_birth: Option<String>,
}

pub fn validate_id(id: &str, country: country::Code) -> bool {
    let country_validator = validator::get_validator(&country);
    return country_validator.validate_id(id);
}

pub fn extract_information(id: &str, country: country::Code) -> Option<Citizen> {
    let country_validator = validator::get_validator(&country);
    return country_validator.extract_citizen(id);
}

pub fn generate_id(citizen: &Citizen, country: country::Code) -> Option<String> {
    let generator = generator::get_generator(country);
    if generator.is_none() {
        return None;
    }

    return Some(generator.unwrap().generate(citizen));
}
