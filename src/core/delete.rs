//! 待解决问题1 : Edges -> Cond 转换
//! ````
//! fn delete_timeout() {
//!     let inner_where = format!(
//!         "({} {})",
//!         "knows",
//!         Cond::new()
//!             .left_easy("influencer")
//!             .op(Operator::Equal)
//!             .right(false.into())
//!             .to_string()
//!     );
//!     // let where_edges:Edges = ("a",Dir::Out,"b").into();
//!
//!     // ((("",Dir::Out,"knows"),Dir::Out,"person"),Dir::Out,&inner_where)
//!     let where_edges: Edges = Edges::new(
//!         Edges::new(
//!             Edges::new("".into(), Dir::Out, "knows".into()).into(),
//!             Dir::Out,
//!             "person".into(),
//!         )
//!         .into(),
//!         Dir::Out,
//!         inner_where.as_str().into(),
//!     );
//!
//!     let delete = DeleteStmt::new()
//!         .table("person".into())
//!         .cond(where_edges.into())
//!         .timeout(sql::Duration::from_secs(5));
//!     dbg!(delete.to_string());
//!     // assert_eq!(delete.to_string().as_str() , "DELETE person WHERE ->knows->person->(knows WHERE influencer = false) TIMEOUT 5s;");
//! }
//! ````
use surrealdb::sql::{statements::DeleteStatement, Duration, Output, Timeout};

use crate::impl_stmt_bridge;

use super::sql::{Cond, SurrealTable};
use super::StmtBridge;
/// ## DELETE statement
/// delete record
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    origin: DeleteStatement,
}

impl DeleteStmt {
    /// ## create DELETE statement
    pub fn new() -> Self {
        DeleteStmt {
            origin: DeleteStatement::default(),
        }
    }
    /// ### set keyword only
    /// use keyword ONLY, Only one problematic record will be returned
    /// > Delete just a single record
    /// >
    /// > Using the ONLY keyword, just an object for the record in question will be returned.
    /// >
    /// > This, instead of an array with a single object.
    ///
    /// ## example
    /// ```
    /// let delete4 = DeleteStmt::new().table(("person", "tobie").into()).only();
    /// assert_eq!(delete4.to_string().as_str(), "DELETE ONLY person:tobie");
    /// ```
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    /// ## set which table to delete
    /// ### example
    /// ```
    /// let delete1 = DeleteStmt::new().table("person".into());
    /// ```
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    /// set WHERE sub query
    /// ### example
    /// ```
    /// let delete = DeleteStmt::new().table("city".into()).cond(
    ///     Cond::new()
    ///         .left("name".into())
    ///         .op(Operator::Equal)
    ///         .right("London".into()),
    /// );
    /// assert_eq!(delete.to_string().as_str(),"DELETE city WHERE name = 'London'");
    /// ```
    pub fn cond(mut self, cond: Cond) -> Self {
        self.origin.cond.replace(cond.to_origin());
        self
    }
    /// ## set RETURN sub query
    /// - DIFF
    /// - NONE
    /// - BEFORE
    /// - AFTER
    /// - FIELD ...
    /// ### example
    /// ```
    /// let delete_none = DeleteStmt::new()
    ///     .table("user".into())
    ///     .cond(
    ///         Cond::new()
    ///             .left_easy("age")
    ///             .op(Operator::MoreThan)
    ///             .right(16.into()),
    ///     )
    ///     .output(sql::Output::None);
    /// let delete_after = DeleteStmt::new()
    ///     .table("user".into())
    ///     .cond(
    ///         Cond::new()
    ///             .left_easy("age")
    ///             .op(Operator::MoreThan)
    ///             .right(16.into()),
    ///     )
    ///     .output(sql::Output::After);
    /// let delete_field_easy =  DeleteStmt::new()
    /// .table("user".into())
    /// .cond(
    ///     Cond::new()
    ///         .left_easy("age")
    ///         .op(Operator::MoreThan)
    ///         .right(16.into()),
    /// )
    /// .output(Field::single("userId", None).into());
    ///
    /// // use surrealdb::sql::Output and surrealdb::sql::Field (not recommend)
    /// let delete_field = DeleteStmt::new()
    ///     .table("user".into())
    ///     .cond(
    ///         Cond::new()
    ///             .left_easy("age")
    ///             .op(Operator::MoreThan)
    ///             .right(16.into()),
    ///     )
    ///     .output(sql::Output::Fields(sql::Fields(
    ///         vec![sql::Field::Single { expr: Table("userId".to_string()).into(), alias: None }],
    ///         false
    ///     )));
    /// assert_eq!(
    ///     delete_none.to_string().as_str(),
    ///     "DELETE user WHERE age > 16 RETURN NONE"
    /// );
    /// assert_eq!(
    ///     delete_after.to_string().as_str(),
    ///     "DELETE user WHERE age > 16 RETURN AFTER"
    /// );
    /// assert_eq!(
    ///     delete_field_easy.to_string().as_str(),
    ///     "DELETE user WHERE age > 16 RETURN userId"
    /// );
    /// assert_eq!(
    ///     delete_field.to_string().as_str(),
    ///     "DELETE user WHERE age > 16 RETURN userId"
    /// );
    /// ```
    pub fn output(mut self, output: Output) -> Self {
        self.origin.output.replace(output);
        self
    }
    /// ## Set delayed execution time
    /// ### example
    /// ```
    /// let delete = DeleteStmt::new()
    ///     .table("person".into())
    ///     .timeout(sql::Duration::from_secs(5));
    /// assert_eq!(delete.to_string().as_str(), "DELETE person TIMEOUT 5s");
    /// ```
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

impl ToString for DeleteStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

impl_stmt_bridge!(DeleteStmt, DeleteStatement);

#[cfg(test)]
mod test_delete {
    use surrealdb::sql::{
        self, statements::DeleteStatement, Expression, Id, Operator, Strand, Table, Value, Values,
    };

