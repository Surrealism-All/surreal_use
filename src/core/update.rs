use surrealdb::sql::{statements::UpdateStatement, Duration, Output, Timeout};

use crate::impl_stmt_bridge;

use super::sql::{Cond, SurrealTable, UpdateData};

use super::StmtBridge;

/// ## 更新UPDATE语句
///
/// ### example for set
/// ```
/// let update = UpdateStmt::new()
/// .only()
/// .table(SurrealTable::table_id("person", "tobie".into()))
/// .data(
///     UpdateData::set()
///         .push(SetField::new("name", None, "Tobie"))
///         .push(SetField::new("company", None, "SurrealDB"))
///         .push(SetField::new(
///             "skills",
///             None,
///             vec!["Rust".to_string(), "Go".to_string()],
///         )),
/// );
/// assert_eq!(update.to_string().as_str(), "UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go']");
/// ```
/// ### example for content
/// ```
/// #[derive(Clone, Debug, PartialEq, Serialize)]
/// struct Person {
///     name: String,
///     company: String,
///     skills: Vec<String>,
/// }
/// let update = UpdateStmt::new()
///     .table("person".into())
///     .data(UpdateData::content(Person {
///         name: "Tobie".to_string(),
///         company: "SurrealDB".to_string(),
///         skills: vec![
///             "Rust".to_string(),
///             "Go".to_string(),
///             "JavaScript".to_string(),
///         ],
///     }));
/// assert_eq!(update.to_string().as_str(),"UPDATE person CONTENT { company: 'SurrealDB', name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }");
/// ```
/// ### example for merge
/// ```
/// #[derive(Clone,Debug,PartialEq,Serialize)]
/// struct Marketing{
///     marketing:bool
/// }
/// #[derive(Clone,Debug,PartialEq,Serialize)]
/// struct Person{
///     settings : Marketing
/// }
/// let update = UpdateStmt::new()
/// .table(("person","tobie").into())
/// .data(UpdateData::merge(Person{
///     settings: Marketing{
///         marketing : true
///     }
/// }));
/// assert_eq!(update.to_string().as_str(),"UPDATE person:tobie MERGE { settings: { marketing: true } }");
/// ```
/// ### example for patch
/// ```
/// let update = UpdateStmt::new()
/// .table(("person","tobie").into())
/// .data(UpdateData::patch(
///     vec![PatchOp::add("Engineering", true)]
/// ));
/// assert_eq!(update.to_string().as_str(),"UPDATE person:tobie PATCH [{ op: 'add', path: 'Engineering', value: true }]");
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct UpdateStmt {
    origin: UpdateStatement,
}

impl UpdateStmt {
    pub fn new() -> Self {
        UpdateStmt {
            origin: UpdateStatement::default(),
        }
    }
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    pub fn data(mut self, data: UpdateData) -> Self {
        self.origin.data.replace(data.into());
        self
    }
    pub fn cond(mut self, cond: Cond) -> Self {
        self.origin.cond.replace(cond.to_origin());
        self
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

impl ToString for UpdateStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

impl_stmt_bridge!(UpdateStmt, UpdateStatement);

#[cfg(test)]
mod test_update_stmt {
    use serde::Serialize;
    use surrealdb::sql::Operator;

    use crate::core::sql::{PatchOp, SetField, SurrealTable, UpdateData};

    use super::UpdateStmt;

    #[test]
    fn patch() {
        let update = UpdateStmt::new()
            .table(("person", "tobie").into())
            .data(UpdateData::patch(vec![PatchOp::add("Engineering", true)]));
        assert_eq!(
            update.to_string().as_str(),
            "UPDATE person:tobie PATCH [{ op: 'add', path: 'Engineering', value: true }]"
        );
    }

    #[test]
    fn merge() {
        #[derive(Clone, Debug, PartialEq, Serialize)]
        struct Marketing {
            marketing: bool,
        }
        #[derive(Clone, Debug, PartialEq, Serialize)]
        struct Person {
            settings: Marketing,
        }
        let update = UpdateStmt::new()
            .table(("person", "tobie").into())
            .data(UpdateData::merge(Person {
                settings: Marketing { marketing: true },
            }));
        assert_eq!(
            update.to_string().as_str(),
            "UPDATE person:tobie MERGE { settings: { marketing: true } }"
        );
    }

    #[test]
    fn simple_content() {
        #[derive(Clone, Debug, PartialEq, Serialize)]
        struct Person {
            name: String,
            company: String,
            skills: Vec<String>,
        }
        let update = UpdateStmt::new()
            .table("person".into())
            .data(UpdateData::content(Person {
                name: "Tobie".to_string(),
                company: "SurrealDB".to_string(),
                skills: vec![
                    "Rust".to_string(),
                    "Go".to_string(),
                    "JavaScript".to_string(),
                ],
            }));
        assert_eq!(update.to_string().as_str(),"UPDATE person CONTENT { company: 'SurrealDB', name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }");
    }

    #[test]
    fn simple_only() {
        let update = UpdateStmt::new()
            .only()
            .table(SurrealTable::table_id("person", "tobie".into()))
            .data(
                UpdateData::set()
                    .push(SetField::new("name", None, "Tobie"))
                    .push(SetField::new("company", None, "SurrealDB"))
                    .push(SetField::new(
                        "skills",
                        None,
                        vec!["Rust".to_string(), "Go".to_string()],
                    )),
            );
        assert_eq!(update.to_string().as_str(), "UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go']");
    }

    #[test]
    fn simple() {
        let update = UpdateStmt::new()
            .table("person".into())
            .data(UpdateData::set().push(SetField::new(
                "skill",
                Some(Operator::Inc),
                vec!["breathing".to_string()],
            )));
        assert_eq!(
            update.to_string().as_str(),
            "UPDATE person SET skill += ['breathing']"
        );
    }
}
