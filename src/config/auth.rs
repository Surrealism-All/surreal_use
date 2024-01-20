use super::AuthBridger;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use surrealdb::opt::auth;
use surrealdb::opt::auth::Credentials;
use surrealdb::opt::auth::Jwt;
use surrealdb::opt::auth::Signin;

macro_rules! is_auth {
    ($auth:ident,$authType:ident) => {
        pub fn $auth(&self) -> bool {
            matches!(self, AuthCredentials::$authType(_))
        }
    };
}

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize)]
pub enum AuthCredentials<P> {
    Root(Root),
    NS(Namespace),
    DB(Database),
    SC(Option<Scope<P>>),
}

impl<P> AuthCredentials<P> {
    is_auth!(is_root, Root);
    is_auth!(is_ns, NS);
    is_auth!(is_db, DB);
    is_auth!(is_sc, SC);
}

impl<P> From<Value> for AuthCredentials<P>
where
    P: Serialize + DeserializeOwned,
{
    fn from(value: Value) -> Self {
        fn try_sc<P>(value: Value) -> Result<AuthCredentials<P>, &'static str>
        where
            P: Serialize + DeserializeOwned,
        {
            if let Ok(scope) = serde_json::from_value::<Scope<P>>(value) {
                return Ok(AuthCredentials::SC(Some(scope)));
            }
            Err("SurrealDB Configuration Error : Couldn't deserialize Scope credentials")
        }

        let trans_value = value.as_object().unwrap().clone();
        let keys = trans_value
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>();
        match trans_value.len() {
            2 => {
                //判断参数
                if to_hashset(Root::keys()).eq(&to_hashset(keys)) {
                    return AuthCredentials::Root(serde_json::from_value::<Root>(value).unwrap());
                } else {
                    panic!("SurrealDB Configuration Error : Credential Root should use `user` and `pass`");
                }
            }
            3 => {
                if to_hashset(Namespace::keys()).eq(&to_hashset(keys)) {
                    return AuthCredentials::NS(
                        serde_json::from_value::<Namespace>(value).unwrap(),
                    );
                } else {
                    panic!("SurrealDB Configuration Error : Credential Namespace should use `user` , `pass` , `ns`");
                }
            }
            4 => {
                if to_hashset(Database::keys()).eq(&to_hashset(keys)) {
                    return AuthCredentials::DB(serde_json::from_value::<Database>(value).unwrap());
                } else {
                    match try_sc::<P>(value){
                        Ok(sc) => sc,
                        Err(_) => panic!("SurrealDB Configuration Error : Credential Namespace should use `user` , `pass` , `ns`,`db`"),
                    }
                }
            }
            _ => match try_sc::<P>(value) {
                Ok(sc) => sc,
                Err(e) => panic!("{}", e),
            },
        }
    }
}

/// Root方式的登录凭证的扩展
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Root {
    /// The username of the root user
    user: String,
    /// The password of the root user
    pass: String,
}

impl Default for Root {
    fn default() -> Self {
        Self::new("root", "root")
    }
}

impl<'a> AuthBridger<'a, Signin> for Root {
    type AuthType = auth::Root<'a>;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<Signin, Jwt>,
    {
        auth::Root {
            username: &self.user,
            password: &self.pass,
        }
    }
    fn keys() -> Vec<&'a str> {
        vec!["user", "pass"]
    }
}

