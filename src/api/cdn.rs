use tracing::info;

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