    use crate::core::sql::{Cond, Field};

    use super::DeleteStmt;

    #[test]
    fn delete_timeout() {
        let delete = DeleteStmt::new()
            .table("person".into())
            .timeout(sql::Duration::from_secs(5));
        assert_eq!(delete.to_string().as_str(), "DELETE person TIMEOUT 5s");
    }

    #[test]
    fn delete_output() {
        let delete_none = DeleteStmt::new()
            .table("user".into())
            .cond(
                Cond::new()
                    .left_easy("age")
                    .op(Operator::MoreThan)
                    .right(16.into()),
            )
            .output(sql::Output::None);
        let delete_after = DeleteStmt::new()
            .table("user".into())
            .cond(
                Cond::new()
                    .left_easy("age")
                    .op(Operator::MoreThan)
                    .right(16.into()),
            )
            .output(sql::Output::After);
        let delete_field_easy = DeleteStmt::new()
            .table("user".into())
            .cond(
                Cond::new()
                    .left_easy("age")
                    .op(Operator::MoreThan)
                    .right(16.into()),
            )
            .output(Field::single("userId", None).into());

        // use surrealdb::sql::Output and surrealdb::sql::Field
        let delete_field = DeleteStmt::new()
            .table("user".into())
            .cond(
                Cond::new()
                    .left_easy("age")
                    .op(Operator::MoreThan)
                    .right(16.into()),
            )
            .output(sql::Output::Fields(sql::Fields(
                vec![sql::Field::Single {
                    expr: Table("userId".to_string()).into(),
                    alias: None,
                }],
                false,
            )));
        assert_eq!(
            delete_none.to_string().as_str(),
            "DELETE user WHERE age > 16 RETURN NONE"
        );
        assert_eq!(
            delete_after.to_string().as_str(),
            "DELETE user WHERE age > 16 RETURN AFTER"
        );
        assert_eq!(
            delete_field_easy.to_string().as_str(),
            "DELETE user WHERE age > 16 RETURN userId"
        );
        assert_eq!(
            delete_field.to_string().as_str(),
            "DELETE user WHERE age > 16 RETURN userId"
        );
    }
    // #[test]
    // fn delete_record_base_on_cond() {
    //     let delete = DeleteStmt::new().table("city".into()).cond(
    //         Cond::new()
    //             .left("name".into())
    //             .op(Operator::Equal)
    //             .right("London".into()),
    //     );
    //     assert_eq!(
    //         delete.to_string().as_str(),
    //         "DELETE city WHERE name = 'London'"
    //     );
    // }

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
            cond: Some(sql::Cond(Value::Expression(Box::new(Expression::Binary {
                l: Value::Table("name".into()),
                o: surrealdb::sql::Operator::Equal,
                r: Value::Strand(Strand("zhang".to_string())),
            })))),
            output: None,
            timeout: None,
            parallel: false,
        };

        assert_eq!(
            origin.to_string().as_str(),
            "DELETE ONLY person:123sdaqo24sno2 WHERE name = 'zhang'"
        );
    }
}
