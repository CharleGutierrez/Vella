use vella::trading::matching::MatchingEngine;
use vella::trading::backtest::BacktestSandbox;

fn main() {
    println!("=== Testing Limit Order Book ===");
    let mut lob = MatchingEngine::new("BTC/USD");
    
    // Add resting asks
    lob.submit_order("Ask", 50100.0, 5);
    lob.submit_order("Ask", 50200.0, 10);
    
    // Add resting bids
    lob.submit_order("Bid", 49900.0, 8);
    lob.submit_order("Bid", 49800.0, 12);
    
    // Submit market-crossing orders
    lob.submit_order("Bid", 50150.0, 7); // Should match 5 with 50100, rest as bid at 50150
    lob.submit_order("Ask", 49900.0, 10); // Should match 8 with 49900, rest as ask at 49900
    
    println!("\n=== Testing Backtester ===");
    let sandbox = BacktestSandbox::new("s3://vella-datasets/tick_data_2023.csv");
    match sandbox.run_simulation("Mean Reversion V1") {
        Ok(report) => println!("{}", report),
        Err(e) => println!("Error: {}", e),
    }
}
