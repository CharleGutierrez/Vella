use crate::ai::middleware::{PromptLogger, SemanticCache, TokenRateLimiter};
use crate::ai::tuner::AiTuner;
use crate::api::build_api_router;
use crate::api::handlers::AppState;
use crate::audit::{ApprovalService, AuditService};
use crate::auth::oauth::OAuthService;
use crate::auth::service::AuthService;
use crate::core::config::VellaConfig;
use crate::core::error::VellaError;
use crate::core::events::EventBus;
use crate::core::hooks::ModelHook;
use crate::core::resilience::{panic_recovery_layer, CircuitBreaker, SystemWatchdog};
use crate::db::{DatabaseType, SchemaMigrator, SqliteDatabase};
use crate::model::{ModelSchema, SchemaRegistry};
use crate::realtime::RealtimeHub;
use crate::types::TypeScriptGenerator;
use crate::ui::{admin_ui_handler, angular_sdk_handler, react_sdk_handler, todo_showcase_handler, vue_sdk_handler, UiConfig};
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Core Application Engine and Server Builder for Vella
pub struct VellaApp {
    pub config: VellaConfig,
    pub schemas: HashMap<String, ModelSchema>,
    pub hooks: Vec<Box<dyn ModelHook>>,
}

/// Backward compatibility / alias
pub type Vella = VellaApp;

