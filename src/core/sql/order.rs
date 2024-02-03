use surrealdb::sql;

use super::Field;

/// build ORDER BY sub query
/// [ ORDER [ BY ]
///         @fields [
///             RAND()
///             | COLLATE
///             | NUMERIC
///         ] [ ASC | DESC ] ...
///     ] ]
pub struct Order(sql::Order);

impl Order {
    pub fn new(field: impl Into<Field>) -> Self {
        let field: Field = field.into();
        Order(sql::Order {
            order: field.to_idiom(),
            random: false,
            collate: Default::default(),
            numeric: Default::default(),
            direction: Default::default(),
        })
    }
    /// ## use order ASC
    pub fn asc(mut self) -> Self {
        self.0.direction = false;
        self
    }
    /// ## use order DESC
    pub fn desc(mut self) -> Self {
        self.0.direction = true;
        self
    }
    /// ## use keyword NUMERIC
    pub fn numeric(mut self) -> Self {
        self.0.numeric = true;
        self
    }
    /// ## use keyword COLLATE
    pub fn collate(mut self) -> Self {
        self.0.collate = true;
        self
    }
    /// ## order random
    pub fn rand(mut self) -> Self {
        self.0.random = true;
        self
    }
    pub fn to_origin(self) -> sql::Order {
        self.0
    }
}

impl From<Order> for sql::Order {
    fn from(value: Order) -> Self {
        value.to_origin()
    }
}

impl From<sql::Order> for Order {
    fn from(value: sql::Order) -> Self {
        Order(value)
    }
}
