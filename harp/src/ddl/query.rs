use super::{
    domain::{field_name::FieldName, field_size::FieldSize, query_start::QueryStart},
    field_types::FieldType,
    key_types::KeyType,
    remark_types::RemarkType,
};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Query {
    table_name: String,

    query_start: QueryStart,

    field: Vec<Field>,
}

impl Query {
    pub fn table_name_mut(&mut self) -> &mut String {
        &mut self.table_name
    }

    pub fn query_start_mut(&mut self) -> &mut QueryStart {
        &mut self.query_start
    }

    pub fn field_mut(&mut self) -> &mut Vec<Field> {
        &mut self.field
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Field {
    field_name: FieldName,

    field_type: FieldType,

    field_size: Option<FieldSize>, // 0 ~ 65535

    is_not_null: bool,

    default_value: Option<RemarkType>,

    key_type: Option<KeyType>,
}

impl Field {
    pub fn field_name(&mut self) -> &mut FieldName {
        &mut self.field_name
    }

    pub fn field_type(&mut self) -> &mut FieldType {
        &mut self.field_type
    }

    pub fn field_size(&mut self) -> &mut Option<FieldSize> {
        &mut self.field_size
    }

    pub fn is_not_null(&mut self) -> &mut bool {
        &mut self.is_not_null
    }

    pub fn default_value(&mut self) -> &mut Option<RemarkType> {
        &mut self.default_value
    }

    pub fn key_type(&mut self) -> &mut Option<KeyType> {
        &mut self.key_type
    }
}
