use std::convert::TryFrom;

use super::to_query_string::ToQueryString;

/// Value objects for field-name.
///
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Default)]
pub struct FieldName {
    field_name_string: String,
}

// Constructs a value object.
impl TryFrom<String> for FieldName {
    type Error = anyhow::Error;

    // TODO Error
    fn try_from(field_name_string: String) -> anyhow::Result<Self> {
        Ok(Self { field_name_string })
    }
}

impl FieldName {
    pub fn of<T: Into<String>>(field_name_string: T) -> anyhow::Result<Self> {
        FieldName::try_from(field_name_string.into())
    }
}

/// FieldName to String conversion process
///
impl From<FieldName> for String {
    fn from(field_name: FieldName) -> Self {
        field_name.field_name_string
    }
}

/// When converting to string, add backquote
impl ToQueryString for FieldName {
    fn to_query_string(&self) -> String {
        format!("`{}`", self.field_name_string)
    }
}
