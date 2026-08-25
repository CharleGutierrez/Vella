/// Vella Dynamic AI Pricing Engine
/// Algorithmic yield management for e-commerce and ride-sharing.
pub struct PricingEngine {
    base_margin_percent: f64,
}

impl PricingEngine {
    pub fn new(margin: f64) -> Self {
        Self { base_margin_percent: margin }
    }

    /// Calculates real-time surge pricing based on market demand and inventory scarcity
    pub fn calculate_surge_price(&self, product_base_price: f64, active_buyers: u32, inventory_left: u32) -> f64 {
        println!("💹 [Vella Commerce] Analyzing market demand... (Buyers: {}, Inventory: {})", active_buyers, inventory_left);
        
        let mut final_price = product_base_price * (1.0 + self.base_margin_percent);

        if active_buyers > inventory_left * 5 {
            println!("📈 [Vella Commerce] HYPER-DEMAND DETECTED. Applying 200% Surge Multiplier.");
            final_price *= 2.0;
        } else if inventory_left > active_buyers * 10 {
            println!("📉 [Vella Commerce] Overstocked. Applying 15% liquidation discount.");
            final_price *= 0.85;
        }

        println!("💰 [Vella Commerce] Final Dynamic Price Locked: ${:.2}", final_price);
        final_price
    }
}
