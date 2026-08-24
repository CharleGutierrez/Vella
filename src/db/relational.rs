use tracing::info;
use crate::ai::tuner::AiTuner;
use std::sync::Arc;

pub struct RelationalQueryBuilder {
    ai_tuner: Arc<AiTuner>,
}

impl RelationalQueryBuilder {
    pub fn new(ai_tuner: Arc<AiTuner>) -> Self {
        Self { ai_tuner }
    }

    /// Expands deep relational queries like `?expand=author.company,comments.user`
    pub fn build_expansion_query(&self, base_table: &str, expands: &[&str], measured_latency_ms: u64) -> String {
        info!("Building deep relational JOIN for {} with expands: {:?}", base_table, expands);
        
        let mut joins = String::new();
        for relation in expands {
            let parts: Vec<&str> = relation.split('.').collect();
            let related_table = parts[0];
            
            // Check if AI recommends a new index based on past query latency
            if let Some(recommendation) = self.ai_tuner.analyze_slow_join(base_table, related_table, measured_latency_ms) {
                info!("Executing AI Autonomous Index Generation: {}", recommendation.ddl);
                // In production, execute `recommendation.ddl` asynchronously against the DB.
            }

            joins.push_str(&format!(" LEFT JOIN {} ON {}.{}_id = {}.id", related_table, base_table, related_table, related_table));
            if parts.len() > 1 {
                let deeply_related = parts[1];
                joins.push_str(&format!(" LEFT JOIN {} ON {}.{}_id = {}.id", deeply_related, related_table, deeply_related, deeply_related));
            }
        }
        
        format!("SELECT * FROM {}{}", base_table, joins)
    }
}
