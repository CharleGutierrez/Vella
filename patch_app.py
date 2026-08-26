import re

with open('src/app.rs', 'r', encoding='utf-8') as f:
    code = f.read()

injection = """
        // --- INJECT TRUE SERVERLESS DYNAMIC SCRIPTING ---
        let mut all_hooks = self.hooks;
        let script_engine = std::sync::Arc::new(crate::scripting::engine::ScriptEngine::new());
        let dynamic_hook = Box::new(crate::scripting::hook::DynamicScriptHook::new(script_engine));
        all_hooks.push(dynamic_hook);

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
            hooks: Arc::new(all_hooks),
"""

code = code.replace(
"""        let app_state = AppState {
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
            hooks: Arc::new(self.hooks),""", injection)

with open('src/app.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print("Injected DynamicScriptHook into AppState")
