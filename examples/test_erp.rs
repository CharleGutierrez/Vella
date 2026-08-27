use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Vella ERP System Demo ===");
    
    // InventoryItem Model
    let inventory_schema = ModelSchema::new("InventoryItem")
        .field(Field::string("name").required())
        .field(Field::string("sku").unique().required())
        .field(Field::integer("stock").default_value(serde_json::json!(0)))
        .field(Field::money("price", "USD"))
        .with_timestamps();

    // Ledger Model
    let ledger_schema = ModelSchema::new("Ledger")
        .field(Field::string("transaction_id").required())
        .field(Field::string("account").required())
        .field(Field::float("amount").required())
        .field(Field::boolean("is_credit").required())
        .with_timestamps();

    // Invoice Model
    let invoice_schema = ModelSchema::new("Invoice")
        .field(Field::string("invoice_number").unique().required())
        .field(Field::string("customer_name").required())
        .field(Field::float("total_amount"))
        .field(Field::r#enum("status", vec!["Pending", "Paid", "Cancelled"]))
        .with_timestamps();

    println!("Registering ERP Schemas:");
    println!("- {}", inventory_schema.name);
    println!("- {}", ledger_schema.name);
    println!("- {}", invoice_schema.name);
    
    println!("\nSimulating transactional update...");
    use vella::commerce::erp::{ErpSystem, InventoryItem};
    
    let mut erp = ErpSystem::new();
    erp.add_inventory(InventoryItem {
        id: "WIDGET-01".to_string(),
        name: "Acme Widget".to_string(),
        stock: 10,
    });
    
    println!("Initial stock for WIDGET-01: {}", erp.inventory.get("WIDGET-01").unwrap().stock);
    println!("Buying 5 items at $10.00 each...");
    
    match erp.buy_item("WIDGET-01", 5, 10.00) {
        Ok(_) => {
            println!("Transaction successful!");
            println!("Remaining stock: {}", erp.inventory.get("WIDGET-01").unwrap().stock);
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
