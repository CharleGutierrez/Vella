use crate::model::ModelSchema;
use serde_json::Value;

/// An AST-like representation of a parameterized SQL Query
#[derive(Debug, Clone)]
pub struct ParameterizedQuery {
    pub sql: String,
    pub params: Vec<Value>,
}

impl ParameterizedQuery {
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            params: Vec::new(),
        }
    }

    pub fn bind(mut self, val: Value) -> Self {
        self.params.push(val);
        self
    }

    pub fn count_query_for(schema: &ModelSchema, where_clause: &str, params: Vec<Value>) -> Self {
        let sql = if where_clause.is_empty() {
            format!("SELECT COUNT(*) as total FROM \"{}\"", schema.table_name)
        } else {
            format!("SELECT COUNT(*) as total FROM \"{}\" {}", schema.table_name, where_clause)
        };
        Self { sql, params }
    }
}
