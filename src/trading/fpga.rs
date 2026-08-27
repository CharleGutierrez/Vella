use rust_hdl::prelude::*;
use std::fs;

#[derive(LogicBlock)]
pub struct TradeSignal {
    pub price: Signal<In, Bits<32>>,
    pub threshold: Signal<In, Bits<32>>,
    pub volatility: Signal<In, Bits<32>>,
    pub buy_signal: Signal<Out, Bit>,
    pub sell_signal: Signal<Out, Bit>,
}

impl Default for TradeSignal {
    fn default() -> Self {
        Self {
            price: Default::default(),
            threshold: Default::default(),
            volatility: Default::default(),
            buy_signal: Default::default(),
            sell_signal: Default::default(),
        }
    }
}

impl Logic for TradeSignal {
    #[hdl_gen]
    fn update(&mut self) {
        // Buy if price is significantly below threshold based on volatility
        if self.price.val() + self.volatility.val() < self.threshold.val() {
            self.buy_signal.next = true;
            self.sell_signal.next = false;
        } 
        // Sell if price is significantly above threshold based on volatility
        else if self.price.val() > self.threshold.val() + self.volatility.val() {
            self.buy_signal.next = false;
            self.sell_signal.next = true;
        } 
        else {
            self.buy_signal.next = false;
            self.sell_signal.next = false;
        }
    }
}

/// Vella Hardware Description Language (HDL) Compiler
/// Translates Rust trading strategies into raw Verilog for FPGA flashing.
pub struct FpgaCompiler;

impl FpgaCompiler {
    /// Compiles high-level trading logic into Verilog for physical silicon execution
    pub fn compile_to_verilog(strategy_name: &str) -> Result<String, String> {
        println!("🖲️ [Vella FPGA] Analyzing Rust trading strategy '{}'...", strategy_name);
        println!("🛠️ [Vella FPGA] Transpiling logic gates to Verilog HDL...");
        
        let mut uut = TradeSignal::default();
        uut.connect_all();
        let verilog = generate_verilog(&uut);
        
        if let Err(e) = fs::write("strategy.v", &verilog) {
            return Err(format!("Failed to write Verilog file: {}", e));
        }
        
        println!("🔥 [Vella FPGA] Verilog compilation successful. Saved to strategy.v. Ready for Zero-Latency Hardware Flashing!");
        Ok(verilog)
    }
}
