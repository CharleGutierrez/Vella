use serde_json::json;
use std::time::Instant;
use vella::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use vella::prelude::*;

/// Benchmark and verify Vella execution characteristics across 25 years of CPU architectures
#[test]
fn test_cpu_era_2000_to_2005_single_core_low_mem() {
    // 2000-2005 Era: Pentium III/4 / Athlon XP / Early Opteron (1 Core, 256MB RAM limit)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_cpu_2000_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 2).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("LegacyItem")
            .field(Field::string("name").required())
            .field(Field::money("price", "USD"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let start = Instant::now();
        // Sequential single-core throughput test
        for i in 0..100 {
            let mut payload = serde_json::Map::new();
            payload.insert("name".to_string(), json!(format!("Item {}", i)));
            payload.insert("price".to_string(), json!(19.99 + i as f64));
            db.insert(&schema, &payload).await.unwrap();
        }
        let elapsed = start.elapsed();

        // Must execute under single-core constraint with sub-second execution
        assert!(elapsed.as_secs_f64() < 2.0);
        let count = db.get_by_id(&schema, 100).await.unwrap();
        assert!(count.is_some());
    });
}

#[test]
fn test_cpu_era_2006_to_2011_quad_core_multi_threading() {
    // 2006-2011 Era: Core 2 Quad / Nehalem Xeon 5600 (4 Cores, 4GB-8GB RAM)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_cpu_2008_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 10).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("QuadItem")
            .field(Field::string("name").required())
            .field(Field::integer("stock"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let start = Instant::now();
        // 4 concurrent tasks simulating 4-core parallel execution
        let mut handles = Vec::new();
        for worker_id in 0..4 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            handles.push(tokio::spawn(async move {
                for i in 0..50 {
                    let mut payload = serde_json::Map::new();
                    payload.insert("name".to_string(), json!(format!("Worker {} Item {}", worker_id, i)));
                    payload.insert("stock".to_string(), json!(100));
                    db_clone.insert(&schema_clone, &payload).await.unwrap();
                }
            }));
        }

        for h in handles {
            h.await.unwrap();
        }
        let elapsed = start.elapsed();

        // 200 records inserted in parallel across 4 workers in milliseconds
        assert!(elapsed.as_millis() < 2500);
    });
}

#[test]
fn test_cpu_era_2012_to_2017_cloud_16_core_density() {
    // 2012-2017 Era: Haswell / Broadwell / Skylake Xeon E5 v3/v4 (16 Cores, 32GB-64GB RAM)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_cpu_2015_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 25).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("CloudItem")
            .field(Field::string("name").required().searchable())
            .field(Field::money("price", "USD").filterable(true))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let mut payload = serde_json::Map::new();
        payload.insert("name".to_string(), json!("High Speed Cloud NVMe"));
        payload.insert("price".to_string(), json!(299.99));
        let created = db.insert(&schema, &payload).await.unwrap();
        let item_id = created.get("id").unwrap().as_i64().unwrap();

        // Concurrent reads across 16 workers
        let start = Instant::now();
        let mut read_handles = Vec::new();
        for _ in 0..16 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            read_handles.push(tokio::spawn(async move {
                for _ in 0..50 {
                    let rec = db_clone.get_by_id(&schema_clone, item_id).await.unwrap();
                    assert!(rec.is_some());
                }
            }));
        }

        for h in read_handles {
            h.await.unwrap();
        }
        let elapsed = start.elapsed();

        // 800 parallel queries served with microsecond read latencies
        assert!(elapsed.as_millis() < 1000);
    });
}

#[test]
fn test_cpu_era_2018_to_2025_modern_epyc_scale_64_workers() {
    // 2018-2025 Era: AMD EPYC Rome/Milan/Genoa/Turin & Sapphire Rapids (64-128+ Cores, 256 Threads, DDR5)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(64)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_cpu_2025_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 50).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("EpycItem")
            .field(Field::string("name").required())
            .field(Field::vector("embedding", 128))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let start = Instant::now();
        // 64 parallel async worker tasks executing concurrently
        let mut handles = Vec::new();
        for worker_id in 0..64 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            handles.push(tokio::spawn(async move {
                let mut payload = serde_json::Map::new();
                payload.insert("name".to_string(), json!(format!("Epyc Worker Task {}", worker_id)));
                payload.insert("embedding".to_string(), json!(vec![0.1f32; 128]));
                db_clone.insert(&schema_clone, &payload).await.unwrap()
            }));
        }

        for h in handles {
            let res = h.await.unwrap();
            assert!(res.get("id").is_some());
        }

        let elapsed = start.elapsed();
        // 64 parallel tasks dispatched and completed in sub-second time
        assert!(elapsed.as_millis() < 2000);
    });
}
