use std::{borrow::Cow, collections::HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
// use surrealdb::opt::auth::{Credentials, Database, Namespace, Root, Scope};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SurrealConfig<'a> {
    endpoint: String,
    port: u32,
    credential: SurrealCredentials<'a>,
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

// impl From<Value> for SurrealConfig {
//     fn from(value: Value) -> Self {
//         Root::from(value)
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SurrealCredentials<'a> {
    Root(Root<'a>),
    Namespace(Namespace<'a>),
    // Database(Database<'a>),
    // Scope(Scope<'a, P>),
}

impl<'a> From<Value> for SurrealCredentials<'a> {
    fn from(value: Value) -> Self {
        Self::deserialize(value)
    }
}

impl<'a> SurrealCredentials<'a> {
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
            .map(|(k, v)| k.as_str())
            .collect::<Vec<&str>>();
        match trans_keys.len() {
            2 => {
                //判断参数
                if (to_hashset(Root::keys()).eq(&to_hashset(trans_keys))) {
                    let r = serde_json::from_value::<Root>(value).unwrap();
                    return SurrealCredentials::Root(r);
                } else {
                    panic!("SurrealDB Configuration Error : Credential Root should use `user` and `pass`")
                }
            }
            _ => panic!("Invalid Configuration"),
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
