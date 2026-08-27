use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_type: String,
    pub price: f64,
    pub size: u64,
}

pub struct MatchingEngine {
    symbol: String,
    bids: VecDeque<Order>, // sorted high to low price
    asks: VecDeque<Order>, // sorted low to high price
}

impl MatchingEngine {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            bids: VecDeque::new(),
            asks: VecDeque::new(),
        }
    }

    /// Submits a Limit Order to the book
    pub fn submit_order(&mut self, order_type: &str, price: f64, mut size: u64) {
        println!("📈 [Vella LOB - {}] Received {} order for {} shares @ ${}", self.symbol, order_type, size, price);
        
        let is_bid = order_type.to_lowercase() == "bid";
        
        if is_bid {
            while size > 0 && !self.asks.is_empty() {
                let best_ask = &mut self.asks[0];
                if best_ask.price <= price {
                    let trade_size = size.min(best_ask.size);
                    println!("🤝 [Vella LOB] Trade Matched! Cleared {} shares @ ${}", trade_size, best_ask.price);
                    size -= trade_size;
                    best_ask.size -= trade_size;
                    if best_ask.size == 0 {
                        self.asks.pop_front();
                    }
                } else {
                    break;
                }
            }
            if size > 0 {
                self.bids.push_back(Order { order_type: order_type.to_string(), price, size });
                self.bids.make_contiguous().sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        } else {
            while size > 0 && !self.bids.is_empty() {
                let best_bid = &mut self.bids[0];
                if best_bid.price >= price {
                    let trade_size = size.min(best_bid.size);
                    println!("🤝 [Vella LOB] Trade Matched! Cleared {} shares @ ${}", trade_size, best_bid.price);
                    size -= trade_size;
                    best_bid.size -= trade_size;
                    if best_bid.size == 0 {
                        self.bids.pop_front();
                    }
                } else {
                    break;
                }
            }
            if size > 0 {
                self.asks.push_back(Order { order_type: order_type.to_string(), price, size });
                self.asks.make_contiguous().sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }
    }
}
