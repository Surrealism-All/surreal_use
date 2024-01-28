use serde::Serialize;
use surrealdb::sql::{to_value, Data, Value};

type UnitOp<'a> = InnerOp<'a, ()>;

/// ## 内置操作方式
#[derive(Debug, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum InnerOp<'a, T> {
    Add { path: &'a str, value: T },
    Remove { path: &'a str },
    Replace { path: &'a str, value: T },
    Change { path: &'a str, value: String },
}

/// ## Json Patch操作
/// 使用JSON PATCH的方式对数据进行更新
/// 这种方式出现在UPDATE语句中
pub struct PatchOp(Value);

impl PatchOp {
    /// ## Patch Add
    pub fn add<T>(path: &str, value: T) -> Self
    where
        T: Serialize,
    {
        let value = get_value(InnerOp::Add { path, value });
        Self(value)
    }
    /// ## Patch Remove
    pub fn remove(path: &str) -> Self {
        let value = get_value(UnitOp::Remove { path });
        Self(value)
    }
    /// ## Patch Replace
    pub fn replace<T>(path: &str, value: T) -> Self
    where
        T: Serialize,
    {
        let value = get_value(InnerOp::Replace { path, value });
        Self(value)
    }
    /// ## Patch Change
    pub fn change(path: &str, diff: &str) -> Self {
        let value = get_value(UnitOp::Change {
            path,
            value: diff.to_string(),
        });
        Self(value)
    }
    pub fn to_value(self) -> Value {
        self.0
    }
    pub fn to_origin(self) -> Data {
        Data::PatchExpression(self.to_value())
    }
}

impl From<PatchOp> for Value {
    fn from(value: PatchOp) -> Self {
        value.to_value()
    }
}

impl From<PatchOp> for Data {
    fn from(value: PatchOp) -> Self {
        value.to_origin()
    }
}

fn get_value<'a, T>(value: InnerOp<'a, T>) -> Value
where
    T: Serialize,
{
    match to_value(value) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}
