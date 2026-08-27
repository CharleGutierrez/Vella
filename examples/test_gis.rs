use vella::environment::geospatial::{GeospatialAnalyzer, Point, BoundingBox};

fn main() {
    println!("Starting Vella GIS Test...");

    let analyzer = GeospatialAnalyzer::new(Some("postgres://postgis:5432/gis_db".to_string()));

    // Example points: San Francisco, CA and Los Angeles, CA
    let san_francisco = Point { latitude: 37.7749, longitude: -122.4194 };
    let los_angeles = Point { latitude: 34.0522, longitude: -118.2437 };

    // Calculate distance
    let distance = analyzer.haversine_distance(san_francisco, los_angeles);
    println!("Distance between San Francisco and Los Angeles: {:.2} km", distance);

    // Bounding Box Test (Rough bounds of California)
    let california_bbox = BoundingBox {
        min: Point { latitude: 32.5343, longitude: -124.4096 },
        max: Point { latitude: 42.0095, longitude: -114.1312 },
    };

    let is_sf_in_ca = analyzer.is_within_bounding_box(san_francisco, california_bbox);
    println!("Is San Francisco in California bounding box? {}", is_sf_in_ca);

    // Another point outside California (e.g., New York City)
    let nyc = Point { latitude: 40.7128, longitude: -74.0060 };
    let is_nyc_in_ca = analyzer.is_within_bounding_box(nyc, california_bbox);
    println!("Is NYC in California bounding box? {}", is_nyc_in_ca);

    println!("GIS Test completed successfully.");
}
