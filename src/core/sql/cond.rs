use super::Field;
use std::mem;
use surrealdb::sql::{self, Expression, Operator, Value};

// use super::Edges;(not reachable)

/// # conditional expression（where）
/// Use in the WHERE clause to construct conditional expressions
/// ```
/// cond: Some(Cond(Value::Expression(Box::new(Expression::Binary {
///     l: Value::Strand(Strand("name".to_string())),
///     o: surrealdb::sql::Operator::Equal,
///     r: Value::Strand(Strand("zhang".to_string())),
/// })))),
/// ```
/// ## example
/// ```
/// // recommend
/// let cond = Cond::new()
///     .left("user.name")
///     .op(surrealdb::sql::Operator::Add)
///     .right("-vip".into());
/// assert_eq!(cond.to_string().as_str(), "WHERE user.name + '-vip'");
/// //----------------------------------------------------
/// let cond = Cond::new()
/// .left_value(Value::Array(vec![
///     "Jack","John"
/// ].into()))
/// .op(surrealdb::sql::Operator::Contain)
/// .right(Value::Strand( "(SELECT name FROM vip WHERE id = '1')".into()));
/// assert_eq!(
/// cond.to_string().as_str(),
/// "WHERE ['Jack', 'John'] CONTAINS \"(SELECT name FROM vip WHERE id = '1')\""
/// );
/// //----------------------------------------------------------------
/// let cond = Cond::new()
/// .left_easy("username")
/// .op(surrealdb::sql::Operator::Equal)
/// .right("Matt".into());
/// assert_eq!(cond.to_string().as_str(), "WHERE username = 'Matt'");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Cond(sql::Cond);

impl Cond {
    /// ## build a new cond instance
    /// use `Expression::Binary`
    pub fn new() -> Cond {
        Cond(sql::Cond(Value::Expression(Box::new(Expression::Binary {
            l: Value::default(),
            o: Operator::default(),
            r: Value::default(),
        }))))
    }
    pub fn to_origin(self) -> sql::Cond {
        self.0
    }

    /// ## build left (Field) (recommend)
    /// use surreal_use::core::sql::Field to build left
    /// 1. multiple types of input, relatively flexible
    /// 2. has clear direction
    /// ### example
    /// ```
    /// let cond = Cond::new()
    ///     .left("user.name")
    ///     .op(surrealdb::sql::Operator::Add)
    ///     .right("-vip".into());
    /// assert_eq!(cond.to_string().as_str(), "WHERE user.name + '-vip'");
    /// ```
    pub fn left(self, left: impl Into<Field>) -> Self {
        let field: Field = left.into();
        self.left_value(field.into())
    }
    /// ## build left
    /// Using the original Value form, flexible and easy to expand
    /// ### example
    /// ```
    /// let cond = Cond::new()
    ///     .left_value(Value::Array(vec!["Jack", "John"].into()))
    ///     .op(surrealdb::sql::Operator::Contain)
    ///     .right(Value::Strand(
    ///         "(SELECT name FROM vip WHERE id = '1')".into(),
    ///     ));
    /// assert_eq!(
    ///     cond.to_string().as_str(),
    ///     "WHERE ['Jack', 'John'] CONTAINS \"(SELECT name FROM vip WHERE id = '1')\""
    /// );
    /// ```
    pub fn left_value(mut self, left: Value) -> Self {
        self.replace(|expression| match expression {
            Expression::Unary { o: _, v: _ } => {
                panic!("Unexpected unary expression , If you see this panic , please send issue!")
            }
            Expression::Binary { l, o: _, r: _ } => {
                let _ = mem::replace(l, left.into());
            }
        });
        self
    }

    /// ## build left (easy)
    /// Used for relatively simple conditional expressions, the left side will become a simple field
    /// ### example
    /// ```
    /// let cond = Cond::new()
    /// .left_easy("username")
    /// .op(surrealdb::sql::Operator::Equal)
    /// .right("Matt".into());
    /// assert_eq!(cond.to_string().as_str(), "WHERE username = 'Matt'");
    /// ```
    pub fn left_easy(self, left: &str) -> Self {
        // let left = Idiom::from(vec![Part::from(left)]);
        // let left = Field::from(str);
        self.left(left)
    }
    /// ## build right
    pub fn right(mut self, right: Value) -> Self {
        self.replace(|expression| match expression {
            Expression::Unary { o: _, v: _ } => {
                panic!("Unexpected unary expression , If you see this panic , please send issue!")
            }
            Expression::Binary { l: _, o: _, r } => {
                let _ = mem::replace(r, right);
            }
        });
        self
    }
    /// ## Building logical operators
    pub fn op(mut self, op: Operator) -> Self {
        self.replace(|expression| match expression {
            Expression::Unary { o: _, v: _ } => {
                panic!("Unexpected unary expression , If you see this panic , please send issue!")
            }
            Expression::Binary { l: _, o, r: _ } => {
                let _ = mem::replace(o, op);
            }
        });
        self
    }

    /// Replace fields in expressions
    ///
    /// maybe:
    /// - left
    /// - right
    /// - op
    /// So FnOnce is used for differentiation operations
    fn replace<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Expression),
    {
        match &mut self.0 .0 {
            Value::Expression(expression) => {
                let mut expr = expression.as_mut();
                f(&mut expr);
            }
            _ => {}
        };
        self
    }
}

impl ToString for Cond {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// impl From<Edges> for Cond {
//     fn from(value: Edges) -> Self {
//         Cond(sql::Cond(value.into()))
//     }
// }

#[cfg(test)]
mod test_cond {
    use surrealdb::sql::{Expression, Value};

    use super::Cond;

    #[test]
    fn left_field() {
        let cond = Cond::new()
            .left("user.name")
            .op(surrealdb::sql::Operator::Add)
            .right("-vip".into());
        assert_eq!(cond.to_string().as_str(), "WHERE user.name + '-vip'");
    }
    #[test]
    fn complex() {
        let cond = Cond::new()
            .left_value(Value::Array(vec!["Jack", "John"].into()))
            .op(surrealdb::sql::Operator::Contain)
            .right(Value::Strand(
                "(SELECT name FROM vip WHERE id = '1')".into(),
            ));
        assert_eq!(
            cond.to_string().as_str(),
            "WHERE ['Jack', 'John'] CONTAINS \"(SELECT name FROM vip WHERE id = '1')\""
        );
    }
    /// 简单的例子
    #[test]
    fn simple() {
        let cond = Cond::new()
            .left_easy("username")
            .op(surrealdb::sql::Operator::Equal)
            .right("Matt".into());
        assert_eq!(cond.to_string().as_str(), "WHERE username = 'Matt'");
    }
    #[test]
    fn test_expression_unary() {
        let express = Expression::Unary {
            o: surrealdb::sql::Operator::Add,
            v: "name".into(),
        };
        assert_eq!(express.to_string().as_str(), "+'name'");
    }
}
