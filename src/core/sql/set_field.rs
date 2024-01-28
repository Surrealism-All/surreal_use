use surrealdb::sql::{Idiom, Operator, Value};

use super::Field;

/// 处理 @field @op @value的情况
/// 例如：
/// - name = "Matt"
/// - age += 1
/// - user.name += "hello"
/// - ["true", "test", "text"] ?~ true
/// Operator枚举含有所有操作符
/// ```
/// 对齐Data::SetExpression(Vec<(Idiom, Operator, Value)>),
///                             ⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧⇧
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SetField {
    field: Field,
    op: Operator,
    value: Value,
}

impl SetField {
    pub fn new(field: impl Into<Field>, op: Option<Operator>, value: impl Into<Value>) -> Self {
        let op = match op {
            Some(o) => o,
            None => Operator::default(),
        };
        SetField {
            field: field.into(),
            op,
            value: value.into(),
        }
    }

    pub fn field(mut self, field: impl Into<Field>) -> Self {
        self.field = field.into();
        self
    }
    /// ## 设置操作符
    /// 一般不用设置，默认为等号 Operater::Equal
    pub fn op(mut self, op: Operator) -> Self {
        self.op = op;
        self
    }
    pub fn value(mut self, value: impl Into<Value>) -> Self {
        self.value = value.into();
        self
    }
    /// 转换为(Idiom, Operator, Value)
    pub fn to_origin(self) -> (Idiom, Operator, Value) {
        let idiom = Value::from(self.field).to_idiom();
        let op = self.op;
        let value = self.value;
        (idiom, op, value)
    }
}

impl From<(&str, &str)> for SetField {
    fn from(value: (&str, &str)) -> Self {
        Self {
            field: value.0.into(),
            op: Operator::default(),
            value: value.1.into(),
        }
    }
}

impl From<(&str, Operator, &str)> for SetField {
    fn from(value: (&str, Operator, &str)) -> Self {
        Self {
            field: value.0.into(),
            op: value.1,
            value: value.2.into(),
        }
    }
}

impl ToString for SetField {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.field.to_string(),
            self.op.to_string(),
            self.value.to_string()
        )
    }
}

#[cfg(test)]
mod test_set_field {
    use super::SetField;

    #[test]
    fn field_value() {
        let sf = SetField::default().field("name").value("Matt");
        assert_eq!(sf.to_string().as_str(), "name = 'Matt'");
    }

    #[test]
    fn new() {
        let sf = SetField::new("name", None, "Matt");
        assert_eq!(sf.to_string().as_str(), "name = 'Matt'");
    }

    #[test]
    fn default() {
        let s_f = SetField::default();
        // [src/core/sql/set_field.rs:26] s_f = SetField {
        //     field: Field(
        //         All,
        //     ),
        //     op: Equal,
        //     value: None,
        // }
        dbg!(s_f);
    }
}
