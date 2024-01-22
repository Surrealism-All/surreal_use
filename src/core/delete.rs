use surrealdb::sql::{statements::DeleteStatement, Duration, Timeout};

use super::value::{Cond, SurrrealTable};

/// ## DELETE statement
/// 删除记录
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    origin: DeleteStatement,
}

impl DeleteStmt {
    pub fn new() -> Self {
        DeleteStmt {
            origin: DeleteStatement::default(),
        }
    }
    /// ### 设置only为true
    /// 使用ONLY关键字，将只返回有问题的一条记录
    /// > Delete just a single record
    /// >
    /// > Using the ONLY keyword, just an object for the record in question will be returned.
    /// >
    /// > This, instead of an array with a single object.
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    pub fn table(mut self, table: SurrrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    /// ### example
    /// ```
    /// let cond = Cond(Value::Expression(Box::new(Expression::Binary {
    ///     l: Value::Strand(Strand("name".to_string())),
    ///     o: surrealdb::sql::Operator::Equal,
    ///     r: Value::Strand(Strand("zhang".to_string())),
    /// }))),
    /// ```
    pub fn cond(mut self , cond:Cond) -> Self {
        self.origin.cond.replace(cond.to_origin());
        self
    }
    pub fn output(mut self) -> Self {
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.origin.timeout = Some(Timeout(timeout));
        self
    }
    pub fn parallel(mut self) -> Self {
        self.origin.parallel = true;
        self
    }
}

impl ToString for DeleteStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

#[cfg(test)]
mod test_delete {
    use surrealdb::sql::{
        statements::DeleteStatement, Cond, Expression, Id, Strand, Value, Values,
    };

    use super::DeleteStmt;

    #[test]
    fn simple_delete() {
        let delete1 = DeleteStmt::new().table("person".into());
        let delete2 = DeleteStmt::new().table(("person", Id::Number(100)).into());
        let delete3 = DeleteStmt::new().table(("person", "tobie").into());
        let delete4 = DeleteStmt::new().table(("person", "tobie").into()).only();
        assert_eq!(delete1.to_string().as_str(), "DELETE person");
        assert_eq!(delete2.to_string().as_str(), "DELETE person:100");
        assert_eq!(delete3.to_string().as_str(), "DELETE person:tobie");
        assert_eq!(delete4.to_string().as_str(), "DELETE ONLY person:tobie");
    }

    #[test]
    fn test_origin() {
        let origin = DeleteStatement {
            only: true,
            what: Values(vec![Value::Thing(("person", "123sdaqo24sno2").into())]),
            cond: Some(Cond(Value::Expression(Box::new(Expression::Binary {
                l: Value::Strand(Strand("name".to_string())),
                o: surrealdb::sql::Operator::Equal,
                r: Value::Strand(Strand("zhang".to_string())),
            })))),
            output: None,
            timeout: None,
            parallel: false,
        };
        // Surreal::select(&self, resource)
        dbg!(origin.to_string());
    }
}
