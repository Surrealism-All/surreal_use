use super::create::CreateStmt;
use super::delete::DeleteStmt;
use super::insert::InsertStmt;
use super::r#use::UseStmt;
use super::select::SelectStmt;
use super::update::UpdateStmt;
pub struct Stmt;

impl Stmt {
    /// ## use statement
    /// ### example
    /// ```
    /// let use_s = Stmt::r#use().ns("surreal").db("use");
    /// let use_str = "USE NS surreal DB use";
    /// assert_eq!(use_str, &use_s.to_string());
    /// ```
    pub fn r#use() -> UseStmt {
        UseStmt::new()
    }
    /// ## delete statement
    /// ### example
    /// ```
    /// let delete = Stmt::delete()
    ///     .table("user".into())
    ///     .cond(
    ///         Cond::new()
    ///             .left("userId")
    ///             .op(surrealdb::sql::Operator::Equal)
    ///             .right("2343jshkq1".into()),
    ///     )
    ///     .output(Field::single("username", None).into())
    ///     .timeout(Duration::from_secs(10))
    ///     .parallel();
    /// assert_eq!(
    ///     delete.to_string().as_str(),
    ///     "DELETE user WHERE userId = '2343jshkq1' RETURN username TIMEOUT 10s PARALLEL"
    /// );
    /// ```
    pub fn delete() -> DeleteStmt {
        DeleteStmt::new()
    }
    /// ## create statement
    /// ### example
    /// ```
    /// let create = Stmt::create()
    ///     .table(("person", "matt1008").into())
    ///     .data(CreateData::set().push(SetField::new("age", None, 46)))
    ///     .output(surrealdb::sql::Output::Before)
    ///     .timeout(Duration::from_millis(15))
    ///     .parallel();
    /// assert_eq!(
    ///     create.to_string().as_str(),
    ///     "CREATE person:matt1008 SET age = 46 RETURN BEFORE TIMEOUT 15ms PARALLEL"
    /// );
    /// ```
    pub fn create() -> CreateStmt {
        CreateStmt::new()
    }
    /// ## insert statement
    /// ### example
    /// ```
    /// let insert = Stmt::insert()
    ///     .table("company".into())
    ///     .data(
    ///         InsertData::set()
    ///             .push("name", "SurrealDB")
    ///             .push("founded", "2021-09-10"),
    ///     )
    ///     .ignore()
    ///     .output(surrealdb::sql::Output::Diff)
    ///     .parallel();
    /// assert_eq!(
    ///     insert.to_string().as_str(),
    ///     "INSERT IGNORE INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10') RETURN DIFF PARALLEL"
    /// )
    /// ```
    pub fn insert() -> InsertStmt {
        InsertStmt::new()
    }
    /// ## update statement
    /// ### example
    /// ```
    /// let update = Stmt::update()
    ///     .only()
    ///     .table(("person", "tobie").into())
    ///     .data(
    ///         UpdateData::set()
    ///             .push(SetField::new("name", None, "Tobie"))
    ///             .push(SetField::new("company", None, "SurrealDB"))
    ///             .push(SetField::new(
    ///                 "skills",
    ///                 None,
    ///                 vec!["Rust".to_string(), "Go".to_string()],
    ///             )),
    ///     );
    /// assert_eq!(update.to_string().as_str(), "UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go']");
    /// ```
    pub fn update() -> UpdateStmt {
        UpdateStmt::new()
    }
    /// ## select statement
    /// ### example
    /// ```
    /// let select = Stmt::select()
    ///     .table(("person", "tobie").into())
    ///     .fields(vec![
    ///         Field::new("name"),
    ///         Field::new("address"),
    ///         Field::new("email"),
    ///     ]);
    /// assert_eq!(
    ///     select.to_string().as_str(),
    ///     "SELECT name, address, email FROM person:tobie"
    /// );
    /// ```
    pub fn select() -> SelectStmt {
        SelectStmt::new()
    }
}

#[cfg(test)]
mod test_stmt {
    use surrealdb::sql::Duration;

    use crate::core::sql::{Cond, CreateData, Field, InsertData, SetField, UpdateData};

    use super::Stmt;
    #[test]
    fn test_select() {
        let select = Stmt::select()
            .table(("person", "tobie").into())
            .fields(vec![
                Field::new("name"),
                Field::new("address"),
                Field::new("email"),
            ]);
        assert_eq!(
            select.to_string().as_str(),
            "SELECT name, address, email FROM person:tobie"
        );
    }

    #[test]
    fn test_update() {
        let update = Stmt::update()
            .only()
            .table(("person", "tobie").into())
            .data(
                UpdateData::set()
                    .push(SetField::new("name", None, "Tobie"))
                    .push(SetField::new("company", None, "SurrealDB"))
                    .push(SetField::new(
                        "skills",
                        None,
                        vec!["Rust".to_string(), "Go".to_string()],
                    )),
            );
        assert_eq!(update.to_string().as_str(), "UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go']");
    }

    #[test]
    fn test_insert() {
        let insert = Stmt::insert()
            .table("company".into())
            .data(
                InsertData::set()
                    .push("name", "SurrealDB")
                    .push("founded", "2021-09-10"),
            )
            .ignore()
            .output(surrealdb::sql::Output::Diff)
            .parallel();
        assert_eq!(
            insert.to_string().as_str(),
            "INSERT IGNORE INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10') RETURN DIFF PARALLEL"
        )
    }

    #[test]
    fn test_use() {
        let use_s = Stmt::r#use().ns("surreal").db("use");
        let use_str = "USE NS surreal DB use";
        assert_eq!(use_str, &use_s.to_string());
    }
    #[test]
    fn test_delete() {
        let delete = Stmt::delete()
            .table("user".into())
            .cond(
                Cond::new()
                    .left("userId")
                    .op(surrealdb::sql::Operator::Equal)
                    .right("2343jshkq1".into()),
            )
            .output(Field::single("username", None).into())
            .timeout(Duration::from_secs(10))
            .parallel();
        assert_eq!(
            delete.to_string().as_str(),
            "DELETE user WHERE userId = '2343jshkq1' RETURN username TIMEOUT 10s PARALLEL"
        );
    }
    #[test]
    fn test_create() {
        let create = Stmt::create()
            .table(("person", "matt1008").into())
            .data(CreateData::set().push(SetField::new("age", None, 46)))
            .output(surrealdb::sql::Output::Before)
            .timeout(Duration::from_millis(15))
            .parallel();
        assert_eq!(
            create.to_string().as_str(),
            "CREATE person:matt1008 SET age = 46 RETURN BEFORE TIMEOUT 15ms PARALLEL"
        );
    }
}
