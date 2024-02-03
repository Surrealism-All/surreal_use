use surrealdb::sql::{
    statements::InsertStatement, Data, Duration, Idiom, Operator, Output, Timeout, Value,
};

use crate::impl_stmt_bridge;

use super::sql::{CreateData, InsertData, SetField, SurrealTable};

use super::StmtBridge;

/// ## create INSERT statement
/// The Insert statement can be used to insert or update data into a database using the same syntax as traditional SQL Insert statements.
/// ### example for set
/// ```
/// let insert = InsertStmt::new()
/// .table("product".into())
/// .data(
///     InsertData::set()
///         .push("name", "Salesforce")
///         .push("url", "salesforce.com"),
/// )
/// .update(vec![SetField::new("tags", Some(Operator::Inc), "crm")]);
/// assert_eq!(insert.to_string().as_str(),"INSERT INTO product (name, url) VALUES ('Salesforce', 'salesforce.com') ON DUPLICATE KEY UPDATE tags += 'crm'");
/// ```
/// ### example for content
/// ```
/// #[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
/// struct Company {
///     name: String,
///     founded: String,
///     founders: Vec<Thing>,
///     tags: Vec<String>,
/// }
/// let insert = InsertStmt::new()
///     .table("company".into())
///     .data(InsertData::content(Company {
///         name: "SurrealDB".to_string(),
///         founded: "2021-09-10".to_string(),
///         founders: vec![
///             Thing {
///                 tb: "person".to_string(),
///                 id: "tobie".into(),
///             },
///             Thing {
///                 tb: "person".to_string(),
///                 id: "jaime".into(),
///             },
///         ],
///         tags: vec!["big data".to_string(), "database".to_string()],
///     }));
/// assert_eq!(insert.to_string().as_str(),"INSERT INTO company { founded: '2021-09-10', founders: [person:tobie, person:jaime], name: 'SurrealDB', tags: ['big data', 'database'] }");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct InsertStmt {
    origin: InsertStatement,
}

impl InsertStmt {
    pub fn new() -> Self {
        InsertStmt {
            origin: InsertStatement::default(),
        }
    }
    /// ## set keyword IGNORE
    /// this keyword usually not be used
    pub fn ignore(mut self) -> Self {
        self.origin.ignore = true;
        self
    }
    /// ## set table name
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.into = table.into();
        self
    }
    /// ## set create data
    /// - CONTENT
    /// - SET
    /// ### example for content
    /// ```
    /// #[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
    /// struct Company {
    ///     name: String,
    ///     founded: String,
    ///     founders: Vec<Thing>,
    ///     tags: Vec<String>,
    /// }
    /// let insert = InsertStmt::new()
    ///     .table("company".into())
    ///     .data(InsertData::content(Company {
    ///         name: "SurrealDB".to_string(),
    ///         founded: "2021-09-10".to_string(),
    ///         founders: vec![
    ///             Thing {
    ///                 tb: "person".to_string(),
    ///                 id: "tobie".into(),
    ///             },
    ///             Thing {
    ///                 tb: "person".to_string(),
    ///                 id: "jaime".into(),
    ///             },
    ///         ],
    ///         tags: vec!["big data".to_string(), "database".to_string()],
    ///     }));
    /// assert_eq!(insert.to_string().as_str(),"INSERT INTO company { founded: '2021-09-10', founders: [person:tobie, person:jaime], name: 'SurrealDB', tags: ['big data', 'database'] }");
    /// ```
    /// ### example for set
    /// ```
    /// let insert = InsertStmt::new().table("company".into()).data(
    ///     InsertData::set()
    ///         .push("name", "SurrealDB")
    ///         .push("founded", "2021-09-10"),
    /// );
    /// assert_eq!(
    ///     insert.to_string().as_str(),
    ///     "INSERT INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10')"
    /// )
    /// ```
    pub fn data(mut self, data: InsertData) -> Self {
        self.origin.data = data.into();
        self
    }
    /// ## set ON DUPLICATE KEY UPDATE sub query
    /// In the VALUES clause, existing records can be updated by specifying a clause ,ON DUPLICATE KEY UPDATE
    ///
    /// This clause also allows for increasing and decreasing numerical values, as well as adding or deleting values in an array.
    /// To increment values or add items to an array
    pub fn update(mut self, sf: Vec<SetField>) -> Self {
        let sf = CreateData::Set(sf)
            .to_set()
            .unwrap()
            .into_iter()
            .map(|x| x.to_origin())
            .collect::<Vec<(Idiom, Operator, Value)>>();
        self.origin.update.replace(Data::UpdateExpression(sf));
        self
    }
    pub fn to_origin(self) -> InsertStatement {
        self.origin
    }
    pub fn output(mut self, output: Output) -> Self {
        self.origin.output.replace(output);
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.origin.timeout = Some(Timeout(timeout));
        self
    }
    /// ## Set whether statements can be processed in parallel
    /// default close
    pub fn parallel(mut self) -> Self {
        self.origin.parallel = true;
        self
    }
}

