use tracing::info;

pub struct VectorTileServer {
    layer_name: String,
}

impl VectorTileServer {
    pub fn new(layer_name: &str) -> Self {
        info!("Initializing Mapbox Vector Tile (MVT) Server for Layer: {}", layer_name);
        Self {
            layer_name: layer_name.to_string(),
        }
    }

    /// Dynamically generates a Protocol Buffer (PBF) vector tile using PostGIS ST_AsMVT
    pub fn generate_mvt_query(&self, z: u8, x: u32, y: u32) -> String {
        info!("Generating Vector Tile for {} at Z:{} X:{} Y:{}", self.layer_name, z, x, y);
        
        // Translates XYZ to Web Mercator Bounding Box natively inside Postgres
        format!(
            "WITH mvtgeom AS (
                SELECT ST_AsMVTGeom(geom, ST_TileEnvelope({z}, {x}, {y})) AS geom, *
                FROM {layer}
                WHERE ST_Intersects(geom, ST_TileEnvelope({z}, {x}, {y}))
            )
            SELECT ST_AsMVT(mvtgeom.*, '{layer}') FROM mvtgeom;",
            z = z, x = x, y = y, layer = self.layer_name
        )
    }
}
