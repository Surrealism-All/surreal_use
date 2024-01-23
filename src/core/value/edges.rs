use super::SurrrealTable;
use surrealdb::sql::{self, Dir, Id, Table, Value};

/// # 边节点
/// 生成 from dir to的结构
/// 如:
/// - surreal -> hello
/// - surreal -> surrealdb <-> user
/// - surreal:db -> user:matt
/// ## 注意
/// 区分 surrealdb::sql::Edges (两者不同，设计理念相同)
#[derive(Debug, Clone, PartialEq)]
pub struct Edges {
    /// 连接边类型
    /// - In : <-
    /// - Out : ->
    /// - Both : <->
    pub dir: Dir,
    /// 在连接边左边的表|记录
    pub from: SurrrealTable,
    /// 在连接边右边的表|记录
    pub to: SurrrealTable,
}

impl Edges {
    /// ## 创建新连接Edges
    /// ### params
    /// - from : `SurrealTable`
    /// - dir : `Dir`
    /// - to : `SurrrealTable`
    /// ### return
    /// `Edges`
    /// ### example
    /// ```
    /// let edges = Edges::new(
    ///     Edges::new("a".into(), Dir::Out, "b".into()).into(),
    ///     Dir::In,
    ///     Edges::new("c".into(),Dir::Out, "d".into()).into()
    /// );
    /// let edges_str = "a->b<-c->d";
    /// assert_eq!(edges.to_string().as_str(),edges_str);
    /// ```
    pub fn new(from: SurrrealTable, dir: Dir, to: SurrrealTable) -> Self {
        Edges { dir, from, to }
    }
    pub fn to_origin(self) -> sql::Edges {
        let table: Table = self.to.into();
        sql::Edges {
            dir: self.dir,
            from: self.from.into(),
            what: table.into(),
        }
    }
}

/// 转换`((&str, Id), Dir, (&str, Id))`
/// ```
/// let simple: Edges = (
///     ("surreal", Id::String("hello".to_string())),
///     Dir::In,
///     ("db", Id::Number(15)),
/// )
///     .into();
/// ```
impl From<((&str, Id), Dir, (&str, Id))> for Edges {
    fn from(value: ((&str, Id), Dir, (&str, Id))) -> Self {
        Edges {
            dir: value.1,
            from: value.0.into(),
            to: value.2.into(),
        }
    }
}

impl From<(&str, Dir, &str)> for Edges {
    fn from(value: (&str, Dir, &str)) -> Self {
        Edges::new(value.0.into(), value.1, value.2.into())
    }
}

/// `((("",Dir::Out,"knows"),Dir::Out,"person"),Dir::Out,&inner_where)`
// #[macro_export]
// macro_rules! tuple_to_edges {
//     // Base case for simple structure
//     (($left:expr, $dir:expr, $right:expr)) => {
//         Edges::new($left.into(), $dir, $right.into())
//     };
//     // Recursive case for nested structure
//     (($left:expr, $dir:expr, $right:expr), $($rest:tt)*) => {
//         tuple_to_edges!(($left, $dir, $right)) // Process the current level
//         tuple_to_edges!($($rest)*); // Recursively process the remaining levels
//     };
// }


impl From<(SurrrealTable, Dir, SurrrealTable)> for Edges {
    fn from(value: (SurrrealTable, Dir, SurrrealTable)) -> Self {
        Edges {
            dir: value.1,
            from: value.0,
            to: value.2,
        }
    }
}

impl From<Edges> for sql::Edges {
    fn from(value: Edges) -> Self {
        value.to_origin()
    }
}

impl From<Edges> for Value {
    fn from(value: Edges) -> Self {
        Value::Edges(Box::new(value.to_origin()))
    }
}

impl From<Edges> for sql::Cond {
    fn from(value: Edges) -> Self {
        sql::Cond(Value::from(value))
    }
}

impl ToString for Edges {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.from.to_string(),
            self.dir.to_string(),
            self.to.to_string()
        )
    }
}

#[cfg(test)]
mod test_edges {
    use super::Edges;
    use surrealdb::sql::{Dir, Id};

    #[test]
    fn complex_edges() {
        let edges = Edges::new(
            Edges::new("a".into(), Dir::Out, "b".into()).into(),
            Dir::In,
            Edges::new("c".into(), Dir::Out, "d".into()).into(),
        );
        let edges_str = "a->b<-c->d";
        assert_eq!(edges.to_string().as_str(), edges_str);
    }
    #[test]
    fn simple_str() {
        let edges = Edges::new("surreal".into(), Dir::In, "db".into());
        let edges_str = "surreal<-db";
        assert_eq!(edges_str, edges.to_string().as_str());
    }
    #[test]
    fn simple() {
        // [src/core/value/edges.rs:51] &edges = Edges {
        //     dir: In,
        //     from: Thing(
        //         Thing {
        //             tb: "surreal",
        //             id: String(
        //                 "hello",
        //             ),
        //         },
        //     ),
        //     to: Thing(
        //         Thing {
        //             tb: "db",
        //             id: Number(
        //                 15,
        //             ),
        //         },
        //     ),
        // }
        let simple: Edges = (
            ("surreal", Id::String("hello".to_string())),
            Dir::In,
            ("db", Id::Number(15)),
        )
            .into();
        let edges = Edges::new(
            ("surreal", "hello").into(),
            Dir::In,
            ("db", Id::Number(15)).into(),
        );
        assert_eq!(edges, simple);
    }
}
