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
    pub fn generate_schema(model_name: &str, prompt: &str) -> ModelSchema {
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
            if let Some(ref help) = field.help_text {
                f_line.push_str(&format!(".help(\"{}\")", help.replace('"', "\\\"")));
            }

            lines.push(format!("        .field({})", f_line));
        }

        lines.push("        .with_timestamps();".to_string());
        lines.join("\n")
    }

    /// Generate complete scaffold result with detected features
    pub fn scaffold(model_name: &str, prompt: &str, db_type: DatabaseType) -> GeneratedScaffoldResult {
        let schema = Self::generate_schema(model_name, prompt);
        let rust_code = Self::generate_rust_code(&schema);
        let sql_ddl = crate::db::SqlDialect::create_table_ddl(db_type, &schema);
        let typescript_definition = crate::types::TypeScriptGenerator::generate_model_interface(&schema);

        let mut features = Vec::new();
        if schema.has_vectors() {
            features.push("LLM Vector Embeddings (RAG Ready)".to_string());
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
