use serde::Serialize;
use surrealdb::sql::{to_value, Data, Idiom, Operator, Value};

use super::SetField;

/// ## How to add data to the CREATE statement
/// - SET @field = @value
/// - CONTENT @value
#[derive(Debug, Clone, PartialEq)]
pub enum CreateData {
    Set(Vec<SetField>),
    Content(Value),
}

impl CreateData {
    /// init CreateData::Set
    pub fn set() -> Self {
        CreateData::Set(vec![])
    }
    /// Add CreateData::Set type data
    pub fn push(mut self, sf: SetField) -> Self {
        match &mut self {
            CreateData::Set(s) => {
                s.push(sf);
            }
            CreateData::Content(_) => panic!("Cannot push to CreateData::Content"),
        };
        self
    }
    /// Remove the last data of the CreateData::Set type
    pub fn pop(mut self) -> Self {
        match &mut self {
            CreateData::Set(s) => s.pop(),
            CreateData::Content(_) => panic!("Cannot pop to CreateData::Content"),
        };
        self
    }
    /// Convert serializable structural data to CreateData::Content
    pub fn content<D>(value: D) -> Self
    where
        D: Serialize,
    {
        match to_value(value) {
            Ok(content) => CreateData::Content(content),
            Err(e) => panic!("{}", e),
        }
    }
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }
    pub fn is_content(&self) -> bool {
        !self.is_set()
    }
    /// Convert to origin set: `Vec<SetField>`
    pub fn to_set(self) -> Option<Vec<SetField>> {
        match self {
            CreateData::Set(s) => Some(s),
            CreateData::Content(_) => None,
        }
    }
    /// Convert to origin content: `Value`
    pub fn to_content(self) -> Option<Value> {
        match self {
            CreateData::Set(_) => None,
            CreateData::Content(c) => Some(c),
        }
    }
    /// Convert `Vec<impl Into<SetField>>` to CreateData::Set
    pub fn from_vec(values: Vec<impl Into<SetField>>) -> Self {
        let sets = values
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<SetField>>();
        Self::Set(sets)
    }
}

impl ToString for CreateData {
    fn to_string(&self) -> String {
        match self {
            CreateData::Set(s) => s
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            CreateData::Content(s) => s.to_string(),
        }
    }
}

impl<D> From<D> for CreateData
where
    D: Serialize,
{
    fn from(value: D) -> Self {
        CreateData::content(value)
    }
}

impl From<CreateData> for Data {
    fn from(value: CreateData) -> Self {
        match value {
            CreateData::Set(s) => {
                let stmt = s
                    .into_iter()
                    .map(|x| x.to_origin())
                    .collect::<Vec<(Idiom, Operator, Value)>>();
                Data::SetExpression(stmt)
            }
            CreateData::Content(c) => Data::ContentExpression(c),
        }
    }
}

#[cfg(test)]
mod test_create_data {
    use serde::Serialize;
    use surrealdb::sql::{Data, Ident, Idiom, Operator, Part};

    use crate::core::sql::SetField;

    use super::CreateData;

    #[test]
    fn test_set() {
        let set = CreateData::set()
            .push(SetField::new("name", None, "Matt"))
            .push(SetField::new("age", None, 14));
        dbg!(set.to_string());
    }

    #[test]
    fn test_content() {
        #[derive(Debug, Clone, Serialize)]
        struct Person {
            name: String,
            age: u8,
        }

        let content = CreateData::content(Person {
            name: "sp".to_string(),
            age: 8,
        });
        assert_eq!(content.to_string().as_str(), "{ age: 8, name: 'sp' }");
    }

    #[test]
    fn origin_set() {
        let d = Data::SetExpression(vec![(
            Idiom(vec![Part::Field(Ident(String::from("name")))]),
            Operator::Equal,
            "Matt".into(),
        )]);
        assert_eq!(d.to_string().as_str(), "SET name = 'Matt'");
    }
}
