use crate::model::{field::FieldType, Field, ModelSchema};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FilterClause {
    pub field_name: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct QueryOptions {
    pub limit: i64,
    pub offset: i64,
    pub order_by: Option<String>,
    pub search_query: Option<String>,
    pub filters: Vec<FilterClause>,
}

impl QueryOptions {
    /// Parse query parameters from a URL query string or HashMap
    pub fn parse(params: &HashMap<String, String>) -> Self {
        let mut limit = 50;
        let mut offset = 0;
        let mut order_by = Some("id DESC".to_string());
        let mut search_query = None;
        let mut filters = Vec::new();

        for (k, v) in params {
            if k == "$limit" {
                if let Ok(l) = v.parse::<i64>() {
                    limit = l.clamp(1, 1000);
                }
            } else if k == "$offset" {
                if let Ok(o) = v.parse::<i64>() {
                    offset = o.max(0);
                }
            } else if k == "$order" {
                if v.starts_with('-') {
                    order_by = Some(format!("\"{}\" DESC", &v[1..]));
                } else {
                    order_by = Some(format!("\"{}\" ASC", v));
                }
            } else if k == "$search" || k == "$q" {
                if !v.trim().is_empty() {
                    search_query = Some(v.trim().to_string());
                }
            } else if !k.starts_with('$') {
                if let Some(pos) = k.find("__") {
                    let field = &k[..pos];
                    let op = &k[pos + 2..];
                    filters.push(FilterClause {
                        field_name: field.to_string(),
                        operator: op.to_string(),
                        value: v.clone(),
                    });
                } else {
                    filters.push(FilterClause {
                        field_name: k.clone(),
                        operator: "eq".to_string(),
                        value: v.clone(),
                    });
                }
            }
        }

        Self {
            limit,
            offset,
            order_by,
            search_query,
            filters,
        }
    }

    /// Coerce a filter query string to a typed JSON Value matching field definition
    fn coerce_value(field: &Field, raw: &str) -> Value {
        match &field.field_type {
            FieldType::Boolean => {
                let b = raw.eq_ignore_ascii_case("true") || raw == "1";
                Value::Bool(b)
            }
            FieldType::Integer | FieldType::ForeignKey { .. } => {
                if let Ok(i) = raw.parse::<i64>() {
                    Value::Number(i.into())
                } else {
                    Value::String(raw.to_string())
                }
            }
            FieldType::Float | FieldType::Money { .. } | FieldType::ProgressBar { .. } => {
                if let Ok(flt) = raw.parse::<f64>() {
                    serde_json::Number::from_f64(flt)
                        .map(Value::Number)
                        .unwrap_or_else(|| Value::String(raw.to_string()))
                } else {
                    Value::String(raw.to_string())
                }
            }
            _ => Value::String(raw.to_string()),
        }
    }

    /// Build SQL WHERE and ORDER BY clauses with parameter bindings
    pub fn build_sql(&self, schema: &ModelSchema) -> (String, Vec<Value>, String, Vec<Value>) {
        let mut where_parts = Vec::new();
        let mut params = Vec::new();

        // 1. Full-text search across searchable fields
        if let Some(ref search) = self.search_query {
            let searchable_fields: Vec<&str> = schema
                .fields
                .iter()
                .filter(|f| f.searchable)
                .map(|f| f.name.as_str())
                .collect();

            if !searchable_fields.is_empty() {
                let mut search_parts = Vec::new();
                for field in searchable_fields {
                    search_parts.push(format!("\"{}\" LIKE ?", field));
                    params.push(Value::String(format!("%{}%", search)));
                }
                where_parts.push(format!("({})", search_parts.join(" OR ")));
            }
        }

        // 2. Process explicit field filters
        for f in &self.filters {
            if let Some(field) = schema.get_field(&f.field_name) {
                let typed_val = Self::coerce_value(field, &f.value);

                match f.operator.as_str() {
                    "eq" => {
                        where_parts.push(format!("\"{}\" = ?", field.name));
                        params.push(typed_val);
                    }
                    "neq" | "not" => {
                        where_parts.push(format!("\"{}\" != ?", field.name));
                        params.push(typed_val);
                    }
                    "gt" => {
                        where_parts.push(format!("\"{}\" > ?", field.name));
                        params.push(typed_val);
                    }
                    "gte" => {
                        where_parts.push(format!("\"{}\" >= ?", field.name));
                        params.push(typed_val);
                    }
                    "lt" => {
                        where_parts.push(format!("\"{}\" < ?", field.name));
                        params.push(typed_val);
                    }
                    "lte" => {
                        where_parts.push(format!("\"{}\" <= ?", field.name));
                        params.push(typed_val);
                    }
                    "contains" | "icontains" => {
                        where_parts.push(format!("\"{}\" LIKE ?", field.name));
                        params.push(Value::String(format!("%{}%", f.value)));
                    }
                    "startswith" | "istartswith" => {
                        where_parts.push(format!("\"{}\" LIKE ?", field.name));
                        params.push(Value::String(format!("{}%", f.value)));
                    }
                    "endswith" | "iendswith" => {
                        where_parts.push(format!("\"{}\" LIKE ?", field.name));
                        params.push(Value::String(format!("%{}", f.value)));
                    }
                    "in" => {
                        let items: Vec<&str> = f.value.split(',').map(|s| s.trim()).collect();
                        if !items.is_empty() {
                            let placeholders: Vec<String> = items.iter().map(|_| "?".to_string()).collect();
                            where_parts.push(format!("\"{}\" IN ({})", field.name, placeholders.join(", ")));
                            for it in items {
                                params.push(Self::coerce_value(field, it));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let where_clause = if where_parts.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_parts.join(" AND "))
        };

        // Count query
        let count_sql = format!("SELECT COUNT(*) as total FROM \"{}\" {}", schema.table_name, where_clause);
        let count_params = params.clone();

        // Select query with strict ORDER BY whitelist validation against schema fields
        let validated_order = if let Some(ref raw_order) = self.order_by {
            let (field_name, is_desc) = if raw_order.ends_with(" DESC") {
                let name = raw_order.trim_start_matches('"').trim_end_matches("\" DESC");
                (name, true)
            } else if raw_order.ends_with(" ASC") {
                let name = raw_order.trim_start_matches('"').trim_end_matches("\" ASC");
                (name, false)
            } else {
                (raw_order.as_str(), false)
            };

            if schema.get_field(field_name).is_some() {
                format!("\"{}\" {}", field_name, if is_desc { "DESC" } else { "ASC" })
            } else {
                "\"id\" DESC".to_string()
            }
        } else {
            "\"id\" DESC".to_string()
        };

        let select_sql = format!(
            "SELECT * FROM \"{}\" {} ORDER BY {} LIMIT ? OFFSET ?",
            schema.table_name, where_clause, validated_order
        );

        params.push(Value::Number(self.limit.into()));
        params.push(Value::Number(self.offset.into()));

        (select_sql, params, count_sql, count_params)
    }
}
