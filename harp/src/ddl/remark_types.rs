use std::fmt::Display;

use strum::{AsRefStr, EnumString};

#[derive(Clone, PartialEq, Eq, Debug, EnumString, AsRefStr)]
pub enum RemarkType {
    #[strum(serialize = "not_null")]
    NotNull,
    #[strum(serialize = "default_null")]
    DafaultNull,
    #[strum(serialize = "default_current_timestamp")]
    DefaultCurrentTimestamp,

    // No match
    Nothing,
}

impl Default for RemarkType {
    fn default() -> Self {
        RemarkType::Nothing
    }
}

impl Display for RemarkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RemarkType::NotNull => write!(f, "NOT NULL"),
            RemarkType::DafaultNull => write!(f, "DEFAULT NULL"),
            RemarkType::DefaultCurrentTimestamp => write!(f, "DEFAULT CURRENT_TIMESTAMP"),
            RemarkType::Nothing => todo!(),
        }
    }
}
