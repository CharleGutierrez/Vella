use crate::db::DatabaseType;
use crate::model::field::{Field, FieldType};
use crate::model::schema::ModelSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedScaffoldResult {
    pub schema: ModelSchema,
    pub rust_code: String,
    pub sql_ddl: String,
    pub typescript_definition: String,
    pub detected_features: Vec<String>,
}

/// Agentic AI Schema Scaffolder: Parses natural language descriptions into complete,
/// production-grade model definitions, complete with types, validation, vector fields, and DDL.
pub struct AiScaffolder;

impl AiScaffolder {
    /// Generate a full ModelSchema from a natural language prompt
    
    pub async fn generate_schema(model_name: &str, prompt: &str) -> ModelSchema {
        // Priority 1: Gemini (cloud)
        if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
            if let Some(schema) = crate::ai::gemini_scaffolder::call_gemini_schema(model_name, prompt, &api_key).await {
                return schema;
            }
        }

        // Priority 2: Ollama (local) — set OLLAMA_SCAFFOLD_MODEL to enable
        // e.g. OLLAMA_SCAFFOLD_MODEL=qwen2.5-coder
        if let Ok(ollama_model) = std::env::var("OLLAMA_SCAFFOLD_MODEL") {
            tracing::info!("🦙 [AiScaffolder] Using Ollama model '{}' for schema generation", ollama_model);
            if let Some(schema) = Self::generate_schema_via_ollama(model_name, prompt, &ollama_model).await {
                return schema;
            }
        }

