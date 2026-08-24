use rhai::{Engine, Scope, AST, Dynamic};
use tracing::{info, warn};

pub struct ScriptEngine {
    engine: Engine,
}

impl ScriptEngine {
    pub fn new() -> Self {
        info!("Initializing Embedded Rhai Scripting Engine...");
        let mut engine = Engine::new();
        
        // Register native Rust functions that can be called from user scripts
        engine.register_fn("log_info", |msg: String| info!("Script Log: {}", msg));
        engine.register_fn("log_warn", |msg: String| warn!("Script Warn: {}", msg));
        
        Self { engine }
    }

    pub fn compile(&self, script: &str) -> Result<AST, Box<dyn std::error::Error + Send + Sync>> {
        let ast = self.engine.compile(script).map_err(|e| e.to_string())?;
        Ok(ast)
    }

    pub fn execute(&self, ast: &AST, context_data: &str) -> Result<Dynamic, Box<dyn std::error::Error + Send + Sync>> {
        let mut scope = Scope::new();
        scope.push("ctx", context_data.to_string());
        
        let result: Dynamic = self.engine.eval_ast_with_scope(&mut scope, ast).map_err(|e| e.to_string())?;
        Ok(result)
    }
}
