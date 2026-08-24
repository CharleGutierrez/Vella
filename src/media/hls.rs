use tracing::{info, warn};

pub enum DrmProvider {
    Widevine,
    FairPlay,
    PlayReady,
}

pub struct HlsManifestGenerator {
    base_cdn_url: String,
    drm: Option<DrmProvider>,
}

impl HlsManifestGenerator {
    pub fn new(base_cdn_url: &str, drm: Option<DrmProvider>) -> Self {
        info!("Initializing HTTP Live Streaming (HLS) Engine with CDN: {}", base_cdn_url);
        Self {
            base_cdn_url: base_cdn_url.to_string(),
            drm,
        }
    }

    /// Generates an adaptive bitrate .m3u8 master playlist
    pub fn generate_master_playlist(&self, video_id: &str) -> String {
        info!("Generating Adaptive Bitrate HLS Manifest for Video: {}", video_id);
        
        let mut manifest = String::from("#EXTM3U\n");
        manifest.push_str("#EXT-X-VERSION:6\n");
        
        if let Some(ref drm) = self.drm {
            let drm_tag = match drm {
                DrmProvider::Widevine => "urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed",
                DrmProvider::FairPlay => "urn:uuid:94ce86fb-07bd-40fe-9a18-52c6f1f50aeb",
                DrmProvider::PlayReady => "urn:uuid:9a04f079-9840-4286-ab92-e65be0885f95",
            };
            warn!("DRM Enabled. Injecting EXT-X-KEY with SystemID: {}", drm_tag);
            manifest.push_str(&format!("#EXT-X-KEY:METHOD=SAMPLE-AES,URI=\"{}/license/{}\",KEYFORMAT=\"urn:uuid:{}\"\n", self.base_cdn_url, video_id, drm_tag));
        }

        // Mock 1080p and 4K streams
        manifest.push_str(&format!("#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1920x1080\n{}/1080p.m3u8\n", self.base_cdn_url));
        manifest.push_str(&format!("#EXT-X-STREAM-INF:BANDWIDTH=15000000,RESOLUTION=3840x2160\n{}/4k.m3u8\n", self.base_cdn_url));
        
        manifest
    }
}
