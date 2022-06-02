use super::field_types::FieldType;

pub struct Query {
    pub table_name: String,

    pub format: String,

    pub field: Vec<Field>,
}

pub struct Field {
    pub field_name: String,

    pub field_type: FieldType,

    pub is_not_null: bool,

    pub is_primary: bool,
}
