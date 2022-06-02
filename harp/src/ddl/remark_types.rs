use strum::EnumString;

#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
pub enum RemarkType {
    #[strum(serialize = r#""not_null""#)]
    NotNull,
    #[strum(serialize = r#""default_null""#)]
    DafaultNull,
    #[strum(serialize = r#""default_current_timestamp""#)]
    DefaultCurrentTimestamp,

    // No match
    Nothing,
}

impl Default for RemarkType {
    fn default() -> Self {
        RemarkType::Nothing
    }
}
