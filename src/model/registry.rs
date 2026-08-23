use crate::model::schema::ModelSchema;
use std::collections::HashMap;

/// Global registry of registered models in Vella
#[derive(Debug, Clone, Default)]
pub struct SchemaRegistry {
    schemas: HashMap<String, ModelSchema>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, ModelSchema>) -> Self {
        Self { schemas: map }
    }

    pub fn register(&mut self, schema: ModelSchema) {
        let key = schema.name.to_lowercase();
        self.schemas.insert(key, schema);
    }

    pub fn get(&self, name: &str) -> Option<&ModelSchema> {
        self.schemas.get(&name.to_lowercase())
    }

    pub fn all(&self) -> Vec<&ModelSchema> {
        let mut list: Vec<&ModelSchema> = self.schemas.values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    pub fn len(&self) -> usize {
        self.schemas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
}
