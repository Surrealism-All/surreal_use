use super::Edges;

use surrealdb::sql::{Id, Table, Thing, Value, Values};

/// # create SurrealDB Table
/// 1. Regular tables: Table
/// 2. Directly declare a table with an Id: Thing
/// 3. table relate with other table: Edges
#[derive(Debug, Clone, PartialEq)]
pub enum SurrealTable {
    // not recommend :`Strand(Strand)`
    /// normal table
    Table(Table),
    /// table with record id
    Thing(Thing),
    /// edge such as:
    /// 1. {{ATable}}->{{BTable}}->{{CTable}}
    /// 2. {{ATable}}->{{BTable}}<-{{CTable}}
    /// 3. ...
    Edges(Box<Edges>),
}

pub trait IntoTable: Sized {
    fn to_table(self) -> Value;
}

impl From<&str> for SurrealTable {
    fn from(value: &str) -> Self {
        SurrealTable::Table(value.into())
    }
}

impl From<(&str, &str)> for SurrealTable {
    fn from(value: (&str, &str)) -> Self {
        SurrealTable::Thing(value.into())
    }
}

impl From<(&str, Id)> for SurrealTable {
    fn from(value: (&str, Id)) -> Self {
        SurrealTable::Thing(value.into())
    }
}

impl From<Table> for SurrealTable {
    fn from(value: Table) -> Self {
        SurrealTable::Table(value)
    }
}

impl From<Thing> for SurrealTable {
    fn from(value: Thing) -> Self {
        SurrealTable::Thing(value)
    }
}

impl From<Edges> for SurrealTable {
    fn from(value: Edges) -> Self {
        SurrealTable::Edges(Box::new(value))
    }
}

impl SurrealTable {
    /// ## Create SurrealTable::Table
    /// This method directly passes &str to generate a table, which can have an ID or not
    /// which is a common method
    /// ### example
    /// ```
    /// let table_without_id: SurrealTable = "surreal".into();
    /// let table_with_id: SurrealTable = "surreal:use".into();
    /// assert_eq!(
    ///     table_without_id,
    ///     SurrealTable::table("surreal")
    /// );
    /// assert_eq!(
    ///     table_with_id,
    ///     SurrealTable::table("surreal:use")
    /// );
    /// ```
    pub fn table(table: &str) -> Self {
        table.into()
    }
    /// ## Create with ID :SurrealTable::Thing
    /// This method can directly display the ID of the declaration table
    /// ### example
    /// ```
    /// let table_normal = SurrealTable::table_id("surreal", "use".into());
    /// let table_number = SurrealTable::table_id("surreal", 12.into());
    /// let table_uuid = SurrealTable::table_id("surreal", Id::uuid());
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

impl ToString for SurrealTable {
    fn to_string(&self) -> String {
        match self {
            SurrealTable::Table(table) => table.to_string(),
            SurrealTable::Thing(thing) => thing.to_string(),
            SurrealTable::Edges(edges) => edges.to_string(),
        }
    }
}

impl From<SurrealTable> for Value {
    fn from(value: SurrealTable) -> Self {
        match value {
            SurrealTable::Table(table) => table.into(),
            SurrealTable::Thing(thing) => thing.into(),
            SurrealTable::Edges(edges) => edges.to_string().into(),
        }
    }
}

impl From<SurrealTable> for Values {
    fn from(value: SurrealTable) -> Self {
        Values(vec![Value::from(value)])
    }
}

impl From<SurrealTable> for Table {
    fn from(value: SurrealTable) -> Self {
        match value {
            SurrealTable::Table(table) => table,
            _ => panic!("{:#?} cannot be converted to surrealdb::sql::Table", value),
        }
    }
}

impl From<SurrealTable> for Thing {
    fn from(value: SurrealTable) -> Self {
        match value {
            SurrealTable::Thing(thing) => thing,
            _ => panic!("{:#?} cannot be converted to surrealdb::sql::Thing", value),
        }
    }
}

impl From<SurrealTable> for Edges {
    fn from(value: SurrealTable) -> Self {
        match value {
            SurrealTable::Edges(edges) => *edges,
            _ => panic!(
                "{:#?} cannot be converted to surreal_use::core::sql::Edges",
                value
            ),
        }
    }
}

#[cfg(test)]
mod test_surreal_table {
    use surrealdb::sql::{Dir, Id};

    use crate::core::sql::Edges;

    use super::SurrealTable;

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
        let edges = SurrealTable::edges(Edges::new(
            Edges::new("a".into(), Dir::Out, "b".into()).into(),
            Dir::In,
            Edges::new("c".into(), Dir::Out, "d".into()).into(),
        ));
        let edges_str = "a->b<-c->d";

        assert_eq!(edges_str, edges.to_string().as_str());
    }

    #[test]
    fn test_table_thing() {
        let table_normal = SurrealTable::table_id("surreal", "use".into());
        let table_number = SurrealTable::table_id("surreal", 12.into());
        let table_uuid = SurrealTable::table_id("surreal", Id::uuid());
        dbg!(table_normal.to_string());
        dbg!(table_number.to_string());
        dbg!(table_uuid.to_string());
    }

    #[test]
    fn test_table() {
        let table_without_id: SurrealTable = "surreal".into();
        let table_with_id: SurrealTable = "surreal:use".into();
        assert_eq!(table_without_id, SurrealTable::table("surreal"));
        assert_eq!(table_with_id, SurrealTable::table("surreal:use"));
    }

    #[test]
    fn test_table_str() {
        let table_without_id: SurrealTable = "surreal".into();
        let table_with_id: SurrealTable = "surreal:use".into();
        assert_eq!(table_without_id.to_string(), String::from("surreal"));
        assert_eq!(table_with_id.to_string(), String::from("`surreal:use`"));
    }
}
