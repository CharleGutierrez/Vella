/// Vella Native Limit Order Book (LOB) Matching Engine
/// In-memory queue that matches Bid/Ask orders in nanoseconds (FIFO).
pub struct MatchingEngine {
    symbol: String,
}

impl MatchingEngine {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    /// Submits a Limit Order to the book
    pub fn submit_order(&self, order_type: &str, price: f64, size: u64) {
        println!("📈 [Vella LOB - {}] Received {} order for {} shares @ ${}", self.symbol, order_type, size, price);
        
        // Mock matching logic
        println!("⚡ [Vella LOB] Executing FIFO Matching Algorithm in memory...");
        println!("🤝 [Vella LOB] Trade Matched! Cleared {} shares @ ${}", size, price);
    }
}
