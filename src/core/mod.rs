mod stmt;
mod use_stmt;

pub use stmt::Stmt;

//语句桥接器
pub trait StmtBridge {
    type OriginType;
    //返回原始数据结构体
    fn to_origin(self) -> Self::OriginType;
    fn origin(&self) -> &Self::OriginType;
}
