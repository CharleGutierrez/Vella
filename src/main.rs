use clap::{Parser, Subcommand};
use vella::ai::generator::AiScaffolder;
use vella::db::DatabaseType;
use vella::prelude::*;

#[derive(Parser)]
#[command(name = "vella")]
#[command(about = "⚡ Vella: LLM-Native Rust Web Framework & Headless CMS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Vella Server and Headless CMS
    Serve {
        #[arg(short, long, default_value = "0.0.0.0:8080")]
        bind: String,

        #[arg(short, long, default_value = "sqlite://vella.db?mode=rwc")]
        database: String,
    },
    /// Agentic AI Scaffolding CLI: Generate schemas from natural language
    Generate {
        #[command(subcommand)]
        generator_type: GenerateCommands,
    },
    /// Export zero-config TypeScript definitions directly to frontend
    ExportTypes {
        #[arg(short, long, default_value = "./frontend/types/vella.d.ts")]
        output: String,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate a model schema from natural language
    Model {
        /// Name of the model (e.g. User, Article, Product)
        name: String,

        /// Natural language description for the AI Tuner
        #[arg(long)]
        ai: String,

        /// Database dialect (sqlite, postgres, mysql)
        #[arg(short, long, default_value = "sqlite")]
        database: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate { generator_type }) => match generator_type {
            GenerateCommands::Model { name, ai, database } => {
                let db_type = match database.to_lowercase().as_str() {
                    "postgres" | "postgresql" => DatabaseType::Postgres,
                    "mysql" | "mariadb" => DatabaseType::MySql,
                    _ => DatabaseType::Sqlite,
                };

                println!("\n✨ [Vella Agentic Scaffolder] Generating model '{}' from prompt:\n   \"{}\"\n", name, ai);
                let scaffold = AiScaffolder::scaffold(&name, &ai, db_type);

                println!("📦 Detected Features:");
                for f in &scaffold.detected_features {
                    println!("   • {}", f);
                }

                println!("\n🦀 Rust Builder Code (Copy into main.rs):");
                println!("--------------------------------------------------");
                println!("{}\n", scaffold.rust_code);

                println!("🗄️ SQL DDL Migration ({}):", db_type.name());
                println!("--------------------------------------------------");
                println!("{}\n", scaffold.sql_ddl);

                println!("📘 TypeScript Definition:");
                println!("--------------------------------------------------");
                println!("{}\n", scaffold.typescript_definition);
            }
        },
        Some(Commands::ExportTypes { output }) => {
            let mut registry = SchemaRegistry::new();
            let sample_schema = ModelSchema::new("Article")
                .field(Field::string("title").required().searchable())
                .field(Field::markdown("content"))
                .field(Field::vector("embedding", 1536))
                .field(Field::r#enum("status", vec!["Draft", "Published"]))
                .with_timestamps();
            registry.register(sample_schema);

            TypeScriptGenerator::export_to_file(&output, &registry)?;
            println!("✨ Successfully exported TypeScript definitions to: {}", output);
        }
        Some(Commands::Serve { bind, database }) => {
            run_server(&bind, &database).await?;
        }
        None => {
            run_server("0.0.0.0:8080", "sqlite://vella.db?mode=rwc").await?;
        }
    }

    Ok(())
}

async fn run_server(bind: &str, database: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Default sample demo app
    let article_schema = ModelSchema::new("Article")
        .category("Content & CMS")
        .icon("file-text")
        .description("Articles and Knowledge Base with Vector Embeddings")
        .field(Field::string("title").required().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::markdown("content").help("Markdown formatted article body"))
        .field(Field::vector("embedding", 1536).help("OpenAI text-embedding-3-small vector"))
        .field(Field::r#enum("status", vec!["Draft", "InReview", "Published", "Archived"]).filterable(true))
        .field(Field::boolean("is_featured").default_value(serde_json::json!(false)))
        .with_timestamps();

    let user_schema = ModelSchema::new("User")
        .category("Auth & CRM")
        .icon("users")
        .description("Platform Users and Billing Profiles")
        .field(Field::string("name").required().searchable())
        .field(Field::email("email").required().unique().searchable())
        .field(Field::string("stripe_customer_id").unique().help("Stripe Customer ID"))
        .field(Field::r#enum("billing_tier", vec!["Free", "Pro", "Enterprise"]))
        .field(Field::money("balance", "USD").filterable(true))
        .field(Field::float("discount_rate").requires_approval().help("Requires Manager approval"))
        .with_timestamps();

    VellaApp::new()
        .site_name("Vella Cloud")
        .bind(bind)
        .database(database)
        .register(article_schema)
        .register(user_schema)
        .run()
        .await?;

    Ok(())
}
