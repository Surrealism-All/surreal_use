use std::fmt::Debug;

use serde::Serialize;
use serde_json::Value;
use surrealdb::opt::auth::{Credentials, Jwt};

use self::auth::AuthCredentials;

pub mod auth;
pub mod entity;
pub mod parser;

/// 默认的配置文件的名字
/// 当不传入指定的配置文件的位置时使用
/// 通过当前项目地址和文件名字构建默认配置文件地址进行推测
const DEFAULT_CONFIG_NAME: &str = "surrealdb.config.json";

/// 认证桥接器
// pub trait AuthBridger<Action>  {
//     fn to_lower_cast(&self)->impl Credentials<Action,Jwt> ;
//     fn keys() -> Vec<&'static str>;
// }

pub trait AuthBridger<'a, Action> {
    type AuthType;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<Action, Jwt>;
    fn keys() -> Vec<&'a str>;
}

#[derive(Debug, Serialize, Clone)]
pub struct SurrealConfig {
    endpoint: String,
    port: u32,
    auth: Value,
}

#[cfg(test)]
mod test_config {
    use crate::config::entity::SurrealCredentials;

    use super::entity::{Namespace, Root};

    use super::parser::Parsers;
    #[test]
    fn test_parser_config() {
        let json = Parsers::Json.parse(None);
        // dbg!(&json);
        let res = json.get("auth").unwrap().clone();
        // let config: SurrealConfig = serde_json::from_value(json).unwrap();
        // let config =SurrealCredentials::deserialize(res);
        // let config:SurrealCredentials<'_,> = serde_json::from_value(res).unwrap();
        // dbg!(config);
    }
}
