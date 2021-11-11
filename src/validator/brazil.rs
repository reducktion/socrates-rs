use crate::country::Code;
use crate::{validator, Citizen};

pub(crate) struct BrazilValidator;

/**
 Brazil national id validation.
 Link: https://pt.wikipedia.org/wiki/Cadastro_de_pessoas_f%C3%ADsicas#Algoritmo
**/
impl validator::CountryValidator for BrazilValidator {
    fn validate_id(&self, id: &str) -> bool {
        let standard_id = self.sanitize_id(id);
        if standard_id.len() != 11 || !standard_id.chars().all(char::is_numeric) {
            return false;
        }

        let cpf = standard_id[0..(standard_id.len() - 2)].chars().rev();
        let mut v1 = 0;
        let mut v2 = 0;
        for (idx, c) in cpf.enumerate() {
            v1 = v1 + c.to_digit(32).unwrap() * (9 - (idx % 10)) as u32;
            v2 = v2 + c.to_digit(32).unwrap() * (9 - ((idx + 1) % 10)) as u32;
        }

        v1 = (v1 % 11) % 10;
        v2 = v2 + (v1 * 9);
        v2 = (v2 % 11) % 10;

        return standard_id[(standard_id.len() - 2)..]
            .parse::<u32>()
            .unwrap()
            == ((v1 * 10) + v2);
    }

    fn country_code(&self) -> Code {
        return crate::country::Code::BR;
    }

    fn extract_citizen(&self, _id: &str) -> Option<Citizen> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::CountryValidator;

    #[test]
    fn br_validator() {
        let validator = super::validator::brazil::BrazilValidator;
        assert_eq!(validator.validate_id("1234567ACAB"), false); // not numeric
        assert_eq!(validator.validate_id("123456789012"), false); // more than 11 digits
        assert_eq!(validator.validate_id("1234567890"), false); // less than 11 digits
        assert_eq!(validator.validate_id("23294954040"), false); // bad checksum

        assert_eq!(validator.validate_id("144-416-762.63"), true);
        assert_eq!(validator.validate_id(" 62363568400 "), true);
        assert_eq!(validator.validate_id("92205820230"), true);
        assert_eq!(validator.validate_id("88958056231"), true);
        assert_eq!(validator.validate_id("90701066555"), true);
        assert_eq!(validator.validate_id("31098035348"), true);
        assert_eq!(validator.validate_id("54271183148"), true);
        assert_eq!(validator.validate_id("03860881795"), true);
        assert_eq!(validator.validate_id("15777379117"), true);
        assert_eq!(validator.validate_id("46959616360"), true);
        assert_eq!(validator.validate_id("51861041675"), true);
        assert_eq!(validator.validate_id("35823686102"), true);
        assert_eq!(validator.validate_id("26319324120"), true);
        assert_eq!(validator.validate_id("81036850463"), true);
        assert_eq!(validator.validate_id("17188856443"), true);
        assert_eq!(validator.validate_id("16556182451"), true);
        assert_eq!(validator.validate_id("13369586347"), true);
        assert_eq!(validator.validate_id("19319810940"), true);
        assert_eq!(validator.validate_id("41120495792"), true);
        assert_eq!(validator.validate_id("79950524482"), true);
        assert_eq!(validator.validate_id("44667914068"), true);
        assert_eq!(validator.validate_id("41947527240"), true);
        assert_eq!(validator.validate_id("23554835234"), true);
        assert_eq!(validator.validate_id("04008125922"), true);
        assert_eq!(validator.validate_id("37025634581"), true);
        assert_eq!(validator.validate_id("26363102820"), true);
        assert_eq!(validator.validate_id("17758534112"), true);
    }

    #[test]
    fn br_extractor() {
        let validator = super::validator::brazil::BrazilValidator;
        assert_eq!(validator.extract_citizen("17758534112").is_none(), true);
    }
}
