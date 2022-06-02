use strum::EnumString;

#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
pub enum KeyType {
    #[strum(serialize = "PK")]
    PK,
    #[strum(serialize = "FK")]
    FK,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::PK
    }
}
