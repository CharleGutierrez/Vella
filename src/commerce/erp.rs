/// Vella ERP (Enterprise Resource Planning)
/// Real, basic implementation of double-entry ledger logic and inventory state machine.

pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub stock: u64,
}

pub struct LedgerEntry {
    pub transaction_id: String,
    pub account: String,
    pub amount: f64,
    pub is_credit: bool,
}

pub struct ErpSystem {
    pub inventory: std::collections::HashMap<String, InventoryItem>,
    pub ledger: Vec<LedgerEntry>,
}

impl ErpSystem {
    pub fn new() -> Self {
        Self {
            inventory: std::collections::HashMap::new(),
            ledger: Vec::new(),
        }
    }

    pub fn add_inventory(&mut self, item: InventoryItem) {
        self.inventory.insert(item.id.clone(), item);
    }

    pub fn buy_item(&mut self, item_id: &str, quantity: u64, price_per_unit: f64) -> Result<(), String> {
        let item = self.inventory.get_mut(item_id).ok_or("Item not found")?;
        
        if item.stock < quantity {
            return Err("Insufficient stock".to_string());
        }

        // Deduct inventory
        item.stock -= quantity;

        // Add ledger entry
        let total_price = (quantity as f64) * price_per_unit;
        let transaction_id = format!("TXN_{}", self.ledger.len() + 1);

        self.ledger.push(LedgerEntry {
            transaction_id: transaction_id.clone(),
            account: "Revenue".to_string(),
            amount: total_price,
            is_credit: true,
        });

        self.ledger.push(LedgerEntry {
            transaction_id,
            account: "Cash".to_string(),
            amount: total_price,
            is_credit: false, // debit cash
        });

        Ok(())
    }
}
