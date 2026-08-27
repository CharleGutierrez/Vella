# High-Frequency Trading (HFT) Engineering with Vella

Welcome to the comprehensive guide for Quantitative Finance and High-Frequency Trading (HFT) Engineers using the Vella framework. Vella is designed to provide ultra-low latency infrastructure, hardware acceleration, and robust quantitative tools right out of the box.

This manual covers the core quantitative finance capabilities built into `src/trading/` and how to leverage them for your trading strategies.

## 1. Limit Order Book (LOB) Matching Engine
The core of any exchange or HFT simulation is the Limit Order Book (LOB). Vella's matching engine (`src/trading/matching.rs`) is optimized for ultra-low latency.

It uses a Price-Time Priority FIFO ruleset. For efficiency, order queues are implemented using `VecDeque`, allowing for O(1) insertions at the back and removals at the front, which perfectly models time priority.

### Key Features:
- **Price Levels:** Bids are sorted descending, asks ascending.
- **Cross Matching:** Instantly crosses incoming marketable orders against resting liquidity.

## 2. Quantitative Backtester
Before deploying a strategy, it must be rigorously tested. The Vella Quantitative Backtester (`src/trading/backtest.rs`) allows you to load tick-level data and simulate execution.

### Capabilities:
- **Tick Data Loading:** Efficiently parse CSV or binary tick data.
- **Real-time PnL Computation:** Tracks realized and unrealized Profit and Loss, accounting for simulated execution latency and slippage.

## 3. FIX Protocol Parser
The Financial Information eXchange (FIX) protocol is the standard for trading connectivity. Vella's `src/trading/fix.rs` provides a zero-allocation, high-throughput FIX parser and builder.

### How it Works:
- **Network Message Construction:** Easily build raw FIX messages with proper tag-value pairs (e.g., `8=FIX.4.2|9=...`).
- **Validation:** Implements modulo-256 ASCII checksum validation, essential for data integrity over TCP.

## 4. FPGA Hardware Compiler
For the absolute lowest latency, software is not enough. Vella integrates deeply with hardware via its FPGA compiler (`src/trading/fpga.rs`).

### Hardware Acceleration:
- **Rust to Verilog:** Using `rust_hdl`, Vella can convert your Rust structs and dynamic market logic directly into Verilog logic gates.
- **Zero-Latency Logic:** Offload critical path logic—like static threshold checks or basic volatility calculations—straight to the FPGA to bypass OS and network stack overhead.

## Code Examples

Here are some explicit Rust snippets to get you started immediately.

### HFT Setup Example
From `examples/test_hft.rs`:

```rust
use vella::trading::matching::OrderBook;
use vella::trading::fix::FixMessageBuilder;

fn main() {
    let mut book = OrderBook::new("BTC/USD");
    
    // Add resting liquidity
    book.add_limit_order(vella::trading::Side::Bid, 50000.0, 1.5);
    
    // Build a FIX message for execution
    let fix_msg = FixMessageBuilder::new()
        .msg_type("D")
        .symbol("BTC/USD")
        .side('1') // Buy
        .price(50000.0)
        .build();
        
    println!("Compiled FIX: {}", fix_msg);
}
```

### Trading and Backtesting Example
From `examples/test_trading.rs`:

```rust
use vella::trading::backtest::Backtester;
use vella::trading::fpga::FpgaCompiler;

fn main() {
    let mut backtester = Backtester::new();
    backtester.load_ticks("data/ticks.csv");
    
    let pnl = backtester.run_strategy(|tick| {
        // Simple momentum logic
        tick.price > tick.moving_average
    });
    println!("Final PnL: {}", pnl);
    
    // Compile logic to FPGA
    FpgaCompiler::compile_strategy_to_verilog("MomentumStrat");
}
```

With Vella, you have the full stack—from network parsing and software order books to FPGA hardware offloading—at your fingertips. Happy trading!
