use crate::model::{FieldType, SchemaRegistry};
use axum::response::{Html, IntoResponse, Json};
use serde_json::{json, Map, Value};

pub struct OpenApiGenerator;

impl OpenApiGenerator {
    /// Generate complete OpenAPI 3.1 JSON specification for registered schemas (including Vector & AI endpoints)
    pub fn generate_spec(schemas: &SchemaRegistry) -> Value {
        let mut paths = Map::new();
        let mut components_schemas = Map::new();

        for schema in schemas.all() {
            let model_path_name = schema.name.to_lowercase();

            // 1. Generate Schema Object
            let mut props = Map::new();
            let mut required_fields = Vec::new();

            for field in &schema.fields {
                let mut prop_obj = Map::new();

                let (type_str, format_str) = match &field.field_type {
                    FieldType::Integer | FieldType::ForeignKey { .. } => ("integer", Some("int64")),
                    FieldType::Float | FieldType::Money { .. } => ("number", Some("double")),
                    FieldType::Boolean => ("boolean", None),
                    FieldType::DateTime => ("string", Some("date-time")),
                    FieldType::Email => ("string", Some("email")),
                    FieldType::Password => ("string", Some("password")),
                    FieldType::Vector { dimensions } => {
                        prop_obj.insert("type".to_string(), json!("array"));
                        prop_obj.insert("items".to_string(), json!({ "type": "number" }));
                        prop_obj.insert("description".to_string(), json!(format!("{}d float vector embedding", dimensions)));
                        props.insert(field.name.clone(), Value::Object(prop_obj));
                        continue;
                    }
                    _ => ("string", None),
                };

                prop_obj.insert("type".to_string(), json!(type_str));
                if let Some(fmt) = format_str {
                    prop_obj.insert("format".to_string(), json!(fmt));
                }
                prop_obj.insert("title".to_string(), json!(field.display_name));
                if let Some(ref help) = field.help_text {
                    prop_obj.insert("description".to_string(), json!(help));
                }
                if field.read_only {
                    prop_obj.insert("readOnly".to_string(), json!(true));
                }

                if field.required && !field.read_only {
                    required_fields.push(field.name.clone());
                }

                props.insert(field.name.clone(), Value::Object(prop_obj));
            }

            let mut schema_obj = json!({
                "type": "object",
                "title": schema.name,
                "description": schema.description.clone().unwrap_or_else(|| format!("Schema for {}", schema.name)),
                "properties": props
            });

            if !required_fields.is_empty() {
                schema_obj["required"] = json!(required_fields);
            }

            components_schemas.insert(schema.name.clone(), schema_obj);

            // 2. Generate Path Operations
            let list_path = format!("/api/d/{}", model_path_name);
            let list_item = json!({
                "get": {
                    "tags": [schema.category],
                    "summary": format!("List or search {}", schema.display_name),
                    "description": format!("Retrieve paginated list of {} with optional filters, search, and sorting", schema.display_name),
                    "parameters": [
                        { "name": "$limit", "in": "query", "description": "Number of records (1-1000)", "schema": { "type": "integer", "default": 50 } },
                        { "name": "$offset", "in": "query", "description": "Records to skip", "schema": { "type": "integer", "default": 0 } },
                        { "name": "$order", "in": "query", "description": "Sort field (-field for descending)", "schema": { "type": "string" } },
                        { "name": "$search", "in": "query", "description": "Search query across searchable fields", "schema": { "type": "string" } }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "total": { "type": "integer" },
                                            "limit": { "type": "integer" },
                                            "offset": { "type": "integer" },
                                            "data": {
                                                "type": "array",
                                                "items": { "$ref": format!("#/components/schemas/{}", schema.name) }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": [schema.category],
                    "summary": format!("Create a new {}", schema.name),
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": format!("#/components/schemas/{}", schema.name) }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Record created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "data": { "$ref": format!("#/components/schemas/{}", schema.name) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
            paths.insert(list_path, list_item);

            let item_path = format!("/api/d/{}/{{id}}", model_path_name);
            let item_op = json!({
                "get": {
                    "tags": [schema.category],
                    "summary": format!("Get {} by ID", schema.name),
                    "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "integer" } }],
                    "responses": {
                        "200": { "description": "Record found", "content": { "application/json": { "schema": { "$ref": format!("#/components/schemas/{}", schema.name) } } } },
                        "404": { "description": "Record not found" }
                    }
                },
                "put": {
                    "tags": [schema.category],
                    "summary": format!("Update {} by ID", schema.name),
                    "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "integer" } }],
                    "requestBody": {
                        "required": true,
                        "content": { "application/json": { "schema": { "$ref": format!("#/components/schemas/{}", schema.name) } } }
                    },
                    "responses": {
                        "200": { "description": "Record updated" },
                        "404": { "description": "Record not found" }
                    }
                },
                "delete": {
                    "tags": [schema.category],
                    "summary": format!("Delete {} by ID", schema.name),
                    "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "integer" } }],
                    "responses": {
                        "200": { "description": "Record deleted" },
                        "404": { "description": "Record not found" }
                    }
                }
            });
            paths.insert(item_path, item_op);

            // Vector similarity search endpoint if schema has vectors
            if schema.has_vectors() {
                let vec_path = format!("/api/d/{}/search-vector", model_path_name);
                let vec_op = json!({
                    "post": {
                        "tags": [schema.category],
                        "summary": format!("Vector similarity search for {}", schema.name),
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "query_vector": { "type": "array", "items": { "type": "number" } },
                                            "vector_field": { "type": "string", "default": "embedding" },
                                            "top_k": { "type": "integer", "default": 10 },
                                            "metric": { "type": "string", "enum": ["Cosine", "Euclidean", "DotProduct"], "default": "Cosine" }
                                        },
                                        "required": ["query_vector"]
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": { "description": "Vector search results ranked by similarity score" }
                        }
                    }
                });
                paths.insert(vec_path, vec_op);
            }
        }

        json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Vella LLM-Native dAPI",
                "version": "1.0.0",
                "description": "Ultra-fast, auto-generated RESTful & Vector API powered by Vella (Rust, PostgreSQL, SQLite, Realtime Sync, AI Tuner)."
            },
            "paths": paths,
            "components": {
                "schemas": components_schemas
            }
        })
    }

    /// Return interactive Swagger UI HTML page
    pub fn swagger_ui_html() -> Html<&'static str> {
        Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Vella dAPI & Vector Engine Documentation</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
  <style>
    body { margin: 0; background: #0b0f19; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
    .swagger-ui .topbar { display: none; }
    .swagger-ui { color: #f1f5f9; }
    .swagger-ui .info .title { color: #38bdf8; }
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script>
    window.onload = () => {
      window.ui = SwaggerUIBundle({
        url: '/api/openapi.json',
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIBundle.SwaggerUIStandalonePreset
        ],
        layout: "BaseLayout"
      });
    };
  </script>
</body>
</html>"#)
    }
}

pub async fn openapi_json_handler(
    axum::extract::State(state): axum::extract::State<crate::api::handlers::AppState>,
) -> impl IntoResponse {
    let spec = OpenApiGenerator::generate_spec(&state.registry);
    Json(spec)
}

pub async fn swagger_handler() -> impl IntoResponse {
    OpenApiGenerator::swagger_ui_html()
}
