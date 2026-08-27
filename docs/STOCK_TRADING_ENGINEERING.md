# Vella Stock Trading & TradFi Engineering Manual

Welcome to the comprehensive guide for Stock Trading and Traditional Finance (TradFi) engineers using the Vella framework. This manual is designed to equip you with the knowledge to build zero-latency, high-throughput equities trading systems using Vella's core components.

## Table of Contents

1. [Equities Order Book (`matching.rs`)](#equities-order-book-matchingrs)
2. [Institutional Connectivity (`fix.rs`)](#institutional-connectivity-fixrs)
3. [Algorithmic Equities Backtesting (`backtest.rs`)](#algorithmic-equities-backtesting-backtestrs)
4. [Hardware Acceleration for Equities (`fpga.rs`)](#hardware-acceleration-for-equities-fpgars)

---

## 1. Equities Order Book (`matching.rs`)

Vella provides a robust, strict Price-Time Priority (FIFO) matching engine, perfect for traditional stock market limit order books.

### Overview

The `matching.rs` module manages the state of the order book, processing incoming orders, and crossing them to execute trades. It ensures that standard equity orders (e.g., TSLA, AAPL shares) are matched fairly and efficiently.

### Example: Processing AAPL Orders

```rust
use vella::matching::{OrderBook, Order, Side, OrderType};

fn main() {
    // Initialize an order book for AAPL
    let mut aapl_book = OrderBook::new("AAPL");

    // Create a limit buy order for 100 shares of AAPL at $150.00
    let buy_order = Order {
        id: 1,
        side: Side::Buy,
        order_type: OrderType::Limit,
        price: 150.0,
        quantity: 100,
    };

    // Create a limit sell order for 50 shares of AAPL at $150.00
    let sell_order = Order {
        id: 2,
        side: Side::Sell,
        order_type: OrderType::Limit,
        price: 150.0,
        quantity: 50,
    };

    // Process orders
    aapl_book.process_order(buy_order);
    let trades = aapl_book.process_order(sell_order);

    for trade in trades {
        println!("Executed trade: {} shares at ${}", trade.quantity, trade.price);
    }
}
```

---

## 2. Institutional Connectivity (`fix.rs`)

Connecting to traditional stock exchanges like NASDAQ or NYSE requires adherence to the Financial Information eXchange (FIX) protocol. Vella's `fix.rs` module provides native support for FIX over TCP sockets.

### TCP Sockets and Cryptographic Checksums

The FIX protocol relies on strict message formatting, terminating with a modulo-256 ASCII checksum. `fix.rs` handles the TCP connection lifecycle, message framing, and automatic checksum validation.

### Example: Sending a New Order Single (35=D) Message

```rust
use vella::fix::{FixClient, FixMessage};

#[tokio::main]
async fn main() {
    // Connect to the exchange FIX gateway
    let mut client = FixClient::connect("fix.exchange.com:4000").await.unwrap();
    client.logon("SENDER_COMP_ID", "TARGET_COMP_ID").await.unwrap();

    // Construct a New Order Single message for AAPL
    let mut msg = FixMessage::new("D");
    msg.set_field(11, "ClOrdID_12345"); // ClOrdID
    msg.set_field(55, "AAPL");          // Symbol
    msg.set_field(54, "1");             // Side (1 = Buy)
    msg.set_field(38, "1000");          // OrderQty
    msg.set_field(40, "2");             // OrdType (2 = Limit)
    msg.set_field(44, "152.50");        // Price

    // The client automatically calculates the modulo-256 checksum and sends over TCP
    client.send_message(msg).await.unwrap();
}
```

---

## 3. Algorithmic Equities Backtesting (`backtest.rs`)

Before deploying to production, strategies must be rigorously backtested. `backtest.rs` allows you to load traditional stock market ticks and evaluate strategies with precise Profit and Loss (PnL) mathematical tracking.

### PnL Tracking and Evaluation

Vella's backtester simulates market conditions, allowing you to test strategies like Mean Reversion on standard equities and analyze detailed performance metrics.

### Example: Mean Reversion Strategy Backtest

```rust
use vella::backtest::{Backtester, Strategy, Tick};

struct MeanReversionStrategy;

impl Strategy for MeanReversionStrategy {
    fn on_tick(&mut self, tick: &Tick, engine: &mut Backtester) {
        // Simple logic: buy if price drops below moving average, sell if it goes above
        if tick.symbol == "TSLA" {
            if tick.price < 200.0 {
                engine.submit_buy_order("TSLA", 100, tick.price);
            } else if tick.price > 250.0 {
                engine.submit_sell_order("TSLA", 100, tick.price);
            }
        }
    }
}

fn main() {
    let mut backtester = Backtester::new();
    backtester.load_historical_data("data/tsla_ticks.csv");
    
    let mut strategy = MeanReversionStrategy;
    backtester.run(&mut strategy);
    
    let pnl = backtester.calculate_pnl();
    println!("Total PnL for TSLA Mean Reversion strategy: ${:.2}", pnl);
}
```

---

## 4. Hardware Acceleration for Equities (`fpga.rs`)

For ultra-low latency trading, Vella integrates with `rust-hdl` to transpile traditional stock trading logic directly into Verilog for zero-latency FPGA execution.

### Transpiling Trading Logic

You can define volatility parameters or price thresholds in Rust, and `fpga.rs` will generate the corresponding Verilog code, allowing the FPGA to execute trading decisions in nanoseconds.

### Example: Transpiling a Price Threshold Filter

```rust
use vella::fpga::{HardwareFilter, Transpiler};
use rust_hdl::prelude::*;

#[derive(LogicBlock)]
struct PriceThresholdFilter {
    pub price_in: Signal<In, Bits<32>>,
    pub threshold: Signal<In, Bits<32>>,
    pub trigger_out: Signal<Out, Bit>,
}

impl Logic for PriceThresholdFilter {
    #[hdl_gen]
    fn update(&mut self) {
        // Trigger output if incoming price is greater than the threshold
        self.trigger_out.next = self.price_in.val() > self.threshold.val();
    }
}

fn main() {
    let filter = PriceThresholdFilter::default();
    
    // Transpile the Rust HDL logic into Verilog for FPGA synthesis
    let verilog_code = Transpiler::generate_verilog(&filter);
    
    println!("Generated Verilog for FPGA:\n{}", verilog_code);
}
```

By leveraging Vella's extensive features, TradFi engineers can build state-of-the-art stock trading systems that operate at the cutting edge of speed and reliability.