impl Default for VellaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl VellaApp {
    pub fn new() -> Self {
        Self {
            config: VellaConfig::default(),
            schemas: HashMap::new(),
            hooks: Vec::new(),
        }
    }

    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.config.site_name = name.into();
        self
    }

    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.config.bind_address = addr.into();
        self
    }

    pub fn database(mut self, url: impl Into<String>) -> Self {
        self.config.database_url = url.into();
        self
    }

    pub fn max_connections(mut self, count: u32) -> Self {
        self.config.max_db_connections = count;
        self
    }

    pub fn auto_export_types_to(mut self, path: impl Into<String>) -> Self {
        self.config.auto_export_types = true;
        self.config.types_export_path = Some(path.into());
        self
    }

    pub fn semantic_cache(mut self, enabled: bool, threshold: f32) -> Self {
        self.config.enable_semantic_cache = enabled;
        self.config.semantic_cache_threshold = threshold;
        self
    }

    pub fn token_rate_limit(mut self, limit_per_minute: u64) -> Self {
        self.config.token_rate_limit_per_minute = limit_per_minute;
        self
    }

    pub fn register(mut self, schema: ModelSchema) -> Self {
        let key = schema.name.to_lowercase();
        self.schemas.insert(key, schema);
        self
    }

    pub fn hook<H: ModelHook + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    /// Construct internal router and state (useful for integration testing)
    pub async fn build_router(self) -> Result<(Router, AppState), VellaError> {
        let db = SqliteDatabase::connect(&self.config.database_url, self.config.max_db_connections).await?;

        // 1. Auto-Migrate System and Domain Models
        SchemaMigrator::migrate_system_tables(&db.pool).await?;
        for schema in self.schemas.values() {
            SchemaMigrator::migrate_model(&db.pool, schema).await?;
        }

        // 2. Initialize Core Services
        let auth_service = Arc::new(AuthService::new(db.pool.clone()));
        auth_service.ensure_admin_user().await?;

        let oauth_service = Arc::new(OAuthService::new(db.pool.clone()));
        let audit_service = Arc::new(AuditService::new(db.pool.clone()));
        let approval_service = Arc::new(ApprovalService::new(db.pool.clone()));
        let event_bus = Arc::new(EventBus::default());
        let realtime_hub = Arc::new(RealtimeHub::default());
        realtime_hub.start_event_bridge(&event_bus, self.config.redis_url.clone());

        let registry = SchemaRegistry::from_map(self.schemas);

        // 3. Optional Auto-Export TypeScript definitions
        if self.config.auto_export_types {
            if let Some(ref path) = self.config.types_export_path {
                let _ = TypeScriptGenerator::export_to_file(path, &registry);
            }
        }

        // 4. Resilience & AI Engine
        let watchdog = Arc::new(SystemWatchdog::default());
        watchdog.start(db.pool.clone());
        let circuit_breaker = Arc::new(CircuitBreaker::new("vella_global_breaker", 5, 10));
        let ai_tuner = Arc::new(AiTuner::default());
        let token_limiter = Arc::new(TokenRateLimiter::new(self.config.token_rate_limit_per_minute));
        let prompt_logger = Arc::new(PromptLogger::default());
        let semantic_cache = Arc::new(SemanticCache::new(self.config.semantic_cache_threshold));

        let app_state = AppState {
            pool: db.pool.clone(),
            db,
            config: Arc::new(self.config.clone()),
            registry,
            auth_service,
            oauth_service,
            audit_service,
            approval_service,
            event_bus,
            realtime_hub,
            hooks: Arc::new(self.hooks),
            watchdog,
            circuit_breaker,
            ai_tuner,
            token_limiter,
            prompt_logger,
            semantic_cache,
        };

        let ui_config = Arc::new(UiConfig {
            site_name: self.config.site_name.clone(),
            base_url: format!("http://{}", self.config.bind_address),
        });

        // 5. Construct Sub-Routers
        let api_router = build_api_router(app_state.clone());

        let ui_router = Router::new()
            .route("/", get(admin_ui_handler))
            .route("/admin", get(admin_ui_handler))
            .route("/todos", get(todo_showcase_handler))
            .route("/showcase", get(todo_showcase_handler))
            .route("/api/sdk/react.ts", get(react_sdk_handler))
            .route("/api/sdk/vue.ts", get(vue_sdk_handler))
            .route("/api/sdk/angular.ts", get(angular_sdk_handler))
            .with_state(ui_config);

        let app = Router::new()
            .merge(api_router)
            .merge(ui_router)
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .layer(panic_recovery_layer());

        Ok((app, app_state))
    }

    /// Build and run the server
    pub async fn run(self) -> Result<(), VellaError> {
        // Initialize OpenTelemetry Tracing Pipeline if configured
        if let Some(ref otlp_endpoint) = self.config.otlp_endpoint {
            use opentelemetry_otlp::WithExportConfig;
            use tracing_subscriber::layer::SubscriberExt;
            use tracing_subscriber::util::SubscriberInitExt;

            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_endpoint(otlp_endpoint))
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .map_err(|e| VellaError::Internal(format!("Failed to initialize OTLP tracer: {}", e)))?;

            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            
            let filter = tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vella=info,tower_http=info".into());

            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer())
                .with(telemetry)
                .init();
                
            println!("🌐 [Vella] OpenTelemetry (OTLP) Tracing initialized targeting: {}", otlp_endpoint);
        } else {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "vella=info,tower_http=info".into()),
                )
                .try_init();
        }

        let bind_addr_str = self.config.bind_address.clone();
        let db_url_str = self.config.database_url.clone();
        let db_type = DatabaseType::from_url(&db_url_str);

        let (app, _) = self.build_router().await?;

        let addr: SocketAddr = bind_addr_str.parse().map_err(|e: std::net::AddrParseError| {
            VellaError::Internal(format!("Invalid bind address: {}", e))
        })?;

        println!(
            r#"
 __      __   _ _       
 \ \    / /  | | |      
  \ \  / /___| | | __ _ 
   \ \/ // _ \ | |/ _` |
    \  /|  __/ | | (_| |
     \/  \___|_|_|\__,_|  v0.1.0

 ⚡ Next-Gen LLM-Native Backend & Headless CMS (PocketBase ➔ Supabase Scale)
  Server Listening:     http://{}
  Headless CMS SPA:     http://{}
  Database Driver:      {}
  Vector Engine:        {}
  AI Tuner & Scaffolder: Online (RAG, Semantic Cache & DDL Advisor)
  Realtime Transport:   WebSocket (ws://{}/api/realtime/ws) & SSE
  TypeScript Sync:      http://{}/api/types/typescript.d.ts
  OpenAPI & Swagger:    http://{}/swagger
  Default Superadmin:   admin / admin
"#,
            addr, addr, db_type.name(), db_type.vector_engine_name(), addr, addr, addr
        );

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
