use surrealdb::sql::{statements::DeleteStatement, Values};

pub struct DeleteStmt {
    origin: DeleteStatement,
}

impl DeleteStmt {
    pub fn new() -> Self {
        DeleteStmt {
            origin: DeleteStatement::default(),
        }
    }
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
}
