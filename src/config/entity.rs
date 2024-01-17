use serde::{
    de::Visitor,
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    marker::PhantomData,
};
use surrealdb::opt::auth::Credentials;
// use surrealdb::opt::auth::{Credentials, Database, Namespace, Root, Scope};

#[derive(Debug, Serialize, Clone)]
pub struct SurrealConfig<'a, P> {
    endpoint: String,
    port: u32,
    credential: SurrealCredentials<'a, P>,
}

// impl  {

// }

/// Root方式登录凭证的扩展
/// 使用智能指针Cow
/// 用于在运行时决定是否拥有所有权的字符串或切片
/// Cow 可以包含 Borrowed（引用）或 Owned（拥有所有权）的数据，并根据需要进行转换
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Root<'a> {
    /// The username of the root user
    #[serde(rename = "user")]
    username: Cow<'a, str>,
    /// The password of the root user
    #[serde(rename = "pass")]
    password: Cow<'a, str>,
}

/// 默认的Root结构体的构造
impl<'a> Default for Root<'a> {
    fn default() -> Self {
        Self {
            username: Cow::from("root"),
            password: Cow::from("root"),
        }
    }
}

impl<'a> Root<'a> {
    /// 创建一个Root结构体
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Self {
            username: Cow::from(username),
            password: Cow::from(password),
        }
    }
    pub fn keys() -> Vec<&'a str> {
        vec!["user", "pass"]
    }
}

//实现ToString trait赋予转换String的能力
impl<'a> ToString for Root<'a> {
    fn to_string(&self) -> String {
        to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Namespace<'a> {
    /// The namespace the user has access to
    #[serde(rename = "ns")]
    namespace: Cow<'a, str>,
    /// The username of the namespace user
    #[serde(rename = "user")]
    username: Cow<'a, str>,
    /// The password of the namespace user
    #[serde(rename = "pass")]
    password: Cow<'a, str>,
}

impl<'a> Namespace<'a> {
    pub fn new(username: &'a str, password: &'a str, ns: &'a str) -> Self {
        Self {
            namespace: Cow::from(ns),
            username: Cow::from(username),
            password: Cow::from(password),
        }
    }
    pub fn keys() -> Vec<&'a str> {
        vec!["user", "pass", "ns"]
    }
}

