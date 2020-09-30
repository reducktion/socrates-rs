#![feature(assoc_char_funcs)]

pub mod country;
mod validator;

pub struct Citizen {
    gender: char,
    year_of_birth: i32,
    month_of_birth: Option<u8>,
    day_of_birth: Option<u8>,
    place_of_birth: Option<String>,
}

pub fn validate_id(id: &str, country: country::Code) -> bool {
    let country_validator = validator::get_validator(country);
    return country_validator.validate_id(id);
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
