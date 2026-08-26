use tracing::info;
use axum::{extract::Multipart, response::Json};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use std::io::Write;

pub struct CdnManager {
    edge_provider_url: String,
}

impl CdnManager {
    pub fn new(edge_provider_url: &str) -> Self {
        info!("Initializing CDN Edge Manager mapped to {}", edge_provider_url);
        Self { edge_provider_url: edge_provider_url.to_string() }
    }

    /// Automatically fires when a CMS record is updated
    pub async fn purge_cache_key(&self, cache_key: &str) {
        info!("Broadcasting HTTP PURGE to {} for Cache Key: {}", self.edge_provider_url, cache_key);
        // Simulation of sending an HTTP PURGE or BAN request to Akamai/Fastly/Cloudflare
        // let client = reqwest::Client::new();
        // client.request(Method::from_bytes(b"PURGE").unwrap(), &self.edge_provider_url).header("Cache-Tag", cache_key).send().await;
    }
}

pub async fn upload_handler(mut multipart: Multipart) -> Json<Value> {
    let upload_dir = "./uploads";
    if !Path::new(upload_dir).exists() {
        let _ = fs::create_dir_all(upload_dir);
    }

    let mut file_urls = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        if file_name == "unknown" && field.name().is_none() {
            continue;
        }
        let unique_name = format!("{}_{}", Uuid::new_v4(), file_name);
        let file_path = format!("{}/{}", upload_dir, unique_name);
        
        let data = field.bytes().await.unwrap_or_default();
        
        if let Ok(mut file) = std::fs::File::create(&file_path) {
            let _ = file.write_all(&data);
            file_urls.push(format!("/uploads/{}", unique_name));
        }
    }

    if file_urls.len() == 1 {
        Json(json!({ "url": file_urls[0] }))
    } else {
        Json(json!({ "urls": file_urls }))
    }
}
