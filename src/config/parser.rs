use super::{SurrealConfig, DEFAULT_CONFIG_NAME};
use serde_json::Value;
use std::{
    env::current_dir,
    fs::{canonicalize, read_to_string},
    path::{Path, PathBuf},
};

/// # 解析器枚举
/// 可以包含多种解析器，例如JSON解析器，TOML解析器，YAML解析等
/// 这里先只实现JSON解析器
/// 对于外部而言应该只使用Parsers枚举来选择解析器进行解析
pub enum Parsers {
    Json,
}

impl Parsers {
    /// 提供简便的处理解析入口
    /// 内部匹配解析器类型进行解析
    /// 最后会返回Value类型，这种方式的好处在于使用者可以使用抛弃框架内部的实现自己编写结构体进行转换
    pub fn parse(&self, path: Option<&str>) -> Value {
        match self {
            Parsers::Json => JsonParser::parse(path),
            _ => panic!("Invalid Parser"),
        }
    }
    /// 提供下层处理器
    /// 便于进行更加复杂的解析
    pub fn json() -> JsonParser {
        JsonParser
    }
    /// 解析为SurrealConfig的形式
    /// 直接使用框架内提供SurrealConfig
    /// 借助SurrealConfig得到具体的配置信息进行使用
    pub fn parse_to_config(&self, path: Option<&str>) -> SurrealConfig {
        let config: SurrealConfig = self.parse(path).into();
        config
    }
}

/// # Json文件解析器
/// 用于解析json形式的配置文件
/// 将json文件转换为统一serde_json::Value
pub struct JsonParser;

impl JsonParser {
    /// ## 解析配置文件
    /// 使用JsonParser解析某个json文件得到配置数据
    ///
    /// 配置数据不需要进行封装，直接返回
    ///
    /// 当传入路径为None时表示使用默认解析文件地址
    ///
    /// 当传入路径为相对路径时使用根目录作为路径凭据
    /// ### param
    /// 1. path : Option<&Path>
    /// ### return
    /// serde_json::Value
    pub fn parse<P>(path: Option<P>) -> Value
    where
        P: AsRef<Path>,
    {
        // Some时借助canonicalize帮助进行解析，赋予处理相对路径的能力
        let path: PathBuf = match path {
            Some(p) => canonicalize(p).unwrap(),
            None => {
                let mut current_dir = current_dir().unwrap();
                let _ = current_dir.push(DEFAULT_CONFIG_NAME);
                current_dir
            }
        };
        //获取字符串文本
        let config_str = read_to_string(path.as_path()).unwrap_or(String::new());
        let res: Value = serde_json::from_str(&config_str).unwrap();
        return res;
    }
}

#[cfg(test)]
mod parser_test {
    use std::path::Path;

    use serde_json::Value;

    use super::{JsonParser, Parsers};

    /// 使用json字符串与JsonParser获取的文件配置进行匹配测试
    /// 确认解析出的serde_json::Value能够得到相同的结果
    #[test]
    #[should_panic]
    fn test_json_str_parser_match() {
        //编写一个json满足需要解析的格式
        let json_str = r#"
      {
        "endpoint":"127.0.0.1",
        "port":10086,
        "auth":{
          "user":"root",
          "pass":"root"
        }
      }
      "#;
        let json_value1: Value = serde_json::from_str(json_str).unwrap();
        //通过绝对路径的方式得到文件配置
        let json_value2 = JsonParser::parse(Some(Path::new(
            "E:\\Rust\\docs\\book\\code\\surreal_use\\surrealdb.config.json",
        )));
        assert_eq!(json_value1, json_value2)
    }

    /// 对使用相对路径和绝对路径的方式进行测试
    /// 使用相对路径是需要以根目录作为路径依据
    /// 主要测试相对路径是否能够得到相同的解析结果
    #[test]
    fn test_json_parser_with_path() {
        let json_value1 = JsonParser::parse(Some(Path::new(
            "E:\\Rust\\docs\\book\\code\\surreal_use\\surrealdb.config.json",
        )));
        let json_value2 = JsonParser::parse(Some(Path::new("./surrealdb.config.json")));
        assert_eq!(json_value1, json_value2);
    }

    /// 测试参数为None的形式的自动获得配置路径的方式解析配置文件
    /// 这种情况下默认获取根目录下的surrealdb.config.json配置文件
    #[test]
    fn test_json_parser_no_path() {
        let json_value1 = JsonParser::parse::<&str>(None);
        let json_value2 = JsonParser::parse(Some(Path::new("./surrealdb.config.json")));
        assert_eq!(json_value1, json_value2);
    }

    /// 测试使用上层解析器枚举帮助进行解析
    /// 间接使用下层不直接调用
    #[test]
    fn test_parsers_run() {
        let json_value1 = Parsers::Json.parse(None);
        let json_value2 = Parsers::Json.parse(Some("./surrealdb.config.json"));
        assert_eq!(json_value1, json_value2);
    }
}
