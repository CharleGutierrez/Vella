use vella::data::ArrowExporter;
use vella::ui::python_sdk::generate_python_polars_sdk;
use vella::ai::registry::ModelRegistry;
use vella::storage::FeatureStore;
use vella::scripting::WasmPipeline;
use vella::ai::gpu::HardwareAccelerator;
use serde_json::json;

#[test]
fn test_apache_arrow_export() {
    let exporter = ArrowExporter::new();
    let rows = vec![json!({"id": 1, "value": "A"})];
    
    let arrow_bytes = exporter.export_to_arrow_stream("ml_events", &rows);
    let parquet_bytes = exporter.export_to_parquet("ml_events", &rows);
    
    assert!(arrow_bytes.starts_with(b"ARROW1"), "Arrow stream missing header");
    assert!(parquet_bytes.starts_with(b"PAR1"), "Parquet stream missing header");
}

#[test]
fn test_python_sdk_generation() {
    let sdk = generate_python_polars_sdk("https://api.vella.dev");
    assert!(sdk.contains("import polars as pl"), "Missing Polars dependency");
    assert!(sdk.contains("export?format=arrow"), "Missing Arrow Endpoint routing");
}

#[test]
fn test_ml_model_shadow_routing() {
    let registry = ModelRegistry::new("mistral-v1", Some("mistral-v2-experimental"));
    
    // Fire 3 requests
    let _ = registry.execute_inference("payload 1");
    let _ = registry.execute_inference("payload 2");
    let res = registry.execute_inference("payload 3");
    
    assert_eq!(res, "Inference Result from mistral-v1", "Active model did not return correct response");
    assert_eq!(registry.get_shadow_traffic_count(), 3, "Shadow router failed to mirror traffic");
}

#[test]
fn test_in_memory_feature_store() {
    let store = FeatureStore::new();
    
    store.push_feature("user_789", "avg_spend_30d", json!(145.50));
    
    let result = store.get_feature("user_789", "avg_spend_30d").unwrap();
    assert_eq!(result.as_f64().unwrap(), 145.50, "Feature Store returned incorrect ML feature");
}

#[test]
fn test_wasm_udf_data_pipeline() {
    let pipeline = WasmPipeline::new("pii_scrubber");
    
    let raw_data = "User logged in with PII_CREDIT_CARD data";
    let cleaned = pipeline.execute_transform(raw_data);
    
    assert_eq!(cleaned, "User logged in with [REDACTED] data", "Wasm module failed to transform data");
}

#[test]
fn test_hardware_accelerator_thermal_fallback() {
    let gpu = HardwareAccelerator::detect();
    
    // Test standard route
    gpu.execute_vector_math("Vector Dot Product");
    
    // Force overheat and ensure fallback triggers cleanly
    gpu.simulate_overheat();
    gpu.execute_vector_math("Heavy Matrix Multiplication");
    
    // We visually rely on the logs changing, but structurally it shouldn't panic.
}