        // Priority 3: rule-based mock (offline fallback)
        Self::generate_schema_mock(model_name, prompt)
    }

    /// Ask a local Ollama model to emit a JSON schema definition, then parse it.
    async fn generate_schema_via_ollama(
        model_name: &str,
        prompt: &str,
        ollama_model: &str,
    ) -> Option<crate::model::schema::ModelSchema> {
        use crate::ai::local_llm::LocalLlmEngine;

        let engine = LocalLlmEngine::new_ollama(ollama_model);

        let system_prompt = format!(
            "You are a Vella framework schema designer. \
             Given a model name and description, output ONLY a valid JSON object with this shape:\n\
             {{\"fields\": [{{\"name\": \"title\", \"type\": \"string\", \"required\": true, \"searchable\": true}}]}}\n\
             Supported types: string, integer, float, boolean, email, password, markdown, html, \
             image, file, json, vector, enum, foreign_key, money, datetime.\n\
             Do not add explanation. Output only the JSON object.\n\n\
             Model name: {}\nDescription: {}",
            model_name, prompt
        );

        let raw = match engine.generate(&system_prompt).await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!("🦙 [AiScaffolder] Ollama generation failed: {}. Falling back to mock.", e);
                return None;
            }
        };

        // Extract JSON from the response (Ollama sometimes wraps in markdown fences)
        let json_str = extract_json(&raw);

        match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(val) => {
                tracing::info!("🦙 [AiScaffolder] Ollama returned parseable JSON schema.");
                Some(json_to_schema(model_name, prompt, &val))
            }
            Err(e) => {
                tracing::warn!(
                    "🦙 [AiScaffolder] Could not parse Ollama JSON output ({}). \
                     Raw response: {}. Falling back to mock.",
                    e, &raw[..raw.len().min(200)]
                );
                None
            }
        }
    }

    pub fn generate_schema_mock(model_name: &str, prompt: &str) -> ModelSchema {
        let p_lower = prompt.to_lowercase();
        let name_clean = if model_name.is_empty() { "Entity" } else { model_name };
        let mut schema = ModelSchema::new(name_clean);

        // Category & Icon detection
        if p_lower.contains("user") || p_lower.contains("auth") || p_lower.contains("customer") {
            schema = schema.category("Auth & CRM").icon("users");
        } else if p_lower.contains("e-commerce") || p_lower.contains("product") || p_lower.contains("order") || p_lower.contains("shop") {
            schema = schema.category("E-Commerce").icon("shopping-cart");
        } else if p_lower.contains("blog") || p_lower.contains("article") || p_lower.contains("post") || p_lower.contains("content") {
            schema = schema.category("Content & CMS").icon("file-text");
        } else if p_lower.contains("billing") || p_lower.contains("invoice") || p_lower.contains("payment") || p_lower.contains("stripe") {
            schema = schema.category("Finance & Billing").icon("credit-card");
        } else if p_lower.contains("rag") || p_lower.contains("ai") || p_lower.contains("knowledge") || p_lower.contains("doc") {
            schema = schema.category("AI & Knowledge Base").icon("cpu");
        } else if p_lower.contains("gis") || p_lower.contains("spatial") || p_lower.contains("map") || p_lower.contains("location") {
            schema = schema.category("GIS & Spatial").icon("map");
        } else {
            schema = schema.category("General").icon("database");
        }

        schema.description = Some(format!("AI Scaffolding: {}", prompt));

        // 1. Identity & Name Fields
        if p_lower.contains("name") || p_lower.contains("title") || p_lower.contains("user") {
            if p_lower.contains("title") || p_lower.contains("article") || p_lower.contains("post") || p_lower.contains("product") {
                schema = schema.field(Field::string("title").required().searchable());
            } else {
                schema = schema.field(Field::string("name").required().searchable());
            }
        }

        if p_lower.contains("slug") {
            schema = schema.field(Field::string("slug").unique().searchable());
        }

        // 2. Auth & User Fields
        if p_lower.contains("email") || p_lower.contains("user") || p_lower.contains("customer") || p_lower.contains("oauth") {
            schema = schema.field(Field::email("email").required().unique().searchable());
        }

        if p_lower.contains("password") || (p_lower.contains("user") && !p_lower.contains("no-auth")) {
            schema = schema.field(Field::password("password_hash"));
        }

        if p_lower.contains("oauth") || p_lower.contains("google") || p_lower.contains("github") {
            schema = schema.field(Field::string("oauth_provider").help("OAuth Provider (google, github)"));
            schema = schema.field(Field::string("oauth_id").unique().help("Third-party OAuth Subject ID"));
        }

        // 3. Billing & Stripe
        if p_lower.contains("stripe") || p_lower.contains("billing") || p_lower.contains("subscription") {
            schema = schema.field(Field::string("stripe_customer_id").unique().help("Stripe Customer ID"));
            schema = schema.field(Field::string("stripe_subscription_id").help("Active Subscription ID"));
            schema = schema.field(Field::r#enum("billing_tier", vec!["Free", "Pro", "Enterprise"]));
        }

        if p_lower.contains("price") || p_lower.contains("money") || p_lower.contains("amount") || p_lower.contains("salary") {
            let field_name = if p_lower.contains("price") { "price" } else { "amount" };
            schema = schema.field(Field::money(field_name, "USD").required().filterable(true));
        }

        if p_lower.contains("discount") {
            schema = schema.field(Field::float("discount_percent").requires_approval().help("Discount percentage"));
        }

        // 4. Content / Rich Text / Markdown
        if p_lower.contains("markdown") || p_lower.contains("body") || p_lower.contains("content") {
            schema = schema.field(Field::markdown("content").help("Markdown formatted body"));
        }

        if p_lower.contains("html") || p_lower.contains("description") {
            schema = schema.field(Field::html("description").help("Rich text overview"));
        }

        if p_lower.contains("avatar") || p_lower.contains("image") || p_lower.contains("photo") {
            schema = schema.field(Field::image("image_url", "uploads/images"));
        }

        // 5. Vector Embeddings (RAG / AI)
        if p_lower.contains("vector") || p_lower.contains("embedding") || p_lower.contains("rag") || p_lower.contains("ai") || p_lower.contains("semantic") {
            let dimensions = if p_lower.contains("768") {
                768
            } else if p_lower.contains("384") {
                384
            } else if p_lower.contains("3072") {
                3072
            } else {
                1536 // Default OpenAI text-embedding-3-small / ada-002
            };
            schema = schema.field(Field::vector("embedding", dimensions));
        }

        // 5.1 GIS / Spatial Data
        if p_lower.contains("location") || p_lower.contains("coordinate") || p_lower.contains("gps") || p_lower.contains("point") {
            let field_name = if p_lower.contains("location") { "location" } else { "coordinates" };
            schema = schema.field(Field::point(field_name, 4326).spatial_indexed());
        }

        if p_lower.contains("boundary") || p_lower.contains("polygon") || p_lower.contains("area") || p_lower.contains("zone") {
            let field_name = if p_lower.contains("boundary") { "boundary" } else if p_lower.contains("zone") { "zone" } else { "area" };
            schema = schema.field(Field::polygon(field_name, 4326).spatial_indexed());
        }

        // 6. Status & Workflow
        if p_lower.contains("status") || p_lower.contains("state") {
            if p_lower.contains("article") || p_lower.contains("post") || p_lower.contains("cms") {
                schema = schema.field(Field::r#enum("status", vec!["Draft", "InReview", "Published", "Archived"]));
            } else if p_lower.contains("order") || p_lower.contains("invoice") {
                schema = schema.field(Field::r#enum("status", vec!["Pending", "Paid", "Fulfilled", "Cancelled"]));
            } else {
                schema = schema.field(Field::r#enum("status", vec!["Active", "Inactive", "Archived"]));
            }
        }

        // 7. Progress Bar / Metrics
        if p_lower.contains("progress") || p_lower.contains("stock") || p_lower.contains("score") {
            if p_lower.contains("stock") {
                schema = schema.field(Field::progress_bar("stock_quantity", 1000.0, "#3b82f6"));
            } else {
                schema = schema.field(Field::progress_bar("progress_percentage", 100.0, "#10b981"));
            }
        }

        // 8. Relations / Foreign Keys
        if p_lower.contains("author") || p_lower.contains("user_id") {
            schema = schema.field(Field::foreign_key("author_id", "User").help("Author relation"));
        }
        if p_lower.contains("category_id") || p_lower.contains("category") {
            schema = schema.field(Field::foreign_key("category_id", "Category").help("Category relation"));
        }

        // 9. Boolean flags
        if p_lower.contains("active") || p_lower.contains("is_active") {
            schema = schema.field(Field::boolean("is_active").default_value(serde_json::json!(true)));
        }
        if p_lower.contains("featured") || p_lower.contains("is_featured") {
            schema = schema.field(Field::boolean("is_featured").default_value(serde_json::json!(false)));
        }

        // Default with timestamps
        schema.with_timestamps()
    }

    /// Generate clean, copy-paste ready Rust code for main.rs
    pub fn generate_rust_code(schema: &ModelSchema) -> String {
        let mut lines = Vec::new();
        lines.push(format!("    let {}_schema = ModelSchema::new(\"{}\")", schema.name.to_lowercase(), schema.name));
        lines.push(format!("        .category(\"{}\")", schema.category));
        lines.push(format!("        .icon(\"{}\")", schema.icon));

        if let Some(ref desc) = schema.description {
            lines.push(format!("        .description(\"{}\")", desc.replace('"', "\\\"")));
        }

        for field in &schema.fields {
            if field.name == "id" || field.name == "created_at" || field.name == "updated_at" {
                continue;
            }

            let mut f_line = match &field.field_type {
                FieldType::String => format!("Field::string(\"{}\")", field.name),
                FieldType::Integer => format!("Field::integer(\"{}\")", field.name),
                FieldType::Float => format!("Field::float(\"{}\")", field.name),
                FieldType::Boolean => format!("Field::boolean(\"{}\")", field.name),
                FieldType::DateTime => format!("Field::datetime(\"{}\")", field.name),
                FieldType::Email => format!("Field::email(\"{}\")", field.name),
                FieldType::Password => format!("Field::password(\"{}\")", field.name),
                FieldType::Html => format!("Field::html(\"{}\")", field.name),
                FieldType::Markdown => format!("Field::markdown(\"{}\")", field.name),
                FieldType::Money { currency } => format!("Field::money(\"{}\", \"{}\")", field.name, currency),
                FieldType::ProgressBar { max, color } => format!("Field::progress_bar(\"{}\", {:.1}, \"{}\")", field.name, max, color),
                FieldType::Image { upload_dir } => format!("Field::image(\"{}\", \"{}\")", field.name, upload_dir),
                FieldType::File { upload_dir } => format!("Field::file(\"{}\", \"{}\")", field.name, upload_dir),
                FieldType::ForeignKey { target_model } => format!("Field::foreign_key(\"{}\", \"{}\")", field.name, target_model),
                FieldType::Enum { choices } => {
                    let items = choices.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
                    format!("Field::r#enum(\"{}\", vec![{}])", field.name, items)
                }
                FieldType::Json => format!("Field::json(\"{}\")", field.name),
                FieldType::Vector { dimensions } => format!("Field::vector(\"{}\", {})", field.name, dimensions),
                FieldType::Point { srid } => format!("Field::point(\"{}\", {})", field.name, srid),
                FieldType::Polygon { srid } => format!("Field::polygon(\"{}\", {})", field.name, srid),
                FieldType::Geometry { geom_type, srid } => format!("Field::geometry(\"{}\", \"{}\", {})", field.name, geom_type, srid),
                FieldType::Crdt => format!("Field::crdt(\"{}\")", field.name),
                FieldType::Web3Address => format!("Field::web3_address(\"{}\")", field.name),
            };

            if field.required && field.field_type != FieldType::Password {
                f_line.push_str(".required()");
            }
            if field.unique {
                f_line.push_str(".unique()");
            }
            if field.searchable && field.field_type != FieldType::Email {
                f_line.push_str(".searchable()");
            }
            if field.requires_approval {
                f_line.push_str(".requires_approval()");
            }
            if field.spatial_indexed {
                f_line.push_str(".spatial_indexed()");
            }
            if let Some(ref help) = field.help_text {
                f_line.push_str(&format!(".help(\"{}\")", help.replace('"', "\\\"")));
            }

            lines.push(format!("        .field({})", f_line));
        }

        lines.push("        .with_timestamps();".to_string());
        lines.join("\n")
    }

    /// Generate complete scaffold result with detected features
    pub async fn scaffold(model_name: &str, prompt: &str, db_type: DatabaseType) -> GeneratedScaffoldResult {
        let schema = Self::generate_schema(model_name, prompt).await;
        let rust_code = Self::generate_rust_code(&schema);
        let sql_ddl = crate::db::SqlDialect::create_table_ddl(db_type, &schema);
        let typescript_definition = crate::types::TypeScriptGenerator::generate_model_interface(&schema);

        let mut features = Vec::new();
        if schema.has_vectors() {
            features.push("LLM Vector Embeddings (RAG Ready)".to_string());
        }
        if schema.fields.iter().any(|f| matches!(f.field_type, FieldType::Point { .. } | FieldType::Polygon { .. } | FieldType::Geometry { .. })) {
            features.push("GIS Spatial Capabilities (PostGIS / Spatialite)".to_string());
        }
        if schema.fields.iter().any(|f| f.requires_approval) {
            features.push("AI Approval Workflow Queue".to_string());
        }
        if schema.fields.iter().any(|f| f.name.contains("stripe") || f.name.contains("billing")) {
            features.push("Stripe Billing Integration".to_string());
        }
        if schema.fields.iter().any(|f| f.name.contains("oauth")) {
            features.push("OAuth 2.0 Single Sign-On".to_string());
        }
        if schema.fields.iter().any(|f| f.searchable) {
            features.push("Indexed Full-Text Search".to_string());
        }

        GeneratedScaffoldResult {
            schema,
            rust_code,
            sql_ddl,
            typescript_definition,
            detected_features: features,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers for Ollama JSON → ModelSchema parsing
// ---------------------------------------------------------------------------

/// Strip markdown code fences (` ```json … ``` `) from Ollama output and
/// extract the first JSON object found.
fn extract_json(raw: &str) -> String {
    // Remove ```json / ``` fences if present
    let stripped = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    // If there's a { ... } block, extract it
    if let (Some(start), Some(end)) = (stripped.find('{'), stripped.rfind('}')) {
        stripped[start..=end].to_string()
    } else {
        stripped.to_string()
    }
}

/// Convert the JSON value returned by an Ollama schema-generation call into a
/// `ModelSchema`.  Falls back gracefully when the model omits optional keys.
fn json_to_schema(model_name: &str, prompt: &str, val: &serde_json::Value) -> ModelSchema {
    let mut schema = ModelSchema::new(model_name)
        .description(&format!("AI Scaffolding (Ollama): {}", prompt));

    if let Some(fields) = val["fields"].as_array() {
        for f in fields {
            let name = match f["name"].as_str() {
                Some(n) => n,
                None => continue,
            };
            let type_str = f["type"].as_str().unwrap_or("string");
            let required = f["required"].as_bool().unwrap_or(false);
            let searchable = f["searchable"].as_bool().unwrap_or(false);
            let unique = f["unique"].as_bool().unwrap_or(false);

            let mut field = match type_str {
                "integer" | "int" => Field::integer(name),
                "float" | "number" => Field::float(name),
                "boolean" | "bool" => Field::boolean(name),
                "email" => Field::email(name),
                "password" => Field::password(name),
                "markdown" => Field::markdown(name),
                "html" => Field::html(name),
                "json" => Field::json(name),
                "datetime" | "date" => Field::datetime(name),
                "vector" => {
                    let dims = f["dimensions"].as_u64().unwrap_or(768) as usize;
                    Field::vector(name, dims)
                }
                "enum" => {
                    let choices: Vec<&str> = f["choices"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                        .unwrap_or_else(|| vec!["Option1", "Option2"]);
                    Field::r#enum(name, choices)
                }
                "foreign_key" | "relation" => {
                    let target = f["target"].as_str().unwrap_or("Unknown");
                    Field::foreign_key(name, target)
                }
                "money" => {
                    let currency = f["currency"].as_str().unwrap_or("USD");
                    Field::money(name, currency)
                }
                _ => Field::string(name), // default to string
            };

            if required {
                field = field.required();
            }
            if searchable {
                field = field.searchable();
            }
            if unique {
                field = field.unique();
            }

            schema = schema.field(field);
        }
    }

    schema.with_timestamps()
}
