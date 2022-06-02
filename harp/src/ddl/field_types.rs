pub enum FieldType {
    NumberField(NumberField),
    StringField(StringField),
    DateField(DateField),
}

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

pub enum StringField {
    Char,
    Varchar,
    Binary,
    VarBinary,
    Blob,
    Text,
}

pub enum DateField {
    Date,
    Time,
    DateTime,
    TimeStamp,
    Year,
}
