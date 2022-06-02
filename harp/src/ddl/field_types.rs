#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldType {
    NumberField(NumberField),
    StringField(StringField),
    DateField(DateField),
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::StringField(StringField::Varchar)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NumberField {
    Bit,
    Tinyiny,
    Bool,
    SmallInt,
    MediumInt,
    Int,
    Integer,
    BigInt,
    Decimal,
    Float,
    Double,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StringField {
    Char,
    Varchar,
    Binary,
    VarBinary,
    Blob,
    Text,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DateField {
    Date,
    Time,
    DateTime,
    TimeStamp,
    Year,
}
