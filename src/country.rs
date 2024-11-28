use strum_macros::EnumIter;
/**
List of ISO country codes: https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
**/
#[derive(EnumIter)]
pub enum Code {
    BE,
    BR,
    CA,
    DK,
    ES,
    FR,
    IT,
    LU,
    PT,
    US,
    DE
}
