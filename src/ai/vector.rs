use crate::core::error::VellaError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Supported distance metrics for vector similarity search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

impl Default for DistanceMetric {
    fn default() -> Self {
        Self::Cosine
    }
}

/// A request for vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchQuery {
    pub model: String,
    #[serde(default = "default_vector_field")]
    pub vector_field: String,
    pub query_vector: Vec<f32>,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    #[serde(default)]
    pub metric: DistanceMetric,
}

fn default_vector_field() -> String {
    "embedding".to_string()
}

fn default_top_k() -> usize {
    10
}

/// A single matched record from a vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: i64,
    pub score: f32, // Similarity score (higher is closer for cosine/dot, lower for euclidean)
    pub record: Value,
}

/// Calculate cosine similarity between two float vectors (-1.0 to 1.0)
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || a.len() != b.len() {
        return 0.0;
    }

    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a.sqrt() * norm_b.sqrt())
}

/// Calculate Euclidean (L2) distance between two vectors
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || a.len() != b.len() {
        return f32::MAX;
    }

    let mut sum_sq = 0.0f32;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum_sq += diff * diff;
    }

    sum_sq.sqrt()
}

/// Calculate Dot Product between two vectors
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || a.len() != b.len() {
        return 0.0;
    }

    let mut dot = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
    }
    dot
}

/// Parse a vector from a JSON Value (array of numbers or JSON string or comma-separated string)
pub fn parse_vector_from_json(val: &Value) -> Result<Vec<f32>, VellaError> {
    match val {
        Value::Array(arr) => {
            let mut vec = Vec::with_capacity(arr.len());
            for item in arr {
                if let Some(num) = item.as_f64() {
                    vec.push(num as f32);
                } else if let Some(s) = item.as_str() {
                    let num = s.parse::<f32>().map_err(|_| {
                        VellaError::VectorError(format!("Invalid vector element: {}", s))
                    })?;
                    vec.push(num);
                } else {
                    return Err(VellaError::VectorError("Vector elements must be numbers".to_string()));
                }
            }
            Ok(vec)
        }
        Value::String(s) => {
            if s.starts_with('[') && s.ends_with(']') {
                let parsed: Vec<f32> = serde_json::from_str(s).map_err(|e| {
                    VellaError::VectorError(format!("Failed to parse JSON vector string: {}", e))
                })?;
                Ok(parsed)
            } else {
                let mut vec = Vec::new();
                for part in s.split(',') {
                    let trimmed = part.trim();
                    if !trimmed.is_empty() {
                        let num = trimmed.parse::<f32>().map_err(|_| {
                            VellaError::VectorError(format!("Invalid vector element: {}", trimmed))
                        })?;
                        vec.push(num);
                    }
                }
                Ok(vec)
            }
        }
        _ => Err(VellaError::VectorError("Value cannot be parsed as a vector".to_string())),
    }
}

/// Format vector for PostgreSQL pgvector literal: '[0.1, 0.2, 0.3]'
pub fn format_pgvector_literal(vec: &[f32]) -> String {
    let parts: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
    format!("[{}]", parts.join(","))
}
