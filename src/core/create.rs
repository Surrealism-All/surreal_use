use surrealdb::sql::{statements::CreateStatement, Duration, Output, Timeout};

use crate::impl_stmt_bridge;

use super::sql::{CreateData, SurrealTable};

use super::StmtBridge;

/// ## create CREATE statement
/// If the records do not exist, they can be added to the database using the CREATE statement
/// ### example
/// ```
/// let s1 = CreateStmt::new().table("person".into()).data(
///     CreateData::set()
///         .push(SetField::new("name", None, "Tobie"))
///         .push(SetField::new("company", None, "SurrealDB"))
///         .push(SetField::new(
///             "skills",
///             None,
///             vec!["Rust", "Go", "JavaScript"],
///         )),
/// );
/// assert_eq!(s1.to_string().as_str(), "CREATE person SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript']" )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CreateStmt {
    origin: CreateStatement,
}

impl CreateStmt {
    pub fn new() -> Self {
        CreateStmt {
            origin: CreateStatement::default(),
        }
    }
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    pub fn data(mut self, data: CreateData) -> Self {
        self.origin.data.replace(data.into());
        self
    }
    pub fn output(mut self, output: Output) -> Self {
        self.origin.output.replace(output);
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.origin.timeout = Some(Timeout(timeout));
        self
    }
    /// ## Set whether statements can be processed in parallel
    /// default close
    pub fn parallel(mut self) -> Self {
        self.origin.parallel = true;
        self
    }
}

impl ToString for CreateStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

impl_stmt_bridge!(CreateStmt, CreateStatement);

#[cfg(test)]
mod test_create_stmt {

    use crate::core::sql::{CreateData, SetField};

    use super::CreateStmt;

    #[test]
    fn simple() {
        let s1 = CreateStmt::new().table("person".into()).data(
            CreateData::set()
                .push(SetField::new("name", None, "Tobie"))
                .push(SetField::new("company", None, "SurrealDB"))
                .push(SetField::new(
                    "skills",
                    None,
                    vec!["Rust", "Go", "JavaScript"],
                )),
        );
        assert_eq!(s1.to_string().as_str(), "CREATE person SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript']" )
    }
}
