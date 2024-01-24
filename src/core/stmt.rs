use super::create::CreateStmt;
use super::delete::DeleteStmt;
use super::r#use::UseStmt;
pub struct Stmt;

impl Stmt {
    /// ## 构建Use语句
    /// 由于use作为rust的关键字所以这里增加r#解决
    pub fn r#use() -> UseStmt {
        UseStmt::new()
    }
    /// ## 删除语句
    /// ### example
    /// ````
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
    pub fn create() -> CreateStmt {
        CreateStmt::new()
    }
}

#[cfg(test)]
mod test_stmt {
    use surrealdb::sql::Duration;

    use crate::core::sql::{Cond, CreateData, Field, SetField};

    use super::Stmt;

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
