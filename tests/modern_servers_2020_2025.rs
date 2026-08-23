use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use vella::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use vella::prelude::*;

#[test]
fn test_2020_zen2_ice_lake_64_threads_throughput() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(64)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_server_2020_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 50).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("ServerMetric2020")
            .field(Field::string("node_id").required().searchable())
            .field(Field::float("cpu_utilization").filterable(true))
            .field(Field::integer("iops"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let total_ops = Arc::new(AtomicU64::new(0));
        let start = Instant::now();

        let mut handles = Vec::new();
        for worker_id in 0..64 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            let ops_clone = total_ops.clone();

            handles.push(tokio::spawn(async move {
                for i in 0..25 {
                    let mut payload = serde_json::Map::new();
                    payload.insert("node_id".to_string(), json!(format!("epyc-rome-node-{}", worker_id)));
                    payload.insert("cpu_utilization".to_string(), json!(78.4 + (i as f64 * 0.1)));
                    payload.insert("iops".to_string(), json!(150000 + i));
                    let res = db_clone.insert(&schema_clone, &payload).await.unwrap();
                    assert!(res.get("id").is_some());
                    ops_clone.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        let ops = total_ops.load(Ordering::Relaxed);
        assert_eq!(ops, 1600);

        println!("⚡ [2020 Server Simulation - 64 Threads]: 1,600 transactions in {:?} ({:.2} ops/sec)", elapsed, ops as f64 / elapsed.as_secs_f64());
        assert!(elapsed.as_millis() < 3000);
    });
}

#[test]
fn test_2022_2023_zen4_sapphire_rapids_128_threads_concurrency() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(128)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/vella_server_2023_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 100).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("GenoaProduct")
            .field(Field::string("sku").required().unique().searchable())
            .field(Field::money("price", "USD").filterable(true))
            .field(Field::integer("stock"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let mut seed = serde_json::Map::new();
        seed.insert("sku".to_string(), json!("GENOA-BASE-001"));
        seed.insert("price".to_string(), json!(499.99));
        seed.insert("stock".to_string(), json!(5000));
        let created = db.insert(&schema, &seed).await.unwrap();
        let base_id = created.get("id").unwrap().as_i64().unwrap();

        let start = Instant::now();
        let total_reads = Arc::new(AtomicU64::new(0));

        let mut handles = Vec::new();
        for _ in 0..128 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            let reads_clone = total_reads.clone();

            handles.push(tokio::spawn(async move {
                for _ in 0..50 {
                    let res = db_clone.get_by_id(&schema_clone, base_id).await.unwrap();
                    assert!(res.is_some());
                    reads_clone.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        let reads = total_reads.load(Ordering::Relaxed);
        assert_eq!(reads, 6400);

        println!("⚡ [2022-2023 Genoa 128 Threads]: 6,400 concurrent reads in {:?} ({:.2} reads/sec)", elapsed, reads as f64 / elapsed.as_secs_f64());
        assert!(elapsed.as_millis() < 3000);
    });
}
