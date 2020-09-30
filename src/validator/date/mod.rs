use chrono::{Datelike, Utc};

pub fn get_year_of_birth(code: &str) -> i32 {
    let mut year_string = "20".to_owned();
    year_string.push_str(code);
    let year = year_string.parse::<i32>().unwrap();
    let current_year = Utc::now().year();

    return if year > current_year {
        year_string = "19".to_owned();
        year_string.push_str(code);
        year_string.parse::<i32>().unwrap()
    } else {
        year
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::date::get_year_of_birth;

    #[test]
    fn year_of_birth() {
        assert_eq!(2019, get_year_of_birth("19"));
        assert_eq!(1934, get_year_of_birth("34"));
    }
}