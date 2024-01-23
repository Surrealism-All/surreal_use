use super::delete::DeleteStmt;
use super::r#use::UseStmt;
pub struct Stmt;

impl Stmt {
    /// ## 构建Use语句
    /// 由于use作为rust的关键字所以这里增加r#解决
    pub fn r#use() -> UseStmt {
        UseStmt::new()
    }
    pub fn delete() -> DeleteStmt {
        DeleteStmt::new()
    }
}

#[cfg(test)]
mod test_stmt {
    use surrealdb::sql::Duration;

    use crate::core::value::{Cond, Field};

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
                    .left_easy("userId")
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
}
