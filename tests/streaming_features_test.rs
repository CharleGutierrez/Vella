use vella::db::CassandraAdapter;
use vella::db::GraphTraversalBuilder;
use vella::media::HlsManifestGenerator;
use vella::media::DrmProvider;
use vella::api::cdn::CdnManager;
use vella::core::chaos::ChaosMonkeyMiddleware;
use vella::ai::vision::VisionPipeline;
use std::time::Instant;

#[test]
fn test_cassandra_multi_master_query() {
    let adapter = CassandraAdapter::new(vec!["10.0.0.1", "10.0.0.2"], "us-east");
    let query = adapter.execute_wide_column_query("netflix", "user_history", "user_123");
    
    assert!(query.contains("LOCAL_QUORUM"), "Cassandra read must enforce distributed quorum");
    assert!(query.contains("partition_id = 'user_123'"), "Partition key routing missing");
}

#[test]
fn test_graph_database_traversal() {
    let graph = GraphTraversalBuilder::new();
    let cypher = graph.build_recommendation_traversal("user_999", 4);
    
    assert!(cypher.contains("MATCH (u:User {id: 'user_999'})"), "Cypher missing anchor node");
    assert!(cypher.contains("shortestPath"), "Cypher missing degree depth boundary");
}

#[test]
fn test_hls_drm_streaming_manifest() {
    let hls = HlsManifestGenerator::new("https://cdn.vella.dev", Some(DrmProvider::Widevine));
    let manifest = hls.generate_master_playlist("stranger_things_s04e01");
    
    assert!(manifest.contains("#EXTM3U"), "Missing HLS header");
    assert!(manifest.contains("EXT-X-KEY"), "Missing DRM key tag");
    assert!(manifest.contains("Widevine") || manifest.contains("edef8ba9-79d6-4ace-a3c8-27dcd51d21ed"), "Missing Widevine SystemID");
    assert!(manifest.contains("RESOLUTION=3840x2160"), "Missing 4K adaptive stream");
}

#[tokio::test]
async fn test_cdn_edge_invalidation() {
    let cdn = CdnManager::new("https://api.cloudflare.com/client/v4/zones/xyz/purge_cache");
    // Just testing the structure doesn't panic on instantiation and async call
    cdn.purge_cache_key("movies_page_1").await;
}

#[tokio::test]
async fn test_chaos_engineering_middleware() {
    // 100% failure rate, 50ms latency max
    let chaos = ChaosMonkeyMiddleware::new(1.0, 50);
    
    let start = Instant::now();
    let result = chaos.inject_chaos().await;
    
    if let Err(e) = result {
        assert_eq!(e, "Simulated Network Partition");
    } else {
        assert!(start.elapsed().as_millis() >= 10, "Latency injection failed");
    }
}

#[test]
fn test_computer_vision_pipelines() {
    let vision = VisionPipeline::new("./models/resnet50.gguf");
    
    let intro_end = vision.analyze_intro_sequence("video_abc");
    assert!(intro_end.is_some(), "Computer vision failed to find Skip Intro timestamp");
    
    let thumb = vision.extract_smart_thumbnail("video_abc");
    assert!(thumb.starts_with(b"JPEG"), "Thumbnail extraction failed to return valid image bytes");
}
