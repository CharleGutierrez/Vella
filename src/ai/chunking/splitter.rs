use text_splitter::{TextSplitter, Characters};
use tracing::info;
use crate::ai::tuner::AiTuner;

pub struct DocumentSplitter {
    ai_tuner: AiTuner,
}

impl DocumentSplitter {
    pub fn new() -> Self {
        info!("Initializing Semantic Document Splitter with AI Tuner");
        Self { ai_tuner: AiTuner::new() }
    }

    pub fn chunk_text_semantically(&self, text: &str) -> Vec<String> {
        // AI determines optimal token bounds based on content density
        let optimal_size = self.ai_tuner.determine_optimal_chunk_size(text);
        
        info!("Splitting document with dynamically tuned size: {}", optimal_size);
        let splitter = TextSplitter::new(optimal_size);
        
        splitter.chunks(text)
            .map(|c| c.to_string())
            .collect()
    }
}