impl Root {
    pub fn new(user: &str, pass: &str) -> Self {
        Root {
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }
}

//实现ToString trait赋予转换String的能力
impl ToString for Root {
    fn to_string(&self) -> String {
        to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Namespace {
    /// The namespace the user has access to
    ns: String,
    /// The username of the namespace user
    user: String,
    /// The password of the namespace user
    pass: String,
}

impl<'a> AuthBridger<'a, Signin> for Namespace {
    type AuthType = auth::Namespace<'a>;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<Signin, Jwt>,
    {
        Self::AuthType {
            namespace: &self.ns,
            username: &self.user,
            password: &self.pass,
        }
    }
    fn keys() -> Vec<&'a str> {
        vec!["ns", "user", "pass"]
    }
}

impl Namespace {
    pub fn new(user: &str, pass: &str, ns: &str) -> Self {
        Self {
            ns: ns.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }
}

impl ToString for Namespace {
    fn to_string(&self) -> String {
        to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Database {
    /// The namespace the user has access to
    pub ns: String,
    /// The database the user has access to
    pub db: String,
    /// The username of the database user
    pub user: String,
    /// The password of the database user
    pub pass: String,
}

impl<'a> AuthBridger<'a, Signin> for Database {
    type AuthType = auth::Database<'a>;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<Signin, Jwt>,
    {
        Self::AuthType {
            namespace: &self.ns,
            database: &self.db,
            username: &self.user,
            password: &self.pass,
        }
    }
    fn keys() -> Vec<&'a str> {
        vec!["ns", "db", "user", "pass"]
    }
}

impl ToString for Database {
    fn to_string(&self) -> String {
        to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Scope<P> {
    /// The namespace the user has access to
    pub ns: String,
    /// The database the user has access to
    pub db: String,
    /// The scope to use for signin and signup
    pub sc: String,
    /// The additional params to use
    #[serde(flatten)]
    pub params: P,
}

impl<'a, T, P> AuthBridger<'a, T> for Scope<P>
where
    P: Serialize + Clone,
{
    type AuthType = auth::Scope<'a, P>;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<T, Jwt>,
    {
        Self::AuthType {
            namespace: &self.ns,
            database: &self.db,
            scope: &self.sc,
            params: self.params.clone(),
        }
    }
    fn keys() -> Vec<&'a str> {
        vec!["sc", "ns", "db"]
    }
}

// impl<'a,P> AuthBridger<'a,Signin> for Scope<P> where P:Serialize+Clone {
//     type AuthType = auth::Scope<'a,P>;
//     fn to_lower_cast(&'a self)->Self::AuthType where Self::AuthType : Credentials<Signin,Jwt> {
//         Self::AuthType{
//             namespace: &self.ns,
//             database: &self.db,
//             scope: &self.sc,
//             params: self.params.clone(),
//         }
//     }
//     fn keys() -> Vec<&'a str> {
//         vec!["sc","ns","db"]
//     }
// }

impl<P> ToString for Scope<P>
where
    P: Serialize,
{
    fn to_string(&self) -> String {
        to_string(self)
    }
}

impl<P> Scope<P>
where
    P: Serialize,
{
    pub fn new(ns: &str, db: &str, sc: &str, params: P) -> Self {
        Scope {
            ns: ns.to_string(),
            db: db.to_string(),
            sc: sc.to_string(),
            params,
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
    use super::{to_hashset, AuthCredentials, Namespace, Root, Scope};
    use crate::config::AuthBridger;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use surrealdb::opt::auth::{self, Signin};

    #[test]
    fn test_auth_deserialize() {
        let auth_scope_json = json!({
            "ns":"test",
            "sc":"test_sc",
            "db":"test",
            "user":"root",
            "pass":"root",
        });
        #[derive(Debug,Serialize,Deserialize)]
        struct Params{
            user : String,
            pass:String
        }
        let auth_root_json = json!({
            "user":"root",
            "pass":"root"
        });
        let auth_root: AuthCredentials<()> = auth_root_json.into();
        assert!(auth_root.is_root());
        let auth_scope:AuthCredentials<Params> =auth_scope_json.into();
        assert!(auth_scope.is_sc());
    }
    #[test]
    fn test_lower_cast() {
        let root = Root::new("Matt", "123456");
        let root_lower = root.to_lower_cast();
        dbg!(serde_json::to_string_pretty(&root_lower).unwrap());
    }

    #[test]
    fn test_scope_deserialize() {
        #[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
        struct Params {
            name: String,
            email: String,
        }

        let scope = Scope::new(
            "test",
            "surreal",
            "use",
            Params {
                name: "Matt".to_string(),
                email: "Matt@gmail.com".to_string(),
            },
        );

        let json_scope = json!(
            {
                "ns":"test",
                "db":"surreal",
                "sc":"use",
                "name":"Matt",
                "email":"Matt@gmail.com"
            }
        );

        //json_scope -> scope
        let scope2: Scope<Params> = serde_json::from_value(json_scope).unwrap();
        assert_eq!(scope2, scope);
    }

    #[test]
    fn test_scope() {
        #[derive(Serialize, Debug, Clone)]
        struct Params {
            name: String,
            email: String,
        }

        let scope = Scope::new(
            "test",
            "surreal",
            "use",
            Params {
                name: "Matt".to_string(),
                email: "Matt@gmail.com".to_string(),
            },
        );
        dbg!(&scope.to_string());

        let scope_keys = <Scope<Params> as AuthBridger<'_, Signin>>::keys();
        dbg!(scope_keys);
        let scope_lower = <Scope<Params> as AuthBridger<'_, Signin>>::to_lower_cast(&scope);
        dbg!(scope_lower);
    }

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
            .map(|(k, _v)| k.as_str())
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
            .map(|(k, _v)| k.as_str())
            .collect::<Vec<&str>>();

        let ns_keys = Namespace::keys();
        assert!(to_hashset(ns_keys).eq(&to_hashset(trans_keys)));
    }
}
