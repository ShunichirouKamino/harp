use std::convert::TryFrom;

use super::to_query_string::ToQueryString;

/// Value objects for field-size.
///
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Default)]
pub struct FieldSize {
    field_size_number: u16,
}

// Constructs a value object.
impl TryFrom<u16> for FieldSize {
    type Error = anyhow::Error;

    // TODO Error
    fn try_from(field_size_number: u16) -> anyhow::Result<Self> {
        Ok(Self { field_size_number })
    }
}

/// FieldSize to u16 conversion process
///
impl From<FieldSize> for u16 {
    fn from(field_size: FieldSize) -> Self {
        field_size.field_size_number
    }
}

impl ToString for FieldSize {
    fn to_string(&self) -> String {
        self.field_size_number.to_string()
    }
}

/// When converting to string, add `()`
impl ToQueryString for FieldSize {
    fn to_query_string(&self) -> String {
        format!("({})", self.field_size_number)
    }
}
