# 🌍 GIS & Spatial Data

Vella has first-class native support for Geographic Information Systems (GIS), allowing you to build location-aware applications, routing engines, and logistics dashboards with zero configuration.

## Spatial Field Types
Vella's ORM supports mapping spatial data directly into your schema definitions.

* **`FieldType::Point`**: Represents a specific GPS coordinate (Longitude, Latitude).
* **`FieldType::Polygon`**: Represents an enclosed boundary (e.g., a geofenced delivery zone).
* **`FieldType::Geometry`**: A generic collection of spatial structures.

## Automatic PostGIS Migrations
When you compile your Vella application with the Postgres dialect enabled, Vella's Migration Engine intercepts your Spatial fields and automatically:
1. Injects `CREATE EXTENSION IF NOT EXISTS postgis;` into your database.
2. Generates the correct `GEOMETRY(Point, 4326)` column types.
3. Attaches **GiST (Generalized Search Tree) Indexes** to the columns to guarantee lightning-fast geographic lookups.

## Example Usage

When building your schema, simply toggle the `spatial_indexed` flag:

```rust
schema.fields.push(Field {
    name: "delivery_location".to_string(),
    display_name: "Delivery Location".to_string(),
    field_type: FieldType::Point,
    spatial_indexed: true, // <-- Automatically builds a GiST index
    // ...
});
```

Because Vella automatically generates frontend SDKs, a `Point` field will automatically compile down into a strict TypeScript tuple `[number, number]` for your React/Vue frontend!
