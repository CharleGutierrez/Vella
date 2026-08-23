use serde_json::json;
use vella::core::wasm::{edge_cosine_similarity, edge_parse_query_filters, edge_validate_payload};
use vella::model::{Field, ModelSchema};

#[test]
fn test_wasm_edge_vector_math() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![1.0, 2.0, 3.0];
    let sim = edge_cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 1e-5);
}

#[test]
fn test_wasm_edge_schema_validator() {
    let schema = ModelSchema::new("Doc")
        .field(Field::string("title").required())
        .field(Field::vector("embedding", 3).required())
        .with_timestamps();

    // Valid payload
    let valid = json!({
        "title": "Edge Computing in WASM",
        "embedding": [0.1, 0.5, 0.9]
    });
    assert!(edge_validate_payload(&schema, &valid).is_ok());

    // Invalid payload (wrong vector dimensionality)
    let invalid_dim = json!({
        "title": "Edge Computing in WASM",
        "embedding": [0.1, 0.5]
    });
    assert!(edge_validate_payload(&schema, &invalid_dim).is_err());

    // Missing required field
    let missing_title = json!({
        "embedding": [0.1, 0.5, 0.9]
    });
    assert!(edge_validate_payload(&schema, &missing_title).is_err());
}

#[test]
fn test_wasm_edge_query_filter_parsing() {
    let qs = "category=DevOps&status__neq=Archived&price__gte=50";
    let filters = edge_parse_query_filters(qs);

    assert_eq!(filters.len(), 3);
    assert_eq!(filters[0], ("category".to_string(), "eq".to_string(), "DevOps".to_string()));
    assert_eq!(filters[1], ("status".to_string(), "neq".to_string(), "Archived".to_string()));
    assert_eq!(filters[2], ("price".to_string(), "gte".to_string(), "50".to_string()));
}
