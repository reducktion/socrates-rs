use crate::{Citizen, validator};
use crate::country::Code;
use crate::validator::date;

pub(crate) struct FranceValidator;

/**
 French national citizen card number validation.

 This validation algorithm is based on documentation released in 22 of March of 2001.
 Link: http://resoo.org/docs/_docs/regles-numero-insee.pdf

 You can also check an english version in wikipedia: https://en.wikipedia.org/wiki/INSEE_code

 For the region codes, the source used was https://fr.wikipedia.org/wiki/Codes_g%C3%A9ographiques_de_la_France
**/
impl validator::CountryValidator for FranceValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = id.replace(" ", "");
        if standard_id.len() != 15 {
            return false;
        }

        for char in standard_id.chars() {
            if !char.is_digit(10) { return false }
        }

        let control_digit = standard_id.get(13..).unwrap().parse::<u64>().unwrap();
        let partial_id = standard_id.get(0..13).unwrap().parse::<u64>().unwrap();

        return control_digit == 97 - (partial_id % 97);
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::FR;
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self::FranceValidator::validate_id(&self, id) {
            return None;
        }

        let region = get_region_of_birth(&id[5..7]);
        return Some(Citizen {
            gender: if String::from(&id[0..1]).parse::<u8>().unwrap() == 1_u8 { 'M' } else { 'F' },
            year_of_birth: date::get_year_of_birth(&id[1..3]),
            month_of_birth: get_month_of_birth(&id[3..5]),
            day_of_birth: None,
            place_of_birth: if region.is_some() { region } else { get_region_of_birth(&id[5..8]) },
        });
    }
}

fn get_month_of_birth(code: &str) -> Option<u8> {
    let month = code.parse::<u8>().unwrap();
    return if month > 1_u8 && month < 13_u8 {
        Some(month)
    } else if month > 30_u8 && month < 43_u8 {
        Some(month - 30_u8)
    } else {
        None
    }
}

