use surrealdb::sql::{self, Fields, Ident, Idiom, Part, Table};

/// ## 字段
/// 常用于语句中
/// ```
/// //---field-----
/// //     ⇩
/// SELECT * FROM user;
/// //----------field-------------
/// //     ⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩⇩
/// SELECT name AS username FROM user;
/// //----field-----
/// //      ⇩
/// WHERE userId = "001"
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Field(sql::Field);

impl Field {
    /// ## 设置All Field
    /// `*`
    /// ### example
    /// ```
    /// let f_all = Field::all();
    /// assert_eq!(f_all.to_string().as_str(),"*");
    /// ```
    pub fn all() -> Self {
        Field(sql::Field::All)
    }

    /// ## 设置常规Field
    /// 1. field
    /// 2. field AS alias
    /// ### example
    /// ```
    /// // no alias
    /// let f_single = Field::single("name", None);
    /// assert_eq!(f_single.to_string().as_str(),"name");
    /// // has alias
    /// let f_single = Field::single("name", Some("username"));
    /// assert_eq!(f_single.to_string().as_str(),"name AS username");
    /// ```
    pub fn single(field: &str, r#as: Option<&str>) -> Self {
        let alias = match r#as {
            Some(a) => Some(Idiom(vec![Part::Field(Ident(a.to_string()))])),
            None => None,
        };

        Field(sql::Field::Single {
            expr: Table(field.to_string()).into(),
            alias,
        })
    }
    pub fn to_origin(self) -> sql::Field {
        self.0
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Field> for sql::Output {
    fn from(value: Field) -> Self {
        sql::Output::Fields(Fields(vec![value.to_origin()], false))
    }
}

#[cfg(test)]
mod test_field {
    use surrealdb::sql::Output;

    use super::Field;

    #[test]
    fn all() {
        let f_all = Field::all();
        assert_eq!(f_all.to_string().as_str(), "*");
    }
    #[test]
    fn single_no_as() {
        let f_single = Field::single("name", None);
        assert_eq!(f_single.to_string().as_str(), "name");
    }
    #[test]
    fn single_as() {
        let f_single = Field::single("name", Some("username"));
        assert_eq!(f_single.to_string().as_str(), "name AS username");
    }
    #[test]
    fn to_output() {
        let f1 = Field::single("name", Some("username"));
        let f2 = Field::single("name", None);
        assert_eq!(
            Output::from(f1).to_string().as_str(),
            "RETURN name AS username"
        );
        assert_eq!(Output::from(f2).to_string().as_str(), "RETURN name");
    }
}
