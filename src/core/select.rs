use std::vec;

use surrealdb::sql::{
    self, statements::SelectStatement, Duration, Explain, Fetch, Fetchs, Fields, Group, Groups,
    Idiom, Idioms, Limit, Orders, Split, Splits, Start, Timeout, Value, Values, With,
};

use super::sql::{Cond, Field, Order, SurrealTable};

use crate::impl_stmt_bridge;

use super::StmtBridge;

/// ## 查询SELECT语句
/// SELECT 语句可用于选择和查询数据库中的数据。
///
/// 每个 SELECT 语句支持从多个目标中进行选择，其中可以包括表、记录、边、子查询、参数、数组、对象和其他值。
/// ### example
/// ```
/// let select1 = SelectStmt::new().table("person".into()).field_all();
/// let select2 = SelectStmt::new()
///     .table(("person", "tobie").into())
///     .fields(vec![
///         Field::new("name"),
///         Field::new("address"),
///         Field::new("email"),
///     ]);
/// let select3 = SelectStmt::new()
///     .table(("person", "tobie").into())
///     .fields(vec![Field::new("user.name")]);
/// assert_eq!(select1.to_string().as_str(), "SELECT * FROM person");
/// assert_eq!(
///     select2.to_string().as_str(),
///     "SELECT name, address, email FROM person:tobie"
/// );
/// assert_eq!(
///     select3.to_string().as_str(),
///     "SELECT user.name FROM person:tobie"
/// );
/// let select1 = SelectStmt::new()
/// .table("person".into())
/// .fields(vec![Field::single(
///     "address.ord.coordinates",
///     Some("coordinates"),
/// )]);
/// let select2 = SelectStmt::new()
/// .only()
/// .fields(vec![Field::new("address"), Field::new("name")])
/// .tables(vec!["person".into(), "user".into()])
/// .fetch(vec![Field::new("artist")])
/// .timeout(Duration::from_secs(1))
/// .with_index(vec!["ft_email"])
/// .split(vec![Field::new("email")])
/// .order_by(vec![Order::new("cityId").desc().numeric()])
/// .parallel();
/// assert_eq!(
/// select1.to_string().as_str(),
/// "SELECT address.ord.coordinates AS coordinates FROM person"
/// );
/// assert_eq!(
/// select2.to_string().as_str(),
/// "SELECT address, name FROM ONLY person, user WITH INDEX ft_email SPLIT ON email ORDER BY cityId NUMERIC FETCH artist TIMEOUT 1s PARALLEL"
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    origin: SelectStatement,
}

