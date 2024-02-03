mod create;
mod delete;
mod insert;
mod select;
pub mod sql;
mod stmt;
mod update;
mod r#use;

pub use stmt::Stmt;

/// ## statement bridge
/// Implement a statement bridge that endows statements with the ability to convert them into original statements
pub trait StmtBridge {
    type OriginType;
    /// convert to original data structure
    fn to_origin(self) -> Self::OriginType;
    /// obtain borrowing of the original data structure
    fn origin(&self) -> &Self::OriginType;
}

/// ## macro for StmtBridge
/// Macro for implementing statement bridge
///
/// Need to pass in the extended statement type and the original statement type
/// ### The generated syntax is as follows
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