impl<'a> ToString for Namespace<'a> {
    fn to_string(&self) -> String {
        to_string(self)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Database<'a> {
    /// The namespace the user has access to
    #[serde(rename = "ns")]
    pub namespace: Cow<'a, str>,
    /// The database the user has access to
    #[serde(rename = "db")]
    pub database: Cow<'a, str>,
    /// The username of the database user
    #[serde(rename = "user")]
    pub username: Cow<'a, str>,
    /// The password of the database user
    #[serde(rename = "pass")]
    pub password: Cow<'a, str>,
}

impl<'a> Database<'a> {
    pub fn new(username: &'a str, password: &'a str, ns: &'a str, db: &'a str) -> Self {
        Self {
            namespace: Cow::from(ns),
            username: Cow::from(username),
            password: Cow::from(password),
            database: Cow::from(db),
        }
    }
    pub fn keys() -> Vec<&'a str> {
        vec!["user", "pass", "ns", "db"]
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Scope<'a, P> {
    /// The namespace the user has access to
    #[serde(rename = "ns")]
    pub namespace: Cow<'a, str>,
    /// The database the user has access to
    #[serde(rename = "db")]
    pub database: Cow<'a, str>,
    /// The scope to use for signin and signup
    #[serde(rename = "sc")]
    pub scope: Cow<'a, str>,
    /// The additional params to use
    #[serde(flatten)]
    pub params: P,
}

// impl<'a, P> Deserialize<'a> for Scope<'a, P> {}

// impl<'a, P> Deserialize<'a> for Scope<'a, P>
// where
//     P: Deserialize<'a>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'a>,
//     {
//         //使用PhantomData
//         //定义了一个 ScopeVisitor，并在 visit_map 方法中处理了结构体的字段。
//         //我们使用 Cow 类型来表示字符串，使用 PhantomData 来表示参数类型 P
//         //并通过 deserialize_map 调用 ScopeVisitor 来完成整个序列化过程。
//         // 请注意，使用了一个内部的结构体 AdditionalParams 来表示额外的参数。
//         //你需要为 Scope 结构体的 params 字段提供一个默认值，这样即使没有额外的参数，也可以正常反序列化。
//         //在这里，我们使用 unwrap_or_default 来提供默认值。
//         //如果你的实际应用场景允许 params 字段为空，则可以考虑使用 Option<P>。
//         struct ScopeVisitor<P>(PhantomData<P>);
//         //Visitor这个特性可以遍历反序列化程序的访问者。
//         impl<'a, P> Visitor<'a> for ScopeVisitor<P>
//         where
//             P: Deserialize<'a>,
//         {
//             type Value = Scope<'a, P>;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct Scope")
//             }
//             fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'a>,
//             {
//                 let mut namespace: Option<Cow<'a, str>> = None;
//                 let mut database: Option<Cow<'a, str>> = None;
//                 let mut scope: Option<Cow<'a, str>> = None;
//                 let mut params: Option<P> = None;

//                 let mut map = map;
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         "ns" => {
//                             if namespace.is_some() {
//                                 return Err(de::Error::duplicate_field("ns"));
//                             }
//                             namespace = Some(map.next_value()?);
//                         }
//                         "db" => {
//                             if database.is_some() {
//                                 return Err(de::Error::duplicate_field("db"));
//                             }
//                             database = Some(map.next_value()?);
//                         }
//                         "sc" => {
//                             if scope.is_some() {
//                                 return Err(de::Error::duplicate_field("sc"));
//                             }
//                             scope = Some(map.next_value()?);
//                         }
//                         _ => {
//                             // Flatten the additional params
//                             if params.is_none() {
//                                 params = Some(map.next_value()?);
//                             } else {
//                                 return Err(de::Error::unknown_field(key, &["ns", "db", "sc"]));
//                             }
//                         }
//                     }
//                 }
//                 Ok(Scope {
//                     namespace: namespace.ok_or_else(|| de::Error::missing_field("ns"))?,
//                     database: database.ok_or_else(|| de::Error::missing_field("db"))?,
//                     scope: scope.ok_or_else(|| de::Error::missing_field("sc"))?,
//                     params: params.unwrap(),
//                 })
//             }
//         }
//         deserializer.deserialize_map(ScopeVisitor(std::marker::PhantomData))
//     }
// }

impl<'a, P> Scope<'a, P>
where
    P: Serialize + Deserialize<'static>,
{
    pub fn new(ns: &'a str, db: &'a str, scope: &'a str, params: P) -> Self {
        Self {
            namespace: Cow::from(ns),
            database: Cow::from(db),
            scope: Cow::from(scope),
            params,
        }
    }
    pub fn keys() -> Vec<&'a str> {
        let mut res = vec!["ns", "db", "sc"];
        //使用serde_json将params字段的类型转为map计算key的数量

        res
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum SurrealCredentials<'a, P> {
    Root(Root<'a>),
    Namespace(Namespace<'a>),
    Database(Database<'a>),
    Scope(Scope<'a, P>),
}

impl<'a, P> From<Value> for SurrealCredentials<'a, P>
where
    P: Serialize + DeserializeOwned,
{
    fn from(value: Value) -> Self {
        Self::deserialize(value)
    }
}

impl<'a, P> SurrealCredentials<'a, P>
where
    P: Serialize + DeserializeOwned,
{
    pub fn deserialize(value: Value) -> Self {
        let trans_value = value
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
            .collect::<Vec<(String, String)>>();
        let trans_keys = trans_value
            .iter()
            .map(|(k, _v)| k.as_str())
            .collect::<Vec<&str>>();
        match trans_keys.len() {
            2 => {
                //判断参数
                if to_hashset(Root::keys()).eq(&to_hashset(trans_keys)) {
                    return SurrealCredentials::Root(
                        serde_json::from_value::<Root>(value).unwrap(),
                    );
                } else {
                    panic!("SurrealDB Configuration Error : Credential Root should use `user` and `pass`");
                }
            }
            3 => {
                if to_hashset(Namespace::keys()).eq(&to_hashset(trans_keys)) {
                    return SurrealCredentials::Namespace(
                        serde_json::from_value::<Namespace>(value).unwrap(),
                    );
                } else {
                    panic!("SurrealDB Configuration Error : Credential Namespace should use `user` , `pass` , `ns`");
                }
            }
            4 => {
                if to_hashset(Database::keys()).eq(&to_hashset(trans_keys)) {
                    return SurrealCredentials::Database(
                        serde_json::from_value::<Database>(value).unwrap(),
                    );
                } else {
                    panic!("SurrealDB Configuration Error : Credential Namespace should use `user` , `pass` , `ns`,`db`");
                }
            }
            _ => {
                return SurrealCredentials::Scope(
                    serde_json::from_value::<Scope<'a, P>>(value).unwrap(),
                );
            }
        }
    }
}

/// 通过serde_json帮助转为String字符串
fn to_string<T>(value: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(value).unwrap()
}

fn to_hashset(value: Vec<&str>) -> HashSet<&str> {
    value.into_iter().collect::<HashSet<&str>>()
}

#[cfg(test)]
mod test_surreal_config {
    use serde_json::json;
    use surrealdb::opt::auth;

    use crate::config::entity::{to_hashset, Namespace};

    use super::Root;

    /// 使用原始surrealdb::Root转为String和json文本进行匹配
    #[test]
    fn test_root_credential_from() {
        let root_str = json!({
            "user" : "root",
            "pass" : "root",
        });
        let root_entity = auth::Root {
            username: "root",
            password: "root",
        };
        let json_str1 = serde_json::to_string(&root_entity).unwrap();
        let json_str2 = serde_json::to_string(&root_str).unwrap();
        assert_eq!(json_str1, json_str2);
    }
    #[test]
    fn test_root_new_default() {
        let root = Root::new("root", "root");
        let root_default = Root::default();
        assert_eq!(root, root_default);
    }
    #[test]
    fn test_root_to_string() {
        let root_value = json!({"user":"root", "pass":"root"});
        let root_str = Root::new("root", "root").to_string();
        assert_eq!(root_str, serde_json::to_string(&root_value).unwrap());
    }
    #[test]
    fn test_ns_to_string() {
        let ns_value = json!({"ns":"test","user":"root", "pass":"root"});
        let ns_str = Namespace::new("root", "root", "test").to_string();
        assert_eq!(ns_str, serde_json::to_string(&ns_value).unwrap());
    }
    #[test]
    fn test_trans_root_to_struct() {
        let trans_json = json!(
            {
                "ns" : "test",
                "user" : "root",
                "pass" : "root",
            }
        );
        let trans_value = trans_json
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
            .collect::<Vec<(String, String)>>();
        let trans_keys = trans_value
            .iter()
            .map(|(k, v)| k.as_str())
            .collect::<Vec<&str>>();
        let root_keys = Root::keys();
        assert!(to_hashset(root_keys).ne(&to_hashset(trans_keys)));
    }
    #[test]
    fn test_trans_ns_to_struct() {
        let trans_json = json!(
            {
                "ns" : "test",
                "user" : "root",
                "pass" : "root",
            }
        );

        let trans_value = trans_json
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
            .collect::<Vec<(String, String)>>();
        let trans_keys = trans_value
            .iter()
            .map(|(k, v)| k.as_str())
            .collect::<Vec<&str>>();

        let ns_keys = Namespace::keys();
        assert!(to_hashset(ns_keys).eq(&to_hashset(trans_keys)));
    }
}
