use vella::trading::forex::{CurrencyPair, calculate_spread, calculate_margin};

fn main() {
    println!("Testing Forex & Stock Trading mechanics...");

    // 1. Currency Pair Management
    let pair = CurrencyPair::new("EUR", "USD");
    println!("Currency Pair: {}", pair.symbol());

    // 2. Pip & Spread Calculation
    let bid = 1.1050;
    let ask = 1.1055;
    let pip_value = 0.0001; // Standard for EUR/USD
    let spread = calculate_spread(ask, bid, pip_value);
    println!("Bid: {}, Ask: {}, Spread (pips): {:.1}", bid, ask, spread);

    // 3. Leverage & Margin Evaluation
    let position_size = 100_000.0; // 1 standard lot
    let leverage = 50.0; // 50:1 leverage
    let margin = calculate_margin(position_size, leverage);
    println!("Position Size: ${}, Leverage: {}:1, Required Margin: ${:.2}", position_size, leverage, margin);
    
    println!("Forex functionality successfully executed.");
}
