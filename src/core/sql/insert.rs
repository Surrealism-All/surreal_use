use serde::Serialize;
use surrealdb::sql::{to_value, Data, Idiom, Value};

use super::Field;

/// Set : Data::ValueExpression
/// Content : Data::SingleExpression
#[derive(Debug, Clone, PartialEq)]
pub enum InsertData {
    Set(Vec<Vec<(Idiom, Value)>>),
    Content(Value),
}

impl InsertData {
    pub fn set() -> Self {
        InsertData::Set(vec![])
    }
    /// ## new instance: InsertData::Content
    pub fn content<D>(value: D) -> Self
    where
        D: Serialize,
    {
        match to_value(value) {
            Ok(v) => InsertData::Content(v),
            Err(e) => panic!("{}", e),
        }
    }
    pub fn push<D>(mut self, key: impl Into<Field>, value: D) -> Self
    where
        D: Serialize,
    {
        match &mut self {
            InsertData::Set(s) => {
                let value = match to_value(value) {
                    Ok(v) => v,
                    Err(e) => panic!("{}", e),
                };
                let item = (Value::from(key.into()).to_idiom(), value);
                if s.len().eq(&0) {
                    s.push(vec![item]);
                } else {
                    s[0].push(item);
                }
                self
            }
            InsertData::Content(_) => panic!("Cannot push to InsertData::Content"),
        }
    }
    pub fn is_content(&self) -> bool {
        matches!(self, InsertData::Content(_))
    }
    pub fn is_set(&self) -> bool {
        !self.is_content()
    }
    pub fn to_origin(self) -> Data {
        Data::from(self)
    }
}

impl From<InsertData> for Data {
    fn from(value: InsertData) -> Self {
        match value {
            InsertData::Set(s) => Data::ValuesExpression(s),
            InsertData::Content(c) => Data::SingleExpression(c),
        }
    }
}

impl ToString for InsertData {
    fn to_string(&self) -> String {
        match self {
            InsertData::Set(s) => Data::ValuesExpression(s.to_vec()).to_string(),
            InsertData::Content(c) => c.to_string(),
        }
    }
}

#[cfg(test)]
mod test_insert_data {
    use serde::Serialize;

    use super::InsertData;
    #[derive(Debug, Clone, Serialize)]
    struct IdCard {
        id: String,
        card_type: String,
    }

    #[test]
    fn content() {
        let content = InsertData::content(IdCard {
            id: "jshdo18ch1823".to_string(),
            card_type: "temp".to_string(),
        });
        assert_eq!(
            content.to_string().as_str(),
            "{ card_type: 'temp', id: 'jshdo18ch1823' }"
        );
    }
    #[test]
    fn set() {
        let set = InsertData::set().push("username", "Matt");
        let set_object = InsertData::set().push("name", "John").push(
            "IdCard.info",
            IdCard {
                id: "jshdo18ch1823".to_string(),
                card_type: "temp".to_string(),
            },
        );
        assert_eq!(set.to_string().as_str(), "(username) VALUES ('Matt')");
        assert_eq!(
            set_object.to_string().as_str(),
            "(name, IdCard.info) VALUES ('John', { card_type: 'temp', id: 'jshdo18ch1823' })"
        );
    }
}