impl SelectStmt {
    pub fn new() -> Self {
        SelectStmt {
            origin: SelectStatement::default(),
        }
    }
    pub fn only(mut self) -> Self {
        self.origin.only = true;
        self
    }
    pub fn table(mut self, table: SurrealTable) -> Self {
        self.origin.what = table.into();
        self
    }
    // 选择多个目标FROM
    pub fn tables(mut self, tables: Vec<SurrealTable>) -> Self {
        self.origin.what = Values(
            tables
                .into_iter()
                .map(|x| Value::from(x))
                .collect::<Vec<Value>>(),
        );
        self
    }
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        let fields = fields
            .into_iter()
            .map(|x| x.to_origin())
            .collect::<Vec<sql::Field>>();
        self.origin.expr = Fields(fields, false);
        self
    }
    pub fn field_all(mut self) -> Self {
        self.fields(vec![Field::all()])
    }
    /// 为了对记录进行排序，SurrealDB 允许对多个字段和嵌套字段进行排序。
    /// 使用该ORDER BY子句指定应用于对结果记录进行排序的逗号分隔的字段名称列表。
    /// 和关键字可用于指定结果是否应按升序或降序排序ASC。
    /// DESC在对字符串值中的文本进行排序时，该COLLATE关键字可用于使用 unicode 排序规则，确保不同情况和不同语言以一致的方式排序。
    /// 最后，NUMERIC可用于正确排序包含数值的文本。
    pub fn order_by(mut self, orders: Vec<Order>) -> Self {
        self.origin.order.replace(Orders(
            orders
                .into_iter()
                .map(|x| x.to_origin())
                .collect::<Vec<sql::Order>>(),
        ));
        self
    }
    /// SurrealDB支持数据聚合和分组，支持多字段、嵌套字段和聚合函数。
    /// 在 SurrealDB 中，出现在 select 语句的字段投影中的每个字段（并且不是聚合函数）也必须出现在子句中GROUP BY。
    pub fn group_by(mut self, groups: Vec<Field>) -> Self {
        let groups = groups
            .into_iter()
            .map(|x| Group(x.to_idiom()))
            .collect::<Vec<Group>>();
        self.origin.group.replace(Groups(groups));
        self
    }
    /// 与传统 SQL 查询一样，SurrealDB SELECT 查询支持使用WHERE子句进行条件过滤。
    /// 如果子句中的表达式WHERE计算结果为 true，则将返回相应的记录。
    pub fn cond(mut self, cond: Cond) -> Self {
        self.origin.cond.replace(cond.to_origin());
        self
    }
    /// 由于 SurrealDB 支持数组和数组中的嵌套字段，因此可以根据特定字段名称拆分结果，
    /// 将数组中的每个值作为单独的值返回，以及记录内容本身。这在数据分析环境中很有用。
    pub fn split(mut self, splits: Vec<Field>) -> Self {
        let splits = splits
            .into_iter()
            .map(|x| Split(x.to_idiom()))
            .collect::<Vec<Split>>();
        self.origin.split.replace(Splits(splits));
        self
    }
    /// 有时，特别是对于包含大量列的表，用户可能希望有一种更简单的方法来选择除少数特定列之外的所有列，使用该OMIT子句可以在输出记录时省略记录中的某些字段。
    pub fn omit(mut self, omits: Vec<Field>) -> Self {
        let omits = omits
            .into_iter()
            .map(|x| x.to_idiom())
            .collect::<Vec<Idiom>>();
        self.origin.omit.replace(Idioms(omits));
        self
    }
    /// SurrealDB中最强大的功能之一是记录链接和图形连接。
    ///
    /// SurrealDB 无需从多个表中提取数据并将数据合并在一起，而是允许您高效地遍历相关记录，而无需使用 JOIN。
    ///
    /// 要获取记录并用远程记录数据替换记录，请使用FETCH子句指定应就地获取并在最终语句响应输出中返回的字段和嵌套字段。
    pub fn fetch(mut self, fetchs: Vec<Field>) -> Self {
        let fetchs = fetchs
            .into_iter()
            .map(|x| Fetch(x.to_idiom()))
            .collect::<Vec<Fetch>>();
        self.origin.fetch.replace(Fetchs(fetchs));
        self
    }
    pub fn explain(mut self, full: bool) -> Self {
        self.origin.explain.replace(Explain(full));
        self
    }
    /// 要限制返回的记录数，请使用LIMIT子句。
    pub fn limit(mut self, len: usize) -> Self {
        self.origin.limit.replace(Limit(len.into()));
        self
    }
    /// 使用LIMIT子句时，可以通过使用该START子句从结果集中的特定记录开始对结果进行分页。
    pub fn start(mut self, len: usize) -> Self {
        self.origin.start.replace(Start(len.into()));
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.origin.timeout = Some(Timeout(timeout));
        self
    }
    /// 查询规划器可以根据查询的结构和要求，用一个或多个索引迭代器替换标准表迭代器。
    /// 然而，在某些情况下，可能需要或需要对这些潜在优化进行手动控制。
    /// 例如，索引的基数可能很高，甚至可能等于表中的记录数。多个索引迭代的记录总和最终可能大于迭代表获得的记录数。
    /// 在这种情况下，如果存在不同的索引可能性，最可能的最佳选择是使用基数最低的已知索引。
    pub fn with(mut self, with: With) -> Self {
        self.origin.with.replace(with);
        self
    }
    /// WITH NOINDEX强制查询规划器使用表迭代器。
    pub fn with_no_index(mut self) -> Self {
        self.origin.with.replace(With::NoIndex);
        self
    }
    /// WITH INDEX @indexes ...限制查询计划程序仅使用指定的索引
    pub fn with_index(mut self, with: Vec<&str>) -> Self {
        self.origin.with.replace(With::Index(
            with.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ));
        self
    }
    /// ## 设置语句是否可以并行处理
    /// 默认关闭
    pub fn parallel(mut self) -> Self {
        self.origin.parallel = true;
        self
    }
}

impl_stmt_bridge!(SelectStmt, SelectStatement);

impl ToString for SelectStmt {
    fn to_string(&self) -> String {
        self.origin.to_string()
    }
}

#[cfg(test)]
mod test_select_stmt {

    use surrealdb::sql::Duration;

    use crate::core::sql::{Field, Order};

    use super::SelectStmt;
    #[test]
    fn complex() {
        let select1 = SelectStmt::new()
            .table("person".into())
            .fields(vec![Field::single(
                "address.ord.coordinates",
                Some("coordinates"),
            )]);
        let select2 = SelectStmt::new()
            .only()
            .fields(vec![Field::new("address"), Field::new("name")])
            .tables(vec!["person".into(), "user".into()])
            .fetch(vec![Field::new("artist")])
            .timeout(Duration::from_secs(1))
            .with_index(vec!["ft_email"])
            .split(vec![Field::new("email")])
            .order_by(vec![Order::new("cityId").desc().numeric()])
            .parallel();
        assert_eq!(
            select1.to_string().as_str(),
            "SELECT address.ord.coordinates AS coordinates FROM person"
        );
        assert_eq!(
            select2.to_string().as_str(),
            "SELECT address, name FROM ONLY person, user WITH INDEX ft_email SPLIT ON email ORDER BY cityId NUMERIC FETCH artist TIMEOUT 1s PARALLEL"
        );
    }
    #[test]
    fn simple() {
        let select1 = SelectStmt::new().table("person".into()).field_all();
        let select2 = SelectStmt::new()
            .table(("person", "tobie").into())
            .fields(vec![
                Field::new("name"),
                Field::new("address"),
                Field::new("email"),
            ]);
        let select3 = SelectStmt::new()
            .table(("person", "tobie").into())
            .fields(vec![Field::new("user.name")]);
        assert_eq!(select1.to_string().as_str(), "SELECT * FROM person");
        assert_eq!(
            select2.to_string().as_str(),
            "SELECT name, address, email FROM person:tobie"
        );
        assert_eq!(
            select3.to_string().as_str(),
            "SELECT user.name FROM person:tobie"
        );
    }
}
