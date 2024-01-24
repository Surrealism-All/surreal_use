mod cond;
mod create;
mod edges;
mod field;
mod set_field;
mod table;

pub use cond::Cond;
pub use edges::Edges;
pub use field::Field;
pub use table::SurrrealTable;
pub use create::CreateData;
pub use set_field::SetField;

#[cfg(test)]
mod test_value {
    use std::{collections::BTreeMap, time};

    use surrealdb::sql::{
        Array, Cast, Datetime, Duration, Ident, Idiom, Mock, Object, Param, Part, Strand, Table,
        Thing, Value,
    };

    #[test]
    fn none() {
        let none = Value::None;
        assert_eq!(none.to_string().as_str(), "NONE");
    }
    #[test]
    fn null() {
        let null = Value::Null;
        assert_eq!(null.to_string().as_str(), "NULL");
    }
    #[test]
    fn number() {
        let number = Value::Number(16.into());
        assert_eq!(number.to_string().as_str(), "16");
    }
    #[test]
    fn bool() {
        let bool = Value::Bool(true);
        assert_eq!(bool.to_string().as_str(), "true");
    }
    #[test]
    fn strand() {
        let strand1 = Value::Strand(Strand(String::from("surreal")));
        let strand2 = Value::Strand(Strand(String::from("surreal:use")));
        assert_eq!(strand1.to_string().as_str(), "'surreal'");
        assert_eq!(strand2.to_string().as_str(), "s'surreal:use'");
    }
    #[test]
    fn duration() {
        let duration = Value::Duration(Duration(time::Duration::new(7711, 1)));
        assert_eq!(duration.to_string().as_str(), "2h8m31s1ns");
    }
    #[test]
    fn datetime() {
        let datetime = Value::Datetime(Datetime::default());
        //'2024-01-23T06:27:14.086126Z'
        dbg!(datetime.to_string().as_str());
    }
    #[test]
    fn uuid() {
        let uuid = Value::Uuid(surrealdb::sql::Uuid::new());
        //'018d3500-b7d8-7398-86eb-d9ba80c3fe5f'
        dbg!(uuid.to_string());
    }
    #[test]
    fn array() {
        let arr = Value::Array(Array(vec![17.into(), "jhell".into()]));
        assert_eq!(arr.to_string().as_str(), "[17, 'jhell']");
    }
    #[test]
    fn object() {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        map.insert("a".to_owned(), 1.into());
        map.insert("b".to_owned(), "2".into());
        let object = Value::Object(Object(map));
        assert_eq!(object.to_string().as_str(), "{ a: 1, b: '2' }");
    }
    #[test]
    fn bytes() {
        let b_str = String::from("hello").into_bytes();
        let b = Value::Bytes(b_str.into());
        assert_eq!(
            b.to_string().as_str(),
            "encoding::base64::decode(\"aGVsbG8\")"
        );
    }
    #[test]
    fn table() {
        let table = Value::Table(Table("user".to_string()));
        assert_eq!(table.to_string().as_str(), "user");
    }
    #[test]
    fn thing() {
        let thing = Value::Thing(Thing {
            tb: "surreal".to_string(),
            id: "use".into(),
        });
        assert_eq!(thing.to_string().as_str(), "surreal:use")
    }
    #[test]
    fn param() {
        let ident = Ident("user".to_string()).to_raw();
        let param = Value::Param(Param(Ident("name".to_string())));
        assert_eq!(param.to_string().as_str(), "$name");
        assert_eq!(ident.as_str(), "user");
    }
    #[test]
    fn idiom() {
        let idiom = Value::Idiom(Idiom(vec![
            Part::All,
            Part::Flatten,
            Part::First,
            Part::Last,
            Part::Index(surrealdb::sql::Number::Float(32.92)),
        ]));
        //[*]…[0][$][32.92f]
        dbg!(idiom.to_string());
    }
    #[test]
    fn mock() {
        let mock_count = Value::Mock(Mock::Count("name".to_string(), 64));
        let mock_range = Value::Mock(Mock::Range("age".to_string(), 18, 88));
        assert_eq!(mock_count.to_string().as_str(), "|name:64|");
        assert_eq!(mock_range.to_string().as_str(), "|age:18..88|");
    }
    #[test]
    fn cast() {
        let cast = Value::Cast(Box::new(Cast(surrealdb::sql::Kind::Any, "hello".into())));
        dbg!(cast.to_string());
    }
}
