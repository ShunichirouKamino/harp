use std::convert::TryFrom;

use super::to_query_string::ToQueryString;

/// Value objects for query-start.
///
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Default)]
pub struct QueryStart {
    table_name: String,
}

// Constructs a value object.
impl TryFrom<String> for QueryStart {
    type Error = anyhow::Error;

    // TODO Error
    fn try_from(table_name: String) -> anyhow::Result<Self> {
        Ok(Self { table_name })
    }
}

impl QueryStart {
    pub fn of<T: Into<String>>(table_name: T) -> anyhow::Result<Self> {
        QueryStart::try_from(table_name.into())
    }
}

/// QueryStart to String conversion process
///
impl From<QueryStart> for String {
    fn from(query_start: QueryStart) -> Self {
        query_start.table_name
    }
}

/// When converting to string, add query start strings
impl ToQueryString for QueryStart {
    fn to_query_string(&self) -> String {
        format!("CREATE TABLE {} (", self.table_name)
    }
}
