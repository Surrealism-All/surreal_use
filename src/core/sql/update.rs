use serde::Serialize;
use surrealdb::sql::{to_value, Data, Idiom, Operator, Value};

use super::{PatchOp, SetField};

/// ## 更新数据的形式
/// - SET
/// - CONTENT
/// - MERGE
/// - PATCH
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateData {
    Set(Vec<SetField>),
    Content(Value),
    Merge(Value),
    Patch(Value),
}

impl UpdateData {
    /// 初始化UpdateData::Set方式
    pub fn set() -> Self {
        UpdateData::Set(vec![])
    }
    /// 增加Set类型数据
    pub fn push(mut self, sf: SetField) -> Self {
        match &mut self {
            UpdateData::Set(s) => {
                s.push(sf);
            }
            _ => panic!("Cannot push to UpdateData::Content"),
        };
        self
    }
    /// 去除Set类型最后一个数据
    pub fn pop(mut self) -> Self {
        match &mut self {
            UpdateData::Set(s) => s.pop(),
            _ => panic!("Cannot pop to UpdateData::Content"),
        };
        self
    }
    /// 将可被序列化的结构体数据转为UpdateData::Content
    pub fn content<D>(value: D) -> Self
    where
        D: Serialize,
    {
        match to_value(value) {
            Ok(content) => UpdateData::Content(content),
            Err(e) => panic!("{}", e),
        }
    }
    pub fn merge<D>(value: D) -> Self
    where
        D: Serialize,
    {
        match to_value(value) {
            Ok(content) => UpdateData::Merge(content),
            Err(e) => panic!("{}", e),
        }
    }
    /// ## 使用JSON Patch方式修改
    pub fn patch(value: Vec<PatchOp>) -> Self {
        let value = value
            .into_iter()
            .map(|x| x.to_value())
            .collect::<Vec<Value>>();
        UpdateData::Patch(value.into())
    }
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }
    pub fn is_content(&self) -> bool {
        matches!(self, Self::Content(_))
    }
    pub fn is_patch(&self) -> bool {
        matches!(self, Self::Patch(_))
    }
    pub fn is_merge(&self) -> bool {
        matches!(self, Self::Merge(_))
    }
    pub fn to_set(self) -> Option<Vec<SetField>> {
        match self {
            UpdateData::Set(s) => Some(s),
            _ => None,
        }
    }
    pub fn to_content(self) -> Option<Value> {
        match self {
            UpdateData::Content(c) => Some(c),
            _ => None,
        }
    }
    pub fn to_patch(self) -> Option<Value> {
        match self {
            UpdateData::Patch(p) => Some(p),
            _ => None,
        }
    }
    pub fn to_merge(self) -> Option<Value> {
        match self {
            UpdateData::Merge(m) => Some(m),
            _ => None,
        }
    }
}

impl From<UpdateData> for Data {
    fn from(value: UpdateData) -> Self {
        match value {
            UpdateData::Set(s) => Data::SetExpression(
                s.into_iter()
                    .map(|x| x.to_origin())
                    .collect::<Vec<(Idiom, Operator, Value)>>(),
            ),
            UpdateData::Content(c) => Data::ContentExpression(c),
            UpdateData::Merge(m) => Data::MergeExpression(m),
            UpdateData::Patch(p) => Data::PatchExpression(p),
        }
    }
}

#[cfg(test)]
mod test_update_data {
    use serde::Serialize;
    use surrealdb::sql::Data;

    use crate::core::sql::PatchOp;

    use super::UpdateData;

    #[test]
    fn patch() {
        let update = UpdateData::patch(vec![
            PatchOp::add("/tags", &["developer", "engineer"]),
            PatchOp::replace("/settings/active", false),
        ]);
        dbg!(Data::from(update).to_string().as_str());
    }

    #[test]
    fn merge() {
        #[derive(Debug, Clone, Serialize)]
        struct Person {
            marketing: bool,
        }
        let update = UpdateData::merge(Person { marketing: true });
        assert_eq!(
            Data::from(update).to_string().as_str(),
            "MERGE { marketing: true }"
        );
    }
}
