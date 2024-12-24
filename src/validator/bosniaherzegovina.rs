use crate::country::Code;
use crate::validator::date::get_year_of_birth;
use crate::validator::CountryValidator;
use crate::Citizen;
use chrono::{Datelike, NaiveDate};
use regex::Regex;

pub(crate) struct BosniaHerzegovinaValidator;

/**
* TODO: Find official source
*/
impl CountryValidator for BosniaHerzegovinaValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if !Regex::new(r"^\d{13}$").unwrap().is_match(&standard_id) {
            return false;
        }

        let region = standard_id[7..9].parse::<u32>().unwrap();
        if (region < 10 || region > 19) && region != 1 {
            return false;
        }

        let check_sum = standard_id[12..13].parse::<u32>().unwrap();
        let mut sum = (7
            * (standard_id[0..1].parse::<u32>().unwrap()
                + standard_id[6..7].parse::<u32>().unwrap()))
            + (6 * (standard_id[1..2].parse::<u32>().unwrap()
                + standard_id[7..8].parse::<u32>().unwrap()))
            + (5 * (standard_id[2..3].parse::<u32>().unwrap()
                + standard_id[8..9].parse::<u32>().unwrap()))
            + (4 * (standard_id[3..4].parse::<u32>().unwrap()
                + standard_id[9..10].parse::<u32>().unwrap()))
            + (3 * (standard_id[4..5].parse::<u32>().unwrap()
                + standard_id[10..11].parse::<u32>().unwrap()))
            + (2 * (standard_id[5..6].parse::<u32>().unwrap()
                + standard_id[11..12].parse::<u32>().unwrap()));

        sum = 11 - (sum % 11);
        if sum == 10 || sum == 11 {
            sum = 0
        }

        sum == check_sum
    }

    fn country_code(&self) -> Code {
        Code::BA
    }

    fn extract_citizen(&self, id: &str) -> Option<Citizen> {
        if !self.validate_id(id) {
            return None;
        }
        let standard_id = self.sanitize_id(id);
        let g;
        if standard_id[9..12].parse::<u32>().unwrap() < 500 {
            g = 'M';
        } else {
            g = 'F';
        }
        let year_of_birth = get_year_of_birth(&standard_id[5..7]);
        let date_of_birth = NaiveDate::from_ymd_opt(
            year_of_birth,
            standard_id[2..4].parse::<u32>().unwrap(),
            standard_id[0..2].parse::<u32>().unwrap(),
        )
        .unwrap();

        let region = match &standard_id[7..9] {
            "01" => Some("foreigner in Bosnia and Herzegovina".to_string()),
            "02" => Some("foreigners in Montenegro".to_string()),
            "03" => Some("foreigners in Croatia".to_string()),
            "04" => Some("foreigners in North Macedonia".to_string()),
            "05" => Some("foreigners in Slovenia".to_string()),
            "06" => Some("foreigners in Central Serbia".to_string()),
            "07" => Some("foreigners in Serbian province of Vojvodina".to_string()),
            "08" => Some("foreigners in Kosovo".to_string()),
            "09" => Some("naturalized citizen with no republican citizenship".to_string()),
            "10" => Some("Banja Luka - Bosnia and Herzegovina".to_string()),
            "11" => Some("Bihać - Bosnia and Herzegovina".to_string()),
            "12" => Some("Doboj - Bosnia and Herzegovina".to_string()),
            "13" => Some("Goražde - Bosnia and Herzegovina".to_string()),
            "14" => Some("Livno - Bosnia and Herzegovina".to_string()),
            "15" => Some("Mostar - Bosnia and Herzegovina".to_string()),
            "16" => Some("Prijedor - Bosnia and Herzegovina".to_string()),
            "17" => Some("Sarajevo - Bosnia and Herzegovina".to_string()),
            "18" => Some("Tuzla - Bosnia and Herzegovina".to_string()),
            "19" => Some("Zenica - Bosnia and Herzegovina".to_string()),
            "20" => Some("Montenegro".to_string()),
            "21" => Some("Podgorica, Danilovgrad, Kolašin - Montenegro".to_string()),
            "22" => Some("Bar, Ulcinj - Montenegro".to_string()),
            "23" => Some("Budva, Kotor, Tivat - Montenegro".to_string()),
            "24" => Some("Herceg Novi - Montenegro".to_string()),
            "25" => Some("Cetinje - Montenegro".to_string()),
            "26" => Some("Nikšić, Plužine, Šavnik - Montenegro".to_string()),
            "27" => Some("Berane, Rožaje, Plav, Andrijevica - Montenegro".to_string()),
            "28" => Some("Bijelo Polje, Mojkovac - Montenegro".to_string()),
            "29" => Some("Pljevlja, Žabljak - Montenegro".to_string()),
            "30" => Some("Osijek, Slavonia region - Croatia".to_string()),
            "31" => Some("Bjelovar, Virovitica, Koprivnica, Pakrac, Podravina region - Croatia".to_string()),
            "32" => Some("Varaždin, Međimurje region - Croatia".to_string()),
            "33" => Some("Zagreb - Croatia".to_string()),
            "34" => Some("Karlovac, Kordun region - Croatia".to_string()),
            "35" => Some("Gospić, Lika region - Croatia".to_string()),
            "36" => Some("Rijeka, Pula, Gorski kotar, Istria and Croatian Littoral regions - Croatia".to_string()),
            "37" => Some("Sisak, Banovina region - Croatia".to_string()),
            "38" => Some("Split, Zadar, Šibenik, Dubrovnik, Dalmatia region - Croatia".to_string()),
            "39" => Some("Hrvatsko Zagorje and mixed - Croatia".to_string()),
            "40" => Some("North Macedonia".to_string()),
            "41" => Some("Bitola - North Macedonia".to_string()),
            "42" => Some("Kumanovo - North Macedonia".to_string()),
            "43" => Some("Ohrid - North Macedonia".to_string()),
            "44" => Some("Prilep - North Macedonia".to_string()),
            "45" => Some("Skopje - North Macedonia".to_string()),
            "46" => Some("Strumica - North Macedonia".to_string()),
            "47" => Some("Tetovo - North Macedonia".to_string()),
            "48" => Some("Veles - North Macedonia".to_string()),
            "49" => Some("Štip - North Macedonia".to_string()),
            "50" => Some("Slovenia".to_string()),
            "60" => Some("Temporary residence".to_string()),
            "61" => Some("Temporary residence".to_string()),
            "62" => Some("Temporary residence".to_string()),
            "63" => Some("Temporary residence".to_string()),
            "64" => Some("Temporary residence".to_string()),
            "65" => Some("Temporary residence".to_string()),
            "66" => Some("Temporary residence".to_string()),
            "67" => Some("Temporary residence".to_string()),
            "68" => Some("Temporary residence".to_string()),
            "69" => Some("Temporary residence".to_string()),
            "70" => Some("Serbian citizens registered abroad at a Serbian diplomatic/consular post - Serbia".to_string()),
            "71" => Some("Belgrade region (City of Belgrade) - Central Serbia".to_string()),
            "72" => Some("Šumadija and Pomoravlje regions (Šumadija District and Pomoravlje District) - Central Serbia".to_string()),
            "73" => Some("Niš region (Nišava District, Pirot District and Toplica District) - Central Serbia".to_string()),
            "74" => Some("Southern Morava region (Jablanica District and Pčinja District) - Central Serbia".to_string()),
            "75" => Some("Zaječar region (Zaječar District and Bor District) - Central Serbia".to_string()),
            "76" => Some("Podunavlje region (Podunavlje District and Braničevo District) - Central Serbia".to_string()),
            "77" => Some("Podrinje and Kolubara regions (Mačva District and Kolubara District) - Central Serbia".to_string()),
            "78" => Some("Kraljevo region (Raška District, Moravica District and Rasina District) - Central Serbia".to_string()),
            "79" => Some("Užice region (Zlatibor District) - Central Serbia".to_string()),
            "80" => Some("Novi Sad region (South Bačka District) - Serbian province of Vojvodina".to_string()),
            "81" => Some("Sombor region (West Bačka District) - Serbian province of Vojvodina".to_string()),
            "82" => Some("Subotica region (North Bačka District) - Serbian province of Vojvodina".to_string()),
            "84" => Some("Kikinda region (North Banat District) - Serbian province of Vojvodina".to_string()),
            "85" => Some("Zrenjanin region (Central Banat District) - Serbian province of Vojvodina".to_string()),
            "86" => Some("Pančevo region (South Banat District) - Serbian province of Vojvodina".to_string()),
            "87" => Some("Vršac region (South Banat District) - Serbian province of Vojvodina".to_string()),
            "88" => Some("Ruma region (part of Syrmia District) - Serbian province of Vojvodina".to_string()),
            "89" => Some("Sremska Mitrovica region (part of Syrmia District) - Serbian province of Vojvodina".to_string()),
            "90" => Some("Kosovo".to_string()),
            "91" => Some("Priština region (Kosovo District) - Kosovo".to_string()),
            "92" => Some("Kosovska Mitrovica region (Kosovska Mitrovica District) - Kosovo".to_string()),
            "93" => Some("Peć region (part of Peć District) - Kosovo".to_string()),
            "94" => Some("Đakovica region (part of Peć District) - Kosovo".to_string()),
            "95" => Some("Prizren region (Prizren District) - Kosovo".to_string()),
            "96" => Some("Gnjilane region (Kosovo-Pomoravlje District) - Kosovo".to_string()),
            "97" => Some("Kosovo".to_string()),
            "98" => Some("Kosovo".to_string()),
            "99" => Some("Kosovo".to_string()),
            _ => None
        };

        Some(Citizen {
            gender: g,
            year_of_birth: date_of_birth.year(),
            month_of_birth: Some(date_of_birth.month() as u8),
            day_of_birth: Some(date_of_birth.day() as u8),
            place_of_birth: region,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::bosniaherzegovina::BosniaHerzegovinaValidator;
    use crate::validator::CountryValidator;

    #[test]
    fn ba_validator_requires_len_13() {
        let validator = BosniaHerzegovinaValidator {};
        assert_eq!(false, validator.validate_id("123456789"));
        assert_eq!(false, validator.validate_id("12345678910111213"));
    }

    #[test]
    fn ba_validator_requires_valid_region() {
        let validator = BosniaHerzegovinaValidator {};
        assert_eq!(false, validator.validate_id("2808928401264"));
        assert_eq!(false, validator.validate_id("2007950274591"));
    }

    #[test]
    fn ba_validator_invalid_ids() {
        let validator = BosniaHerzegovinaValidator {};
        assert_eq!(false, validator.validate_id("1108291065212"));
        assert_eq!(false, validator.validate_id("2801826817261"));
        assert_eq!(false, validator.validate_id("1012999121239"));
    }

    #[test]
    fn ba_validator_valid_ids() {
        let validator = BosniaHerzegovinaValidator {};
        assert_eq!(true, validator.validate_id("1502957172694"));
        assert_eq!(true, validator.validate_id("2508995191483"));
        assert_eq!(true, validator.validate_id("1012980163603"));
        assert_eq!(true, validator.validate_id("1310963145538"));
        assert_eq!(true, validator.validate_id("1806998154160"));
    }

    #[test]
    fn ba_extractor_valid_citizens() {
        let validator = BosniaHerzegovinaValidator {};
        let c1 = validator.extract_citizen("1310963145538").unwrap();
        assert_eq!('F', c1.gender);
        assert_eq!(1963, c1.year_of_birth);
        assert_eq!(10, c1.month_of_birth.unwrap());
        assert_eq!(13, c1.day_of_birth.unwrap());
        assert_eq!("Livno - Bosnia and Herzegovina", c1.place_of_birth.unwrap());

        let c2 = validator.extract_citizen("1806998154160").unwrap();
        assert_eq!('M', c2.gender);
        assert_eq!(1998, c2.year_of_birth);
        assert_eq!(6, c2.month_of_birth.unwrap());
        assert_eq!(18, c2.day_of_birth.unwrap());
        assert_eq!(
            "Mostar - Bosnia and Herzegovina",
            c2.place_of_birth.unwrap()
        );
    }
}
