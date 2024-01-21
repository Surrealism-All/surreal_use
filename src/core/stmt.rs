use super::use_stmt::UseStmt;
pub struct Stmt;

impl Stmt {
    /// ## 构建Use语句
    /// 由于use作为rust的关键字所以这里增加s作为区分
    pub fn uses() -> UseStmt {
        UseStmt::new()
    }
}

#[cfg(test)]
mod test_stmt {
    use super::Stmt;

    #[test]
    fn test_use() {
        let use_s = Stmt::uses().ns("surreal").db("use");
        let use_str = "USE NS surreal DB use";
        assert_eq!(use_str, &use_s.to_string());
    }
}
