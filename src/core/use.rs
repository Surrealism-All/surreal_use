use super::StmtBridge;
use serde::{Deserialize, Serialize};
use surrealdb::sql::statements::UseStatement;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct UseStmt {
    origin: UseStatement,
}

impl UseStmt {
    /// 创建实例
    pub fn new() -> Self {
        UseStmt {
            origin: UseStatement { ns: None, db: None },
        }
    }
    /// 设置命名空间
    pub fn ns(mut self, ns: &str) -> Self {
        self.origin.ns = Some(ns.to_string());
        self
    }
    /// 设置数据库
    pub fn db(mut self, db: &str) -> Self {
        self.origin.db = Some(db.to_string());
        self
    }
}

impl StmtBridge for UseStmt {
    type OriginType = UseStatement;

    fn to_origin(self) -> Self::OriginType {
        self.origin
    }
    fn origin(&self) -> &Self::OriginType {
        &self.origin
    }
}

impl ToString for UseStmt {
    fn to_string(&self) -> String {
        String::from(&self.origin.to_string())
    }
}

#[cfg(test)]
mod test_use_stmt {
    use super::*;

    //测试从结构体转为语句
    #[test]
    fn test_to_string() {
        let use_stmt = UseStmt::new().ns("test_ns").db("test_db");
        let use_str = "USE NS test_ns DB test_db";
        assert_eq!(use_stmt.to_string(), use_str);
    }
}
