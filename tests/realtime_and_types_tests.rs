use serde_json::json;
use std::fs;
use vella::core::events::{EventBus, SystemEvent};
use vella::model::{Field, ModelSchema, SchemaRegistry};
use vella::realtime::RealtimeHub;
use vella::types::TypeScriptGenerator;
use vella::ui::{angular_sdk, react_sdk, vue_sdk};

#[test]
fn test_typescript_definitions_generation_and_export() {
    let mut registry = SchemaRegistry::new();
    let product_schema = ModelSchema::new("Product")
        .field(Field::string("name").required().searchable())
        .field(Field::money("price", "USD").required())
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        .field(Field::vector("embedding", 1536))
        .field(Field::boolean("is_featured"))
        .with_timestamps();

    registry.register(product_schema);

    let dts = TypeScriptGenerator::generate_full_definitions(&registry);

    assert!(dts.contains("export interface Product {"));
    assert!(dts.contains("name: string;"));
    assert!(dts.contains("price: number;"));
    assert!(dts.contains("status?: 'Draft' | 'Published' | 'Archived';"));
    assert!(dts.contains("embedding?: number[];"));
    assert!(dts.contains("is_featured?: boolean;"));
    assert!(dts.contains("export interface VellaModelMap {"));
    assert!(dts.contains("'product': Product;"));

    let export_path = "target/types_test/vella.d.ts";
    TypeScriptGenerator::export_to_file(export_path, &registry).unwrap();
    assert!(fs::metadata(export_path).is_ok());
}

#[tokio::main]
#[test]
async fn test_realtime_event_bridge_broadcast() {
    let event_bus = EventBus::default();
    let hub = RealtimeHub::default();
    hub.start_event_bridge(&event_bus, None);

    let mut rx = hub.subscribe();

    // Publish model creation event to event bus
    event_bus.publish(SystemEvent::RecordCreated {
        model: "Product".to_string(),
        id: 42,
        data: json!({ "id": 42, "name": "Mechanical Keyboard" }),
    });

    // Realtime hub should bridge and format the message
    let msg = tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv())
        .await
        .expect("Realtime message received")
        .expect("Ok message");

    assert_eq!(msg.topic, "models:product");
    assert_eq!(msg.event, "CREATE");
    assert_eq!(msg.payload.get("id").unwrap().as_i64().unwrap(), 42);
}

#[test]
fn test_frontend_sdks_generation() {
    let base_url = "http://localhost:8080";

    let react_code = react_sdk::generate_react_sdk(base_url);
    assert!(react_code.contains("export class VellaClient"));
    assert!(react_code.contains("useRealtimeSubscription"));
    assert!(react_code.contains("searchVector"));
    assert!(react_code.contains("ragQuery"));

    let vue_code = vue_sdk::generate_vue_sdk(base_url);
    assert!(vue_code.contains("export class VellaClient"));
    assert!(vue_code.contains("useVellaQuery"));

    let ng_code = angular_sdk::generate_angular_sdk(base_url);
    assert!(ng_code.contains("export class VellaService"));
    assert!(ng_code.contains("providedIn: 'root'"));
}
