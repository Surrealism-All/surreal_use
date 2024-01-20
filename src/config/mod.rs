use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use surrealdb::opt::auth::{Credentials, Jwt};

use self::auth::AuthCredentials;

pub mod auth;
pub mod parser;

/// 默认的配置文件的名字
/// 当不传入指定的配置文件的位置时使用
/// 通过当前项目地址和文件名字构建默认配置文件地址进行推测
const DEFAULT_CONFIG_NAME: &str = "surrealdb.config.json";

/// 认证桥接器
/// 简单trait，这种方式不够好，缺少生命周期标注，返回类型宽泛，使用者体验不好
/// ```code
/// pub trait AuthBridger<Action>  {
///     fn to_lower_cast(&self)->impl Credentials<Action,Jwt> ;
///     fn keys() -> Vec<&'static str>;
/// }
/// ```
pub trait AuthBridger<'a, Action> {
    type AuthType;
    fn to_lower_cast(&'a self) -> Self::AuthType
    where
        Self::AuthType: Credentials<Action, Jwt>;
    fn keys() -> Vec<&'a str>;
}

/// SurrealDB的配置
#[derive(Debug, Serialize, Clone)]
pub struct SurrealConfig {
    /// 启动URL地址
    endpoint: String,
    /// 启动端口
    port: u16,
    /// 登录凭证数据
    auth: Value,
}

impl From<Value> for SurrealConfig {
    fn from(value: Value) -> Self {
        let endpoint = value.get("endpoint").unwrap().as_str().unwrap().to_string();
        let port = value.get("port").unwrap().as_u64().unwrap() as u16;
        let auth = value.get("auth").unwrap().clone();
        Self {
            endpoint,
            port,
            auth,
        }
    }
}

impl SurrealConfig {
    /// 获取登录凭证数据
    /// 所有的凭证实际上都能够进行转换
    /// 事实上用户可能完全不知道是什么类型的登录凭证
    /// @return Value
    pub fn get_auth<P>(&self) -> AuthCredentials<P>
    where
        P: Serialize + DeserializeOwned,
    {
        let res: AuthCredentials<P> = self.auth.clone().into();
        res
    }
    pub fn get_endpoint(&self) -> &str {
        &self.endpoint
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn url(&self) -> String {
        format!("{}:{}", self.endpoint, self.port)
    }
}

#[cfg(test)]
mod test_config {

    use serde_json::Value;

    use crate::config::auth::AuthCredentials;

    use super::{parser::Parsers, SurrealConfig};
    //尝试解析配置
    #[test]
    fn test_parser_config() {
        let json = Parsers::Json.parse(None);
        let config: SurrealConfig = json.into();
        dbg!(&config);
        let auth_credentail: AuthCredentials<Value> = config.get_auth();
        dbg!(&auth_credentail);
        //当json为：
        // {
        //     "endpoint":"127.0.0.1",
        //     "port":10086,
        //     "auth":{
        //       "user":"root",
        //       "pass":"root",
        //       "sc":"test_sc",
        //       "db":"surrealdb",
        //       "ns":"test_ns"
        //     }
        //   }
        // 得到AuthCredentials：
        // Some(
        //     Scope {
        //         ns: "test_ns",
        //         db: "surrealdb",
        //         sc: "test_sc",
        //         params: Object {
        //             "user": String("root"),
        //             "pass": String("root"),
        //         },
        //     },
        // ),
    }
}
