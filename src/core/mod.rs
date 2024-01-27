mod create;
mod delete;
mod insert;
mod select;
pub mod sql;
mod stmt;
mod update;
mod r#use;

pub use stmt::Stmt;

/// ## 语句桥接器
/// 实现语句桥接器能够赋予语句转换为原始语句的能力
pub trait StmtBridge {
    type OriginType;
    /// 转换为原始数据结构体
    fn to_origin(self) -> Self::OriginType;
    /// 获得原始数据结构体的借用
    fn origin(&self) -> &Self::OriginType;
}

/// ## 语句桥接器宏
/// 生成语句桥接器的实现的宏
/// 需要传入扩展的语句类型和原始的语句类型
/// ### 生成的语法如下
/// ```
/// impl StmtBridge for UseStmt {
///     type OriginType = UseStatement;
///
///     fn to_origin(self) -> Self::OriginType {
///         self.origin
///     }
///     fn origin(&self) -> &Self::OriginType {
///         &self.origin
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_stmt_bridge {
    ($stmt:ty , $origin:ty) => {
        impl StmtBridge for $stmt {
            type OriginType = $origin;

            fn to_origin(self) -> Self::OriginType {
                self.origin
            }
            fn origin(&self) -> &Self::OriginType {
                &self.origin
            }
        }
    };
}
