use super::field_types::FieldType;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Query {
    table_name: String,

    format: String,

    field: Vec<Field>,
}

impl Query {
    pub fn table_name_mut(&mut self) -> &mut String {
        &mut self.table_name
    }

    pub fn set_format_mut(&mut self) -> &mut String {
        &mut self.format
    }

    pub fn set_field_mut(&mut self) -> &mut Vec<Field> {
        &mut self.field
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Field {
    field_name: String,

    field_type: FieldType,

    field_size: Option<u16>, // 0 ~ 65535

    is_not_null: bool,

    is_primary: bool,
}

impl Field {
    pub fn field_name(&mut self) -> &mut String {
        &mut self.field_name
    }

    pub fn field_type(&mut self) -> &mut FieldType {
        &mut self.field_type
    }

    pub fn field_size(&mut self) -> &mut Option<u16> {
        &mut self.field_size
    }

    pub fn is_not_null(&mut self) -> &mut bool {
        &mut self.is_not_null
    }

    pub fn is_primary(&mut self) -> &mut bool {
        &mut self.is_primary
    }
}
