pub mod entity;
pub mod parser;

/// 默认的配置文件的名字
/// 当不传入指定的配置文件的位置时使用
/// 通过当前项目地址和文件名字构建默认配置文件地址进行推测
const DEFAULT_CONFIG_NAME: &str = "surrealdb.config.json";

#[cfg(test)]
mod test_config {
    use crate::config::entity::SurrealCredentials;

    use super::entity::{Namespace, Root, SurrealConfig};

    use super::parser::Parsers;
    #[test]
    fn test_parser_config() {
        let json = Parsers::Json.parse(None);
        // dbg!(&json);
        let res = json.get("credential").unwrap().clone();
        // let config: SurrealConfig = serde_json::from_value(json).unwrap();
        let config: SurrealCredentials = res.into();
        dbg!(config);
    }
}
