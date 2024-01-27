mod create;
mod delete;
mod insert;
mod select;
pub mod sql;
mod stmt;
mod update;
mod r#use;

pub use stmt::Stmt;

//语句桥接器
pub trait StmtBridge {
    type OriginType;
    //返回原始数据结构体
    fn to_origin(self) -> Self::OriginType;
    fn origin(&self) -> &Self::OriginType;
}

/// 生成实现语句桥接器宏
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
