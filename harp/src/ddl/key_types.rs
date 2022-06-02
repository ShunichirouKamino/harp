use strum::EnumString;

#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
pub enum KeyTypes {
    #[strum(serialize = "PK")]
    PK,
    #[strum(serialize = "FK")]
    FK,
}
