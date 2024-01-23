
use std::error::Error;

use super::Edges;

use surrealdb::sql::{Id, Table, Thing, Value, Values};

/// # SurrealDB表的表示方式
/// 1. 常规通过str生成的表： Table
/// 2. 直接声明带有Id的表：Thing
/// 3. 表示表的连接：Edges 
#[derive(Debug, Clone, PartialEq)]
pub enum SurrrealTable {
    // 不建议使用Strand(Strand)
    /// 表类型,仅有表名没有ID
    Table(Table),
    /// 表+ID类型，有表名+记录ID
    Thing(Thing),
    /// 连接边类型:
    /// 1. {{ATable}}->{{BTable}}->{{CTable}}
    /// 2. {{ATable}}->{{BTable}}<-{{CTable}}
    /// 3. ...
    Edges(Box<Edges>),
}

pub trait IntoTable: Sized {
    fn to_table(self) -> Value;
}

impl From<&str> for SurrrealTable {
    fn from(value: &str) -> Self {
        SurrrealTable::Table(value.into())
    }
}

impl From<(&str, &str)> for SurrrealTable {
    fn from(value: (&str, &str)) -> Self {
        SurrrealTable::Thing(value.into())
    }
}

impl From<(&str, Id)> for SurrrealTable {
    fn from(value: (&str, Id)) -> Self {
        SurrrealTable::Thing(value.into())
    }
}

impl From<Table> for SurrrealTable {
    fn from(value: Table) -> Self {
        SurrrealTable::Table(value)
    }
}

impl From<Thing> for SurrrealTable {
    fn from(value: Thing) -> Self {
        SurrrealTable::Thing(value)
    }
}

impl From<Edges> for SurrrealTable {
    fn from(value: Edges) -> Self {
        SurrrealTable::Edges(Box::new(value))
    }
}

impl SurrrealTable {
    /// ## 创建SurrealTable::Table
    /// 这种方式直接传入&str生成表，可以带有ID也可以不带有，是一种常规方式
    /// ### example
    /// ```
    /// let table_without_id: SurrrealTable = "surreal".into();
    /// let table_with_id: SurrrealTable = "surreal:use".into();
    /// assert_eq!(
    ///     table_without_id,
    ///     SurrrealTable::table("surreal")
    /// );
    /// assert_eq!(
    ///     table_with_id,
    ///     SurrrealTable::table("surreal:use")
    /// );
    /// ```
    pub fn table(table: &str) -> Self {
        table.into()
    }
    /// ## 创建带有ID的SurrealTable::Thing
    /// 这种方式可直接显示声明表的ID
    /// ### example
    /// ```
    /// let table_normal = SurrrealTable::table_id("surreal", "use".into());
    /// let table_number = SurrrealTable::table_id("surreal", 12.into());
    /// let table_uuid = SurrrealTable::table_id("surreal", Id::uuid());
    /// dbg!(table_normal.to_string());
    /// dbg!(table_number.to_string());
    /// dbg!(table_uuid.to_string());
    /// ```
    pub fn table_id(name: &str, id: Id) -> Self {
        let thing = Thing {
            tb: String::from(name),
            id,
        };
        thing.into()
    }
    pub fn edges(edges: Edges) -> Self {
        edges.into()
    }
}

impl ToString for SurrrealTable {
    fn to_string(&self) -> String {
        match self {
            SurrrealTable::Table(table) => table.to_string(),
            SurrrealTable::Thing(thing) => thing.to_string(),
            SurrrealTable::Edges(edges) => edges.to_string(),
        }
    }
}


impl From<SurrrealTable> for Value {
    fn from(value: SurrrealTable) -> Self {
        match value {
            SurrrealTable::Table(table) => table.into(),
            SurrrealTable::Thing(thing) => thing.into(),
            SurrrealTable::Edges(edges) => edges.to_string().into() ,
        }
    }
}

impl From<SurrrealTable> for Values{
    fn from(value: SurrrealTable) -> Self {
        Values(vec![Value::from(value)])
    }
}

impl From<SurrrealTable> for Table{
    fn from(value: SurrrealTable) -> Self {
        match value {
            SurrrealTable::Table(table) => table,
            _ => panic!("{:#?} cannot be converted to surrealdb::sql::Table",value),
        }
    }
}

impl From<SurrrealTable> for Thing{
    fn from(value: SurrrealTable) -> Self {
        match value {
            SurrrealTable::Thing(thing) => thing,
            _ => panic!("{:#?} cannot be converted to surrealdb::sql::Thing",value),
        }
    }
}

impl From<SurrrealTable> for Edges {
    fn from(value: SurrrealTable) -> Self {
        match value {
            SurrrealTable::Edges(edges) => *edges,
            _ => panic!("{:#?} cannot be converted to surreal_use::core::value::Edges",value),
        }
    }
}

#[cfg(test)]
mod test_surreal_table {
    use surrealdb::sql::{Dir, Table, Id};

    use crate::core::value::Edges;

    use super::SurrrealTable;

    #[test]
    fn test_table_edges() {
        // [src/core/value/table.rs:105] edges = Edges(
        //     Edges {
        //         dir: In,
        //         from: Edges(
        //             Edges {
        //                 dir: Out,
        //                 from: Table(
        //                     Table(
        //                         "a",
        //                     ),
        //                 ),
        //                 to: Table(
        //                     Table(
        //                         "b",
        //                     ),
        //                 ),
        //             },
        //         ),
        //         to: Edges(
        //             Edges {
        //                 dir: Out,
        //                 from: Table(
        //                     Table(
        //                         "c",
        //                     ),
        //                 ),
        //                 to: Table(
        //                     Table(
        //                         "d",
        //                     ),
        //                 ),
        //             },
        //         ),
        //     },
        // )
        let edges = SurrrealTable::edges(Edges::new(
            Edges::new("a".into(), Dir::Out, "b".into()).into(),
            Dir::In,
            Edges::new("c".into(), Dir::Out, "d".into()).into(),
        ));
        let edges_str = "a->b<-c->d";

        assert_eq!(edges_str, edges.to_string().as_str());
    }

    #[test]
    fn test_table_thing() {
        let table_normal = SurrrealTable::table_id("surreal", "use".into());
        let table_number = SurrrealTable::table_id("surreal", 12.into());
        let table_uuid = SurrrealTable::table_id("surreal", Id::uuid());
        dbg!(table_normal.to_string());
        dbg!(table_number.to_string());
        dbg!(table_uuid.to_string());
    }

    #[test]
    fn test_table() {
        let table_without_id: SurrrealTable = "surreal".into();
        let table_with_id: SurrrealTable = "surreal:use".into();
        assert_eq!(
            table_without_id,
            SurrrealTable::table("surreal")
        );
        assert_eq!(
            table_with_id,
            SurrrealTable::table("surreal:use")
        );
    }
    
    #[test]
    fn test_table_str() {
        let table_without_id: SurrrealTable = "surreal".into();
        let table_with_id: SurrrealTable = "surreal:use".into();
        assert_eq!(table_without_id.to_string(), String::from("surreal"));
        assert_eq!(table_with_id.to_string(), String::from("`surreal:use`"));
    }
}
