use surrealdb::sql::{Operator, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct SetField {
    field: String,
    op: Operator,
    value: Value,
}
