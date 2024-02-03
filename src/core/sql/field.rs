use surrealdb::sql::{self, Fields, Ident, Idiom, Part};

/// ## Field
/// be used in many statements such as:
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
/// //--------field----------
/// //     ⇩⇩⇩⇩⇩⇩⇩⇩⇩
/// SELECT user.name FROM user;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Field(sql::Field);

impl Default for Field {
    fn default() -> Self {
        Self(sql::Field::default())
    }
}

impl Field {
    /// ## build All Field
    /// `*`
    /// ### example
    /// ```
    /// let f_all = Field::all();
    /// assert_eq!(f_all.to_string().as_str(),"*");
    /// ```
    pub fn all() -> Self {
        Field(sql::Field::All)
    }

    /// ## build normal Field
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
            Some(a) => Some(str_to_idiom(a)),
            None => None,
        };
        let expr = str_to_idiom(field).into();
        Field(sql::Field::Single { expr, alias })
    }
    pub fn signle_value(field: Idiom, r#as: Option<Idiom>) -> Self {
        Field(sql::Field::Single {
            expr: sql::Value::Idiom(field),
            alias: r#as,
        })
    }
    /// ## new instance Field
    /// This method has no aliases
    pub fn new(field: &str) -> Self {
        let expr = str_to_idiom(field).into();
        Field(sql::Field::Single { expr, alias: None })
    }
    pub fn to_origin(self) -> sql::Field {
        self.0
    }
    pub fn to_idiom(self) -> Idiom {
        sql::Value::from(self).to_idiom()
    }
    pub fn from_vec(value: Vec<&str>) -> Vec<Field> {
        value
            .into_iter()
            .map(|x| Field::from(x))
            .collect::<Vec<Field>>()
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

impl From<Vec<&str>> for Field {
    fn from(value: Vec<&str>) -> Self {
        Field::signle_value(vec_to_idiom(value), None)
    }
}

impl From<Vec<String>> for Field {
    fn from(value: Vec<String>) -> Self {
        Field::from(value.iter().map(|x| x.as_str()).collect::<Vec<&str>>())
    }
}

impl From<Vec<Part>> for Field {
    fn from(value: Vec<Part>) -> Self {
        Field::signle_value(value.into(), None)
    }
}

/// 将a.b.c类&str 转为 Field
/// 这类转换不会存在AS
impl From<&str> for Field {
    fn from(value: &str) -> Self {
        Field::signle_value(str_to_idiom(value), None)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Field::from(value.as_str())
    }
}

/// ⚠️ 将Field转为Value时会丢弃AS
impl From<Field> for sql::Value {
    fn from(value: Field) -> Self {
        // sql::Value::Idiom(vec![Part::Field(value.to_string().into())].into())
        match value.to_origin() {
            sql::Field::All => sql::Value::Idiom("*".to_string().into()),
            sql::Field::Single { expr, alias: _ } => expr,
        }
    }
}

/// 切分`.`生成父子结构
fn str_to_idiom(value: &str) -> Idiom {
    let values = value.split('.').collect::<Vec<&str>>();
    vec_to_idiom(values)
}

// vec -> Idiom
fn vec_to_idiom(value: Vec<&str>) -> Idiom {
    let parts = value
        .into_iter()
        .map(|x| Part::Field(Ident::from(x)))
        .collect::<Vec<Part>>();
    parts.into()
}

#[cfg(test)]
mod test_field {
    use surrealdb::sql::{Output, Part, Value};

    use super::Field;
    #[test]
    fn test_dot() {
        let f = Field::single("a.b", None);
        assert_eq!(f.to_string().as_str(), "a.b");
    }
    #[test]
    fn to_value() {
        let parts = vec![
            Part::Field("a".to_string().into()),
            Part::Field("b".to_string().into()),
        ];
        let f: Field = parts.into();
        let v: Value = f.into();
        assert_eq!(v.to_string().as_str(), "a.b");
    }
    #[test]
    fn from_vec_part() {
        let parts = vec![
            Part::Field("a".to_string().into()),
            Part::Field("b".to_string().into()),
        ];
        let f: Field = parts.into();
        assert_eq!(f.to_string().as_str(), "a.b");
    }

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
