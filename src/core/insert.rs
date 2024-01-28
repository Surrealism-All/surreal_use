use surrealdb::sql::{
    statements::InsertStatement, Data, Duration, Idiom, Operator, Output, Timeout, Value,
};

use crate::impl_stmt_bridge;

use super::sql::{CreateData, InsertData, SetField, SurrealTable};

use super::StmtBridge;

/// ## 插入数据INSERT语句
/// INSERT语句可用于将数据插入或更新到数据库中，使用与传统SQL Insert语句相同的语句语法。
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
    /// ## 设置IGNORE关键字
    /// 该关键字常常被忽略
    pub fn ignore(mut self) -> Self {
        self.origin.ignore = true;
        self
    }
    /// ## 设置表名
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.into = table.into();
        self
    }
    /// ## 设置更新条目
    /// - CONTENT 方式
    /// - SET 方式
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
    /// ## 设置ON DUPLICATE KEY UPDATE子句
    /// VALUES子句中可以通过指定子句来更新已存在的记录ON DUPLICATE KEY UPDATE。
    ///
    /// 该子句还允许递增和递减数值，以及在数组中添加或删除值。要递增数值或向数组添加项目
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
    /// ## 设置语句是否可以并行处理
    /// 默认关闭
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
