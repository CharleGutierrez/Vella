use crate::model::schema::ModelSchema;
use crate::model::field::{Field, FieldType};

pub async fn call_gemini_schema(model_name: &str, prompt: &str, api_key: &str) -> Option<ModelSchema> {
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
    
    let schema_definition = r#"
Generate a JSON object matching this Rust struct for a database schema:

{
  "name": "string (Model Name)",
  "table_name": "string (snake_case plural)",
  "display_name": "string",
  "category": "string (e.g. 'E-Commerce')",
  "icon": "string (a lucide icon name like 'users', 'shopping-cart')",
  "description": "string",
  "fields": [
    {
      "name": "string (field name)",
      "display_name": "string",
      "field_type": { "kind": "String" } | { "kind": "Integer" } | { "kind": "Money", "config": { "currency": "USD" } } | { "kind": "Vector", "config": { "dimensions": 1536 } },
      "required": boolean,
      "unique": boolean,
      "searchable": boolean,
      "filterable": boolean,
      "list_display": boolean,
      "read_only": boolean,
      "encrypted": boolean,
      "requires_approval": boolean,
      "spatial_indexed": boolean,
      "default_value": null,
      "help_text": "string or null"
    }
  ]
}

Only valid kind values: String, Integer, Float, Boolean, DateTime, Email, Password, Html, Markdown, Money, Vector, Point, CRDT.

Output ONLY valid JSON without markdown wrapping.
"#;

    let payload = serde_json::json!({
        "contents": [{
            "parts": [{"text": format!("{}\n\nModel: {}\nPrompt: {}", schema_definition, model_name, prompt)}]
        }]
    });

    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .ok()?;

    let json: serde_json::Value = res.json().await.ok()?;
    let text = json["candidates"][0]["content"]["parts"][0]["text"].as_str()?;
    
    // Clean up potential markdown blocks
    let text = text.trim();
    let text = if text.starts_with("`json") {
        text.strip_prefix("`json").unwrap().strip_suffix("`").unwrap_or(text).trim()
    } else if text.starts_with("`") {
        text.strip_prefix("`").unwrap().strip_suffix("`").unwrap_or(text).trim()
    } else {
        text
    };

    serde_json::from_str::<ModelSchema>(text).ok()
}

pub async fn call_gemini_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY not set")?;
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}", api_key);
    
    let payload = serde_json::json!({
        "model": "models/text-embedding-004",
        "content": {
            "parts": [{"text": text}]
        }
    });

    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    
    let embedding = json["embedding"]["values"]
        .as_array()
        .ok_or("Failed to parse embedding")?
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect::<Vec<f32>>();
        
    Ok(embedding)
}
