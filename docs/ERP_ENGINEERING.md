# Vella ERP Engineering Manual

Welcome to the **Vella ERP Engineering Manual**. This guide provides a comprehensive overview of the Enterprise Resource Planning (ERP) capabilities built directly into the Vella framework, specifically focusing on the `vella::commerce::erp` module.

As an ERP Engineer, you can use Vella to build robust supply chain, accounting, and commerce products. This manual covers everything from our inventory state machine to the integrated double-entry accounting ledger.

---

## 1. System Overview

At the heart of the Vella ERP module (`src/commerce/erp.rs`) is the `ErpSystem` struct, which acts as an in-memory orchestrator (or engine) for your transactional state. The system primarily handles two major components:

1. **Inventory Management**: A real-time state machine for tracking items, checking availability, and preventing stockouts.
2. **Double-Entry Ledgers**: An atomic, compliant accounting engine that ensures every transaction accurately records matching debits and credits.

This tight coupling guarantees that when physical inventory changes, the financial state correctly updates alongside it in one atomic flow.

---

## 2. Inventory Management

The `InventoryItem` struct allows you to define your core products.

```rust
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub stock: u64,
}
```

### Managing Stock
You load items into the ERP system using `add_inventory`. To safely reduce stock (for instance, when a customer buys a product), you use the `buy_item` method. The system implements safeguards to ensure you **cannot sell more than you have**:

```rust
// Internally checks:
if item.stock < quantity {
    return Err("Insufficient stock".to_string());
}
```

This acts as a basic yet powerful state machine, ensuring your supply chain records are always mathematically sound and physically possible.

---

## 3. Double-Entry Ledgers

To remain compliant with standard accounting practices, Vella includes a `LedgerEntry` tracking system. Every time a financial event occurs (like a purchase), the system generates balancing **Debits** and **Credits**.

```rust
pub struct LedgerEntry {
    pub transaction_id: String,
    pub account: String,
    pub amount: f64,
    pub is_credit: bool,
}
```

### Atomic Transactions
When you call `buy_item`, Vella calculates the total price and automatically inserts matching ledger entries. 

For example, a purchase will:
1. **Credit** the **Revenue** account (increasing your recorded earnings).
2. **Debit** the **Cash** account (increasing your recorded cash on hand).

Both entries share the same `transaction_id`. If either fails (or if there is insufficient stock), the entire operation aborts, preserving data integrity.

---

## 4. Integration with Vella APIs & Database

While the `ErpSystem` can run in-memory, typical production usage ties it into Vella's `ModelSchema` engine to persist data via your database and expose it via REST APIs.

You can map the ERP concepts into robust database schemas like so:

```rust
// Inventory Schema
let inventory_schema = ModelSchema::new("InventoryItem")
    .field(Field::string("name").required())
    .field(Field::string("sku").unique().required())
    .field(Field::integer("stock").default_value(serde_json::json!(0)))
    .field(Field::money("price", "USD"))
    .with_timestamps();

// Ledger Schema
let ledger_schema = ModelSchema::new("Ledger")
    .field(Field::string("transaction_id").required())
    .field(Field::string("account").required())
    .field(Field::float("amount").required())
    .field(Field::boolean("is_credit").required())
    .with_timestamps();
```

By hooking your API controllers into the `ErpSystem` engine, and saving the results against these `ModelSchema` definitions, you create a fully functional, API-first ERP.

---

## 5. Code Examples

Here is a complete, copy-pasteable example of initializing the ERP system, adding stock, executing a purchase, and verifying the double-entry logs.

```rust
use vella::prelude::*;
use vella::commerce::erp::{ErpSystem, InventoryItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Vella ERP System Demo ===");
    
    // 1. Initialize the Engine
    let mut erp = ErpSystem::new();
    
    // 2. Add Inventory
    erp.add_inventory(InventoryItem {
        id: "WIDGET-01".to_string(),
        name: "Acme Widget".to_string(),
        stock: 10,
    });
    
    println!("Initial stock for WIDGET-01: {}", erp.inventory.get("WIDGET-01").unwrap().stock);
    
    // 3. Process a Transaction (Atomic Stock & Ledger update)
    println!("Buying 5 items at $10.00 each...");
    match erp.buy_item("WIDGET-01", 5, 10.00) {
        Ok(_) => {
            println!("Transaction successful!");
            println!("Remaining stock: {}", erp.inventory.get("WIDGET-01").unwrap().stock);
            
            // 4. Verify Ledgers
            println!("Ledger entries created: {}", erp.ledger.len());
            for entry in &erp.ledger {
                println!(" -> [{}] {} {}: ${:.2}", 
                    entry.transaction_id, 
                    if entry.is_credit { "CREDIT" } else { "DEBIT" },
                    entry.account, 
                    entry.amount
                );
            }
        },
        Err(e) => println!("Transaction failed: {}", e),
    }
    
    Ok(())
}
```

**Output:**
```
=== Vella ERP System Demo ===
Initial stock for WIDGET-01: 10
Buying 5 items at $10.00 each...
Transaction successful!
Remaining stock: 5
Ledger entries created: 2
 -> [TXN_1] CREDIT Revenue: $50.00
 -> [TXN_1] DEBIT Cash: $50.00
```

With this foundation, ERP Engineers can easily extend the engine to support POs, Invoicing, Tax calculation, and complex Supply Chain routing.