fn get_region_of_birth(code: &str) -> Option<String> {
    let region = match code {
        "1" => "Ain",
        "2" => "Aisne",
        "3" => "Allier",
        "4" => "Alpes-de-Haute-Provence",
        "5" => "Hautes-Alpes",
        "6" => "Alpes-Maritimes",
        "7" => "Ardèche",
        "8" => "Ardennes",
        "9" => "Ariège",
        "10" => "Aube",
        "11" => "Aude",
        "12" => "Aveyron",
        "13" => "Bouches-du-Rhône",
        "14" => "Calvados",
        "15" => "Cantal",
        "16" => "Charente",
        "17" => "Charente-Maritime",
        "18" => "Cher",
        "19" => "Corrèze",
        "21" => "Côte-d’Or",
        "22" => "Côtes-d’Armor",
        "23" => "Creuse",
        "24" => "Dordogne",
        "25" => "Doubs",
        "26" => "Drôme",
        "27" => "Eure",
        "28" => "Eure-et-Loir",
        "29" => "Finistère",
        "30" => "Gard",
        "31" => "Haute-Garonne",
        "32" => "Gers",
        "33" => "Gironde",
        "34" => "Hérault",
        "35" => "Ille-et-Vilaine",
        "36" => "Indre",
        "37" => "Indre-et-Loire",
        "38" => "Isère",
        "39" => "Jura",
        "40" => "Landes",
        "41" => "Loir-et-Cher",
        "42" => "Loire",
        "43" => "Haute-Loire",
        "44" => "Loire-Atlantique",
        "45" => "Loiret",
        "46" => "Lot",
        "47" => "Lot-et-Garonne",
        "48" => "Lozère",
        "49" => "Maine-et-Loire",
        "50" => "Manche",
        "51" => "Marne",
        "52" => "Haute-Marne",
        "53" => "Mayenne",
        "54" => "Meurthe-et-Moselle",
        "55" => "Meuse",
        "56" => "Morbihan",
        "57" => "Moselle",
        "58" => "Nièvre",
        "59" => "Nord",
        "60" => "Oise",
        "61" => "Orne",
        "62" => "Pas-de-Calais",
        "63" => "Puy-de-Dôme",
        "64" => "Pyrénées-Atlantiques",
        "65" => "Hautes-Pyrénées",
        "66" => "Pyrénées-Orientales",
        "67" => "Bas-Rhin",
        "68" => "Haut-Rhin",
        "69" => "Circonscription départementale du Rhône",
        "70" => "Haute-Saône",
        "71" => "Saône-et-Loire",
        "72" => "Sarthe",
        "73" => "Savoie",
        "74" => "Haute-Savoie",
        "75" => "Paris",
        "76" => "Seine-Maritime",
        "77" => "Seine-et-Marne",
        "78" => "Yvelines",
        "79" => "Deux-Sèvres",
        "80" => "Somme",
        "81" => "Tarn",
        "82" => "Tarn-et-Garonne",
        "83" => "Var",
        "84" => "Vaucluse",
        "85" => "Vendée",
        "86" => "Vienne",
        "87" => "Haute-Vienne",
        "88" => "Vosges",
        "89" => "Yonne",
        "90" => "Territoire de Belfort",
        "91" => "Essonne",
        "92" => "Hauts-de-Seine",
        "93" => "Seine-Saint-Denis",
        "94" => "Val-de-Marne",
        "95" => "Val-d’Oise",
        "971" => "Guadeloupe",
        "972" => "Martinique",
        "973" => "Guyane (française)",
        "974" => "La Réunion",
        "975" => "Saint-Pierre-et-Miquelon (voir aussi ISO 3166-1:PM)",
        "976" => "Mayotte",
        "977" => "Saint-Barthélemy (voir aussi ISO 3166-1:BL)",
        "978" => "Saint-Martin (voir aussi ISO 3166-1:MF)",
        "984" => "Terres australes et antarctiques françaises (voir aussi ISO 3166-1:TF)",
        "986" => "Wallis-et-Futuna (voir aussi ISO 3166-1:WF)",
        "987" => "Polynésie française (voir aussi ISO 3166-1:PF)",
        "988" => "Nouvelle-Calédonie (voir aussi ISO 3166-1:NC)",
        "989" => "Île de Clipperton (voir aussi ISO 3166-1:CP)",
        _ => ""
    };

    return if region.is_empty() { None } else { Some(String::from(region)) }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;
    use crate::validator::france::get_region_of_birth;

    #[test]
    fn fr_validator_requires_min_len_of_15() {
        let validator = super::validator::france::FranceValidator;
        assert_eq!(false, validator.validate_id("123"));
        assert_eq!(false, validator.validate_id("123 456 789 0"));
    }

    #[test]
    fn fr_validator_invalid_ids() {
        let validator = super::validator::france::FranceValidator;
        assert_eq!(validator.validate_id("2312760989812 01"), false);
        assert_eq!(validator.validate_id("2312763214568 54"), false);
    }

    #[test]
    fn fr_validator_valid_ids() {
        let validator = super::validator::france::FranceValidator;
        assert_eq!(validator.validate_id("2820819398814 09"), true);
        assert_eq!(validator.validate_id("1350455179061 16"), true);
        assert_eq!(validator.validate_id("2381080214568 11"), true);
        assert_eq!(validator.validate_id("1880858704571 57"), true);
        assert_eq!(validator.validate_id("1820897401154 75"), true);
    }

    #[test]
    fn fr_extractor_returns_none_for_invalid_id() {
        let validator = super::validator::france::FranceValidator;
        assert_eq!(validator.extract_citizen("2312760989812 01").is_none(), true);
    }

    #[test]
    fn fr_extractor_returns_citizen_for_valid_ids() {
        let validator = super::validator::france::FranceValidator;

        let citizen_annette = validator.extract_citizen("2820819398814 09").unwrap();
        assert_eq!(citizen_annette.gender, 'F');
        assert_eq!(citizen_annette.year_of_birth, 1982);
        assert_eq!(citizen_annette.month_of_birth.unwrap(), 8);
        assert_eq!(citizen_annette.place_of_birth.unwrap(), "Corrèze");

        let citizen_lothair = validator.extract_citizen("1880858704571 57").unwrap();
        assert_eq!(citizen_lothair.gender, 'M');
        assert_eq!(citizen_lothair.year_of_birth, 1988);
        assert_eq!(citizen_lothair.month_of_birth.unwrap(), 8);
        assert_eq!(citizen_lothair.place_of_birth.unwrap(), "Nièvre");
    }

    #[test]
    fn fr_get_region_code() {
        let unknown_region = get_region_of_birth("999");
        let known_region = get_region_of_birth("1");
        assert_eq!(unknown_region.is_none(), true);
        assert_eq!(known_region.is_some(), true);
        assert_eq!(known_region.unwrap(), "Ain");
    }
}