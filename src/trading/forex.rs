#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyPair {
    pub base_currency: String,
    pub quote_currency: String,
}

impl CurrencyPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base_currency: base.to_string(),
            quote_currency: quote.to_string(),
        }
    }
    
    pub fn symbol(&self) -> String {
        format!("{}/{}", self.base_currency, self.quote_currency)
    }
}

/// Calculate the pip spread between ask and bid.
/// `pip_value` indicates the decimal value of one pip (e.g., 0.0001 for EUR/USD, 0.01 for USD/JPY).
pub fn calculate_spread(ask: f64, bid: f64, pip_value: f64) -> f64 {
    if ask < bid {
        return 0.0;
    }
    (ask - bid) / pip_value
}

/// Calculate required margin based on leverage.
/// E.g., for a $100,000 position with 50:1 leverage, the margin is 100,000 / 50 = $2,000.
pub fn calculate_margin(position_size: f64, leverage: f64) -> f64 {
    if leverage <= 0.0 {
        return position_size;
    }
    position_size / leverage
}
