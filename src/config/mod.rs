pub mod entity;
pub mod parser;

/// 默认的配置文件的名字
/// 当不传入指定的配置文件的位置时使用
/// 通过当前项目地址和文件名字构建默认配置文件地址进行推测
const DEFAULT_CONFIG_NAME: &str = "surrealdb.config.json";
