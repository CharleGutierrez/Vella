use crate::ai::gateway::AiTool;
use serde_json::Value;
use std::process::Command;
use tracing::info;

pub struct AgentSkills;

impl AgentSkills {
    /// Returns the JSON schemas for the agent tools
    pub fn get_available_tools() -> Vec<AiTool> {
        vec![
            AiTool {
                name: "execute_bash".to_string(),
                description: "Execute a bash command on the host system. Useful for DevOps, System Management, and Git operations.".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "The bash command to run" }
                    },
                    "required": ["command"]
                })
            },
            AiTool {
                name: "fetch_webpage".to_string(),
                description: "Fetch the HTML or text content of a URL. Useful for scraping competitors or reading documentation.".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "The full URL to fetch (e.g. https://example.com)" }
                    },
                    "required": ["url"]
                })
            }
        ]
    }

    /// Executes the requested tool natively in Rust
    pub async fn execute(name: &str, args: &Value) -> Result<String, String> {
        match name {
            "execute_bash" => {
                let cmd = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
                info!("⚙️ [Agent Skill] Executing Bash: {}", cmd);
                
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .map_err(|e| format!("Failed to spawn shell: {}", e))?;
                    
                let mut result = String::from_utf8_lossy(&output.stdout).to_string();
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                if !err.is_empty() {
                    result.push_str(&format!("\nSTDERR: {}", err));
                }
                
                // Truncate to avoid blowing up the LLM context limit
                if result.len() > 15000 {
                    result.truncate(15000);
                    result.push_str("\n...[TRUNCATED]");
                }
                
                Ok(result)
            },
            "fetch_webpage" => {
                let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
                info!("⚙️ [Agent Skill] Fetching URL: {}", url);
                
                let client = reqwest::Client::builder()
                    .user_agent("Vella-Autonomous-Agent/1.0")
                    .build()
                    .map_err(|e| e.to_string())?;
                    
                let res = client.get(url).send().await.map_err(|e| format!("Failed to fetch URL: {}", e))?;
                let text = res.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;
                
                let mut truncated = text;
                if truncated.len() > 15000 {
                    truncated.truncate(15000);
                    truncated.push_str("\n...[TRUNCATED]");
                }
                
                Ok(truncated)
            },
            _ => Err(format!("Unknown skill requested: {}", name))
        }
    }
}
