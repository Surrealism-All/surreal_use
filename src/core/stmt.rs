use super::r#use::UseStmt;
pub struct Stmt;

impl Stmt {
    /// ## 构建Use语句
    /// 由于use作为rust的关键字所以这里增加r#解决
    pub fn r#use() -> UseStmt {
        UseStmt::new()
    }
}

#[cfg(test)]
mod test_stmt {
    use super::Stmt;

    #[test]
    fn test_use() {
        let use_s = Stmt::r#use().ns("surreal").db("use");
        let use_str = "USE NS surreal DB use";
        assert_eq!(use_str, &use_s.to_string());
    }
}
