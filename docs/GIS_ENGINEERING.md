# Vella Framework: GIS Engineering Manual

Welcome to the comprehensive Geographic Information Systems (GIS) manual for the Vella framework. This guide is designed specifically for GIS Engineers to leverage Vella's robust spatial capabilities, primarily found in `src/environment/geospatial.rs`.

## 1. Spatial Data Structures

Vella provides precise and lightweight data structures to represent geographic entities on Earth.

### `Point`
The `Point` struct represents a specific location using Longitude and Latitude coordinates.

```rust
use vella::environment::geospatial::Point;

let location = Point {
    lat: 37.7749, // Latitude
    lon: -122.4194 // Longitude
};
```

### `BoundingBox`
The `BoundingBox` struct defines a geographic perimeter (a spatial rectangle), making it easy to define boundaries such as states, countries, or delivery zones.

```rust
use vella::environment::geospatial::BoundingBox;

let sf_zone = BoundingBox {
    min_lat: 37.70,
    max_lat: 37.81,
    min_lon: -122.52,
    max_lon: -122.35,
};
```

## 2. Distance Mathematics (Haversine Formula)

For location proximity logic, calculating the great-circle distance between two points on the spherical Earth is essential. Vella uses the highly accurate Haversine formula for this.

### Using `haversine_distance`

The `haversine_distance` function computes the distance (typically in kilometers or meters) between two `Point` instances across a spherical surface.

```rust
use vella::environment::geospatial::{Point, haversine_distance};

let new_york = Point { lat: 40.7128, lon: -74.0060 };
let los_angeles = Point { lat: 34.0522, lon: -118.2437 };

let distance_km = haversine_distance(&new_york, &los_angeles);
println!("Distance: {:.2} km", distance_km);
```

## 3. Spatial Bounding Checks

Executing polygon-style region checks is a core GIS operation. Vella simplifies this with the `is_within_bounding_box` function, perfect for geofencing or checking if a user coordinate sits inside a boundary.

### Using `is_within_bounding_box`

```rust
use vella::environment::geospatial::{Point, BoundingBox, is_within_bounding_box};

let user_location = Point { lat: 37.75, lon: -122.40 };
let delivery_zone = BoundingBox {
    min_lat: 37.70, max_lat: 37.81,
    min_lon: -122.52, max_lon: -122.35,
};

if is_within_bounding_box(&user_location, &delivery_zone) {
    println!("User is within the delivery zone!");
} else {
    println!("User is outside the delivery zone.");
}
```

## 4. Complete Code Example: Integrating into Vella APIs

Here is an explicit example (similar to `examples/test_gis.rs`) demonstrating how you might use these tools to build a proximity-based location tracking feature into your Vella REST APIs.

```rust
// examples/test_gis.rs
use vella::environment::geospatial::{Point, BoundingBox, haversine_distance, is_within_bounding_box};

fn main() {
    // Define a service area (e.g., a city boundary)
    let city_bounds = BoundingBox {
        min_lat: 34.00,
        max_lat: 34.10,
        min_lon: -118.30,
        max_lon: -118.15,
    };

    // Incoming API Request payload with user coordinates
    let user_coord = Point { lat: 34.05, lon: -118.20 };
    let store_coord = Point { lat: 34.04, lon: -118.25 };

    // 1. Spatial Bounding Check (Geofence)
    let is_in_city = is_within_bounding_box(&user_coord, &city_bounds);
    println!("User within city limits: {}", is_in_city);

    // 2. Distance Mathematics
    if is_in_city {
        let dist = haversine_distance(&user_coord, &store_coord);
        println!("User is {:.2} km away from the store.", dist);
        
        if dist < 5.0 {
            println!("User is within 5km radius! Proceed with delivery order.");
        } else {
            println!("User is too far for immediate delivery.");
        }
    }
}
```
