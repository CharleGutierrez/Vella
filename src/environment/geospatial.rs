/// Vella Geospatial AI Pipeline
/// Basic GIS and Geospatial Calculations

pub struct GeospatialAnalyzer {
    pub postgis_connection: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl GeospatialAnalyzer {
    pub fn new(conn_string: Option<String>) -> Self {
        Self {
            postgis_connection: conn_string,
        }
    }

    /// Calculate the Haversine distance between two points in kilometers
    pub fn haversine_distance(&self, p1: Point, p2: Point) -> f64 {
        let earth_radius_km = 6371.0;

        let d_lat = (p2.latitude - p1.latitude).to_radians();
        let d_lon = (p2.longitude - p1.longitude).to_radians();

        let lat1 = p1.latitude.to_radians();
        let lat2 = p2.latitude.to_radians();

        let a = (d_lat / 2.0).sin().powi(2) +
                (d_lon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        earth_radius_km * c
    }

    /// Check if a given point is within a bounding box
    pub fn is_within_bounding_box(&self, point: Point, bbox: BoundingBox) -> bool {
        point.latitude >= bbox.min.latitude &&
        point.latitude <= bbox.max.latitude &&
        point.longitude >= bbox.min.longitude &&
        point.longitude <= bbox.max.longitude
    }
}
