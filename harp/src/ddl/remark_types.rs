use strum::EnumString;

#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
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
