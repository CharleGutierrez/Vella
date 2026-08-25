use vella::app::VellaApp;
use vella::model::{ModelSchema, Field, FieldType};

/// Google Earth Clone - Backend Engine
/// Run this with: `cargo run --example google_earth_clone`
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Booting Vella - Google Earth Spatial Engine...");
    
    // 1. Map Tiles (For rendering custom satellite imagery)
    let map_tiles = ModelSchema::new("MapTiles")
        .description("Satellite imagery XYZ tiles")
        .field(Field {
            name: "zoom_level".to_string(),
            display_name: "Zoom Level".to_string(),
            field_type: FieldType::Integer,
            required: true,
            unique: false, searchable: false, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "tile_x".to_string(),
            display_name: "Tile X".to_string(),
            field_type: FieldType::Integer,
            required: true,
            unique: false, searchable: false, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "tile_y".to_string(),
            display_name: "Tile Y".to_string(),
            field_type: FieldType::Integer,
            required: true,
            unique: false, searchable: false, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "image_blob".to_string(),
            display_name: "Image Blob".to_string(),
            field_type: FieldType::String, 
            required: false,
            unique: false, searchable: false, filterable: false, list_display: false,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .with_timestamps();

    // 2. Landmarks (3D Models and Points of Interest)
    let landmarks = ModelSchema::new("Landmarks")
        .description("Global Points of Interest (Eiffel Tower, Mt. Everest)")
        .field(Field {
            name: "name".to_string(),
            display_name: "Name".to_string(),
            field_type: FieldType::String,
            required: true,
            unique: false, searchable: true, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "coordinates".to_string(),
            display_name: "Coordinates".to_string(),
            field_type: FieldType::Point { srid: 4326 },     
            required: true,
            spatial_indexed: true,            
            unique: false, searchable: true, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, 
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "elevation_meters".to_string(),
            display_name: "Elevation".to_string(),
            field_type: FieldType::Integer,
            required: false,
            unique: false, searchable: false, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .with_timestamps();

    // 3. Geopolitical Boundaries (Countries, States)
    let boundaries = ModelSchema::new("Boundaries")
        .description("Country and State borders")
        .field(Field {
            name: "region_name".to_string(),
            display_name: "Region Name".to_string(),
            field_type: FieldType::String,
            required: true,
            unique: false, searchable: true, filterable: true, list_display: true,
            read_only: false, encrypted: false, requires_approval: false, spatial_indexed: false,
            default_value: None, help_text: None,
        })
        .field(Field {
            name: "border".to_string(),
            display_name: "Border".to_string(),
            field_type: FieldType::Polygon { srid: 4326 },   
            required: true,
            spatial_indexed: true,            
            unique: false, searchable: false, filterable: true, list_display: false,
            read_only: false, encrypted: false, requires_approval: false, 
            default_value: None, help_text: None,
        })
        .with_timestamps();

    let app = VellaApp::new()
        .register(map_tiles)
        .register(landmarks)
        .register(boundaries);
    
    // Boot the highly-concurrent Spatial Engine
    println!("🚀 Google Earth Backend Live at http://localhost:8080");
    app.run().await?;
    
    Ok(())
}
