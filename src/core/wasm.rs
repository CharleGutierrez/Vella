/// WebAssembly (WASM32-WASI & Edge) Compatibility Layer for Vella.
/// Enables Vella core validation, vector calculations, schema introspection,
/// and query parsing in serverless Edge runtimes (Cloudflare Workers, Fermyon Spin, Fastly Compute).

use crate::ai::vector::cosine_similarity;
use crate::model::validator::FieldValidator;
use crate::model::ModelSchema;
use serde_json::Value;

/// Standalone edge vector similarity calculator (no OS/native dependencies)
pub fn edge_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    cosine_similarity(a, b)
}

/// Pure edge validator for incoming JSON payloads against model schemas
pub fn edge_validate_payload(schema: &ModelSchema, payload: &Value) -> Result<(), String> {
    let obj = match payload {
        Value::Object(map) => map,
        _ => return Err("Payload must be a JSON object".to_string()),
    };

    for field in &schema.fields {
        FieldValidator::validate_field(field, obj.get(&field.name))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Standalone edge query filter extractor
pub fn edge_parse_query_filters(query_string: &str) -> Vec<(String, String, String)> {
    let mut clauses = Vec::new();
    for pair in query_string.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let val = parts.next().unwrap_or("");

        if !key.starts_with('$') && !key.is_empty() {
            if let Some(pos) = key.find("__") {
                let field = &key[..pos];
                let op = &key[pos + 2..];
                clauses.push((field.to_string(), op.to_string(), val.to_string()));
            } else {
                clauses.push((key.to_string(), "eq".to_string(), val.to_string()));
            }
        }
    }
    clauses
}

/// Is running inside a WebAssembly environment
pub fn is_wasm_runtime() -> bool {
    cfg!(target_arch = "wasm32")
}
