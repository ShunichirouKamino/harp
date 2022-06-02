use strum::EnumString;

#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
pub enum FieldType {
    // Number types
    #[strum(serialize = "bit")]
    Bit,
    #[strum(serialize = "tinyiny")]
    Tinyiny,
    #[strum(serialize = "bool")]
    Bool,
    #[strum(serialize = "smallint")]
    SmallInt,
    #[strum(serialize = "mediumint")]
    MediumInt,
    #[strum(serialize = "integer")]
    Integer,
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "bigint")]
    BigInt,
    #[strum(serialize = "decimal")]
    Decimal,
    #[strum(serialize = "float")]
    Float,
    #[strum(serialize = "double")]
    Double,

    // String types
    #[strum(serialize = "char")]
    Char(u64),
    #[strum(serialize = "varchar")]
    Varchar(u64),
    #[strum(serialize = "binary")]
    Binary(u64),
    #[strum(serialize = "varbinary")]
    VarBinary(u64),
    #[strum(serialize = "blob")]
    Blob,
    #[strum(serialize = "text")]
    Text,

    // Date types
    #[strum(serialize = "date")]
    Date,
    #[strum(serialize = "time")]
    Time,
    #[strum(serialize = "datetime")]
    DateTime,
    #[strum(serialize = "timestamp")]
    TimeStamp,
    #[strum(serialize = "year")]
    Year,

    // No match
    Nothing,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Nothing
    }
}
