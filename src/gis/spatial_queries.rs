use tracing::info;

pub struct SpatialQueryTranslator;

impl SpatialQueryTranslator {
    /// Translates URL query parameters like `?geom[intersects]=POLYGON((...))` into PostGIS AST
    pub fn parse_spatial_filters(url_query: &str) -> Option<String> {
        if url_query.contains("[intersects]=") {
            let geom_text = url_query.split("=").nth(1).unwrap_or("POLYGON EMPTY");
            info!("AST Translator: Parsed Spatial Intersection Operator");
            Some(format!("ST_Intersects(geom, ST_GeomFromText('{}', 4326))", geom_text))
        } 
        else if url_query.contains("[dwithin]=") {
            let params = url_query.split("=").nth(1).unwrap_or("POINT(0 0),1000");
            info!("AST Translator: Parsed Spatial Distance (DWithin) Operator");
            Some(format!("ST_DWithin(geom::geography, ST_GeomFromText('{}')::geography)", params))
        } else {
            None
        }
    }
}
