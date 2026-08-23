use vella::ai::generator::AiScaffolder;
use vella::db::DatabaseType;
use vella::model::FieldType;

#[test]
fn test_agentic_ai_scaffolder_user_billing_oauth() {
    let prompt = "A user with stripe billing and OAuth";
    let scaffold = AiScaffolder::scaffold("User", prompt, DatabaseType::Sqlite);

    assert_eq!(scaffold.schema.name, "User");
    assert_eq!(scaffold.schema.category, "Auth & CRM");

    // Check fields
    assert!(scaffold.schema.get_field("email").is_some());
    assert!(scaffold.schema.get_field("oauth_provider").is_some());
    assert!(scaffold.schema.get_field("oauth_id").is_some());
    assert!(scaffold.schema.get_field("stripe_customer_id").is_some());
    assert!(scaffold.schema.get_field("billing_tier").is_some());

    // Check code generation
    assert!(scaffold.rust_code.contains("ModelSchema::new(\"User\")"));
    assert!(scaffold.rust_code.contains("Field::email(\"email\")"));
    assert!(scaffold.rust_code.contains("Field::string(\"stripe_customer_id\")"));

    // Check TypeScript
    assert!(scaffold.typescript_definition.contains("export interface User {"));
    assert!(scaffold.typescript_definition.contains("email: string;"));
    assert!(scaffold.typescript_definition.contains("billing_tier?: 'Free' | 'Pro' | 'Enterprise';"));
}

#[test]
fn test_agentic_ai_scaffolder_rag_vector_doc() {
    let prompt = "A technical article with markdown content, published status, and 1536 vector embeddings for semantic search";
    let scaffold = AiScaffolder::scaffold("Article", prompt, DatabaseType::Postgres);

    assert_eq!(scaffold.schema.name, "Article");
    assert!(scaffold.schema.has_vectors());

    let vec_field = scaffold.schema.get_field("embedding").unwrap();
    assert_eq!(vec_field.field_type, FieldType::Vector { dimensions: 1536 });

    // In Postgres, check pgvector DDL & HNSW index
    assert!(scaffold.sql_ddl.contains("CREATE EXTENSION IF NOT EXISTS vector;"));
    assert!(scaffold.sql_ddl.contains("\"embedding\" vector(1536)"));
    assert!(scaffold.sql_ddl.contains("USING hnsw (\"embedding\" vector_cosine_ops)"));

    // Check TypeScript vector array
    assert!(scaffold.typescript_definition.contains("embedding?: number[];"));
    assert!(scaffold.typescript_definition.contains("status?: 'Draft' | 'InReview' | 'Published' | 'Archived';"));
}
