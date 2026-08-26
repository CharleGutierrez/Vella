use crate::core::hooks::ModelHook;
use crate::core::error::VellaError;
use crate::scripting::engine::ScriptEngine;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn};

pub struct DynamicScriptHook {
    engine: Arc<ScriptEngine>,
}

impl DynamicScriptHook {
    pub fn new(engine: Arc<ScriptEngine>) -> Self {
        Self { engine }
    }

    fn execute_script(&self, event: &str, model: &str, data: &mut Value) -> Result<(), VellaError> {
        let script_path = format!("scripts/{}_{}.rhai", model, event);
        if !Path::new(&script_path).exists() {
            return Ok(()); // No script for this hook
        }

        info!("? Executing dynamic script hook: {}", script_path);
        
        let script = std::fs::read_to_string(&script_path)
            .map_err(|e| VellaError::Internal(format!("Failed to read script: {}", e)))?;

        // Convert the JSON Value (data) into a Rhai Dynamic
        let mut rhai_map = rhai::serde::to_dynamic(data.clone())
            .map_err(|e| VellaError::Internal(format!("Failed to serialize to Rhai: {}", e)))?;

        // Evaluate the script, passing the payload as ctx
        let ast = self.engine.compile(&script)
            .map_err(|e| VellaError::Internal(format!("Script compile error: {}", e)))?;

        // execute_with_ctx will allow the script to modify hai_map
        self.engine.execute_mut(&ast, &mut rhai_map)
            .map_err(|e| VellaError::Validation(format!("Script rejected operation: {}", e)))?;

        // Convert back to JSON Value
        let new_data: Value = rhai::serde::from_dynamic(&rhai_map)
            .map_err(|e| VellaError::Internal(format!("Failed to deserialize from Rhai: {}", e)))?;

        *data = new_data;
        Ok(())
    }
}

#[async_trait]
impl ModelHook for DynamicScriptHook {
    async fn before_create(&self, model: &str, data: &mut Value) -> Result<(), VellaError> {
        self.execute_script("before_create", model, data)
    }

    async fn before_update(&self, model: &str, _id: i64, data: &mut Value) -> Result<(), VellaError> {
        self.execute_script("before_update", model, data)
    }
}
