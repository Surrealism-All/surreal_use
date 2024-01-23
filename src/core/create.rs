use surrealdb::sql::{statements::CreateStatement, Data, Duration, Output, Timeout};

use super::value::SurrrealTable;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateStmt {
    origin: CreateStatement,
}

// 	pub data: Option<Data>,

impl CreateStmt {
    pub fn new() -> Self {
        CreateStmt {
            origin: CreateStatement::default(),
        }
    }
    pub fn table(mut self, table: SurrrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    pub fn data(mut self, data: Data) -> Self {
        self.origin.data.replace(data);
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
