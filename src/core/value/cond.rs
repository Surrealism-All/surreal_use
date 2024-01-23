use std::mem;

use surrealdb::sql::{self, Expression, Operator, Value};

use super::Edges;

/// # 条件表达式（where）
/// 使用在WHERE子句中，构造条件表达式
/// ```
/// cond: Some(Cond(Value::Expression(Box::new(Expression::Binary {
///     l: Value::Strand(Strand("name".to_string())),
///     o: surrealdb::sql::Operator::Equal,
///     r: Value::Strand(Strand("zhang".to_string())),
/// })))),
/// ```
/// ## example
/// ```
/// let cond = Cond::new()
/// .left(Value::Array(vec![
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
    /// ## 创建条件表达式
    /// 使用Expression::Binary的方式进行构建
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
    /// ## 构建左侧
    pub fn left(mut self, left: Value) -> Self {
        self.replace(|expression| match expression {
            Expression::Unary { o: _, v: _ } => {
                panic!("Unexpected unary expression , If you see this panic , please send issue!")
            }
            Expression::Binary { l, o: _, r: _ } => {
                let _ = mem::replace(l, left);
            }
        });
        self
    }
    /// ## 构建左侧（简单）
    /// 用于比较简单的条件表达，左侧将变为简单的field
    /// ### example
    /// ```
    /// let cond = Cond::new()
    /// .left_easy("username")
    /// .op(surrealdb::sql::Operator::Equal)
    /// .right("Matt".into());
    /// assert_eq!(cond.to_string().as_str(), "WHERE username = 'Matt'");
    /// ```
    pub fn left_easy(mut self, left: &str) -> Self {
        let left = Value::Table(left.into());
        self.left(left)
    }
    /// ## 构建右侧
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
    /// ## 构建逻辑操作符
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

    /// 替换表达式中的字段
    /// 可能是：
    /// - left
    /// - right
    /// - op
    /// 所以采用FnOnce进行区分操作
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

impl From<Edges> for Cond {
    fn from(value: Edges) -> Self {
        Cond(value.into())
    }
}

#[cfg(test)]
mod test_cond {
    use surrealdb::sql::{Expression, Value};

    use super::Cond;

    #[test]
    fn complex() {
        let cond = Cond::new()
            .left(Value::Array(vec!["Jack", "John"].into()))
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
