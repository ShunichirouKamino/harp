use std::fmt;

use strum::{AsRefStr, EnumString};

#[derive(Clone, PartialEq, Eq, Debug, EnumString, AsRefStr)]
pub enum KeyType {
    #[strum(serialize = "PK")]
    PK,
    #[strum(serialize = "FK")]
    FK,

    // No match
    Nothing,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::Nothing
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KeyType::PK => write!(f, "PRIMARY KEY"),
            KeyType::FK => todo!(),
            KeyType::Nothing => todo!(),
        }
    }
}
