use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Region {
    code: String,
    region: String,
}

pub fn get_region_from_csv(code: &str, file_path: &str) -> Option<String> {
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Region = result.unwrap();
        if record.code == code.to_owned() {
            return Some(record.region);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::validator::regions::get_region_from_csv;

    #[test]
    fn year_of_birth() {
        assert_eq!(
            "ALBANIA",
            get_region_from_csv("Z100", "./src/validator/regions/italy_regions.csv").unwrap()
        );
    }
}