impl ToString for InsertStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

impl_stmt_bridge!(InsertStmt, InsertStatement);

#[cfg(test)]
mod test_insert_stmt {
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::{
        statements::InsertStatement, Data, Ident, Idiom, Operator, Part, Table, Thing,
    };

    use crate::core::sql::{InsertData, SetField};

    use super::InsertStmt;

    #[test]
    fn more() {
        #[derive(Debug, Clone, Serialize, PartialEq)]
        struct Person {
            id: String,
            name: String,
            surname: String,
        }
        let insert = InsertStmt::new().data(InsertData::content(vec![
            Person {
                id: "person:jaime".to_string(),
                name: "Jaime".to_string(),
                surname: "Morgan Hitchcock".to_string(),
            },
            Person {
                id: "person:tobie".to_string(),
                name: "Tobie".to_string(),
                surname: "Morgan Hitchcock".to_string(),
            },
        ]));
        assert_eq!(insert.to_string().as_str(),"INSERT INTO NONE [{ id: s'person:jaime', name: 'Jaime', surname: 'Morgan Hitchcock' }, { id: s'person:tobie', name: 'Tobie', surname: 'Morgan Hitchcock' }]");
    }

    #[test]
    fn complex() {
        let insert = InsertStmt::new()
            .table("product".into())
            .data(
                InsertData::set()
                    .push("name", "Salesforce")
                    .push("url", "salesforce.com"),
            )
            .update(vec![SetField::new("tags", Some(Operator::Inc), "crm")]);
        assert_eq!(insert.to_string().as_str(),"INSERT INTO product (name, url) VALUES ('Salesforce', 'salesforce.com') ON DUPLICATE KEY UPDATE tags += 'crm'");
    }

    #[test]
    fn simple_set() {
        let insert = InsertStmt::new().table("company".into()).data(
            InsertData::set()
                .push("name", "SurrealDB")
                .push("founded", "2021-09-10"),
        );
        assert_eq!(
            insert.to_string().as_str(),
            "INSERT INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10')"
        )
    }

    #[test]
    fn simple_content() {
        #[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
        struct Company {
            name: String,
            founded: String,
            founders: Vec<Thing>,
            tags: Vec<String>,
        }
        let insert = InsertStmt::new()
            .table("company".into())
            .data(InsertData::content(Company {
                name: "SurrealDB".to_string(),
                founded: "2021-09-10".to_string(),
                founders: vec![
                    Thing {
                        tb: "person".to_string(),
                        id: "tobie".into(),
                    },
                    Thing {
                        tb: "person".to_string(),
                        id: "jaime".into(),
                    },
                ],
                tags: vec!["big data".to_string(), "database".to_string()],
            }));
        assert_eq!(insert.to_string().as_str(),"INSERT INTO company { founded: '2021-09-10', founders: [person:tobie, person:jaime], name: 'SurrealDB', tags: ['big data', 'database'] }");
    }

    #[test]
    fn origin() {
        let insert = InsertStatement {
            into: Table::from("person").into(),
            data: Data::ValuesExpression(vec![vec![
                (
                    Idiom(vec![
                        Part::Field(Ident("name".to_string())),
                        Part::Field(Ident("age".to_string())),
                    ]),
                    "Matt".into(),
                ),
                (
                    Idiom(vec![
                        Part::Field(Ident("name1".to_string())),
                        Part::Field(Ident("age1".to_string())),
                    ]),
                    "Matt1".into(),
                ),
            ]]),
            ignore: true,
            update: None,
            output: None,
            timeout: None,
            parallel: false,
        };
        //INSERT IGNORE INTO person (name.age, name1.age1) VALUES ('Matt', 'Matt1')
        dbg!(insert.to_string());
    }
}
