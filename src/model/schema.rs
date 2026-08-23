use crate::model::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

/// Definition and relational schema of a complete database model/table in Vella.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSchema {
    pub name: String,
    pub table_name: String,
    pub display_name: String,
    pub category: String,
    pub icon: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
}

impl ModelSchema {
    /// Create a new model schema with auto-assigned primary key
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        let table_name = Self::pluralize(&name_str.to_lowercase());
        let display_name = Field::generate_display_name(&name_str);

        let mut schema = Self {
            name: name_str,
            table_name,
            display_name,
            category: "General".to_string(),
            icon: "box".to_string(),
            description: None,
            fields: Vec::new(),
        };

        // All models have standard auto-incrementing ID field
        schema.fields.push(Field {
            name: "id".to_string(),
            display_name: "ID".to_string(),
            field_type: FieldType::Integer,
            required: false,
            unique: true,
            searchable: true,
            filterable: true,
            list_display: true,
            read_only: true,
            encrypted: false,
            requires_approval: false,
            default_value: None,
            help_text: Some("Primary Key".to_string()),
        });

        schema
    }

    pub fn table_name(mut self, table_name: impl Into<String>) -> Self {
        self.table_name = table_name.into();
        self
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add standard timestamps (created_at and updated_at)
    pub fn with_timestamps(mut self) -> Self {
        self.fields.push(Field {
            name: "created_at".to_string(),
            display_name: "Created At".to_string(),
            field_type: FieldType::DateTime,
            required: false,
            unique: false,
            searchable: false,
            filterable: true,
            list_display: true,
            read_only: true,
            encrypted: false,
            requires_approval: false,
            default_value: None,
            help_text: Some("Creation timestamp".to_string()),
        });
        self.fields.push(Field {
            name: "updated_at".to_string(),
            display_name: "Updated At".to_string(),
            field_type: FieldType::DateTime,
            required: false,
            unique: false,
            searchable: false,
            filterable: true,
            list_display: false,
            read_only: true,
            encrypted: false,
            requires_approval: false,
            default_value: None,
            help_text: Some("Last modification timestamp".to_string()),
        });
        self
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name.eq_ignore_ascii_case(name))
    }

    /// Retrieve all vector embedding fields defined on this model
    pub fn vector_fields(&self) -> Vec<(&Field, usize)> {
        self.fields
            .iter()
            .filter_map(|f| match f.field_type {
                FieldType::Vector { dimensions } => Some((f, dimensions)),
                _ => None,
            })
            .collect()
    }

    pub fn has_vectors(&self) -> bool {
        !self.vector_fields().is_empty()
    }

    fn pluralize(s: &str) -> String {
        if s.ends_with('y') && !s.ends_with("ay") && !s.ends_with("ey") && !s.ends_with("oy") {
            format!("{}ies", &s[..s.len() - 1])
        } else if s.ends_with('s') || s.ends_with("ch") || s.ends_with("sh") || s.ends_with('x') {
            format!("{}es", s)
        } else {
            format!("{}s", s)
        }
    }
}
