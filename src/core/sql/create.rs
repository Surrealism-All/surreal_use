use surrealdb::sql::{statements::SetStatement, Array, Data, Object, Value};

use super::SetField;

#[derive(Debug, Clone, PartialEq)]
pub enum CreateData {
    Set(Vec<SetField>),
    Content(Object),
}

impl CreateData {
    // pub fn set() -> Self {
    //   CreateData(
    //     Data::SetExpression(())
    //   )
    // }
    pub fn content(value: Value) -> Self {
        CreateData::Content()
    }
}

#[cfg(test)]
mod test_create_data {
    use surrealdb::sql::{Data, Ident, Idiom, Operator, Part};

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
