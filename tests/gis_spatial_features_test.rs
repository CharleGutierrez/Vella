use vella::gis::{VectorTileServer, WmsRenderer, SpatialQueryTranslator, Cesium3DTileset};
use vella::api::ogc::WfsService;

#[test]
fn test_mvt_generation() {
    let mvt = VectorTileServer::new("buildings");
    let sql = mvt.generate_mvt_query(14, 4823, 6192);
    
    assert!(sql.contains("ST_AsMVTGeom"), "Missing PostGIS MVT geometry transform");
    assert!(sql.contains("ST_TileEnvelope(14, 4823, 6192)"), "Tile XYZ not mapped to envelope");
}

#[test]
fn test_wms_raster_rendering() {
    let wms = WmsRenderer::new();
    let png_bytes = wms.render_geotiff_to_png("-73.9,40.7,-73.8,40.8", 1024, 1024, "terrain");
    
    assert!(png_bytes.starts_with(b"PNG"), "WMS Engine failed to output valid PNG raster bytes");
}

#[test]
fn test_spatial_query_translation() {
    let url_query = "geom[intersects]=POLYGON((0 0, 10 0, 10 10, 0 10, 0 0))";
    let ast = SpatialQueryTranslator::parse_spatial_filters(url_query).unwrap();
    
    assert!(ast.contains("ST_Intersects"), "Failed to translate [intersects] to PostGIS operator");
    assert!(ast.contains("POLYGON"), "Lost WKT geometry in translation");
}

#[test]
fn test_3d_point_cloud_streaming() {
    let tileset = Cesium3DTileset::new("nyc_lidar");
    let lod_payload = tileset.fetch_lod_node(16.0);
    
    assert!(lod_payload.contains(r#""magic": "pnts""#), "Missing 3D Tile Point Cloud signature");
    assert!(lod_payload.contains("nyc_lidar"), "Lost dataset context");
}

#[test]
fn test_ogc_wfs_capabilities() {
    let wfs = WfsService;
    let xml = wfs.get_capabilities();
    
    assert!(xml.contains("<wfs:WFS_Capabilities"), "Missing OGC standard XML root");
    assert!(xml.contains("EPSG::4326"), "Missing WGS84 CRS definition");
}
