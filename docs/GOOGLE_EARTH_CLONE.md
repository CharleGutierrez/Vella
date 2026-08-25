# 🌍 Building a Google Earth Clone with Vella

Vella's native PostGIS integration makes it the absolute perfect backend for building high-performance 3D mapping and GIS applications.

This guide explains the architecture of the `examples/google_earth_clone.rs` file and how to consume it on the frontend using **CesiumJS** (the industry standard open-source 3D globe library).

---

## 1. The Backend Architecture (Rust + PostGIS)

In `examples/google_earth_clone.rs`, we define three core tables:
1. **`MapTiles`**: Stores XYZ coordinate references to your raw satellite imagery (stored in S3/Cloudflare R2).
2. **`Landmarks`**: Uses Vella's `FieldType::Point` to store latitude/longitude pairs of famous landmarks. Vella automatically builds a **GiST Index** on this column in PostgreSQL so that spatial queries (e.g., *"Find all landmarks within the user's current screen view"*) execute in milliseconds.
3. **`Boundaries`**: Uses Vella's `FieldType::Polygon` to store massive GeoJSON boundaries of countries.

You can run the backend server with:
```bash
cargo run --release --example google_earth_clone
```

---

## 2. Running the Local Demo (`earth.html`)

We have included a pre-built frontend demo file called `earth.html` in the root of the repository. This file uses pure HTML/JS to render the 3D Cesium globe and connect to the Vella engine.

### Bypassing CORS (Crucial Step)
If you double-click `earth.html` and open it via a `file:///` URL, your web browser's security policies (CORS) will silently block the ArcGIS satellite imagery from downloading. The globe will appear as a red dot floating in a dark blue void.

To fix this, you must serve the HTML file over a real HTTP server:

```bash
# Start a simple Python web server in the root of the repo
python -m http.server 8081
```

Now, open your browser and navigate to:
`http://localhost:8081/earth.html`

You will instantly see the fully textured planet Earth spinning in space, powered by Vella's GIS spatial coordinates!

---

## 3. The React Frontend (For Production)

To render a 3D globe in a modern application, we recommend using [Resium](https://resium.reearth.io/) (React components for CesiumJS).

### Installation
```bash
npm install cesium resium
```

### The 3D Globe Component
This component mounts a 3D Earth, fetches `Landmark` Points from Vella's REST API, and drops 3D pins directly onto the globe.

```tsx
import React, { useEffect, useState } from 'react';
import { Viewer, Entity, PointGraphics, EntityDescription } from 'resium';
import { Cartesian3, Color } from 'cesium';

export default function VellaEarth() {
    const [landmarks, setLandmarks] = useState([]);

    // 1. Fetch Spatial Data from Vella
    useEffect(() => {
        // In a real app, you would pass a bounding box query here to only fetch 
        // points currently visible on the camera screen.
        fetch('http://localhost:8080/api/collections/landmarks/records')
            .then(res => res.json())
            .then(data => setLandmarks(data.items));
    }, []);

    return (
        // Mounts the 3D Globe
        <Viewer full>
            {landmarks.map(landmark => {
                // Vella automatically serializes Point types into [Longitude, Latitude]
                const [lng, lat] = landmark.coordinates; 
                
                return (
                    <Entity
                        key={landmark.id}
                        name={landmark.name}
                        position={Cartesian3.fromDegrees(lng, lat, landmark.elevation_meters || 0)}
                    >
                        <PointGraphics pixelSize={10} color={Color.RED} />
                        <EntityDescription>
                            <h1>{landmark.name}</h1>
                            <p>Elevation: {landmark.elevation_meters}m</p>
                        </EntityDescription>
                    </Entity>
                );
            })}
        </Viewer>
    );
}
```

---

## 3. Production Deployment (High-Scale GIS)

Rendering 3D globes generates a massive amount of network traffic. Here is how you deploy Vella to handle Google Earth-level scale:

### 1. Database Tuning
GIS queries are CPU-intensive. You must deploy Vella alongside a highly tuned PostgreSQL instance.
*   Increase `shared_buffers` in PostgreSQL to hold GiST indexes entirely in memory.
*   Use Vella's `DATABASE_MAX_CONNECTIONS=500` env var to allow heavy concurrency.

### 2. Edge Caching (Cloudflare)
Do not serve MapTiles directly from Vella's API. 
Instead, configure Cloudflare to cache `GET /api/collections/map_tiles/records?zoom=...` requests at the Edge so the Vella Rust binary is only hit when new tiles are uploaded.

### 3. Vella Load Balancing
Because Vella is a stateless Rust binary, you can deploy it infinitely horizontally.

**docker-compose.yml**
```yaml
version: '3'
services:
  lb:
    image: nginx:latest
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
  vella_node_1:
    image: your-vella-image
    environment:
      - DATABASE_URL=postgres://gis_user:pass@db:5432/vella
  vella_node_2:
    image: your-vella-image
    environment:
      - DATABASE_URL=postgres://gis_user:pass@db:5432/vella
  vella_node_3:
    image: your-vella-image
    environment:
      - DATABASE_URL=postgres://gis_user:pass@db:5432/vella
```
NGINX will round-robin the massive influx of spatial coordinate queries across the three lightweight Rust binaries.
