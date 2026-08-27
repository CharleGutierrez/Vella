import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as crypto from 'crypto';

export function activate(context: vscode.ExtensionContext) {
    let syncSdkDisposable = vscode.commands.registerCommand('vella.syncSdk', () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('Vella: No workspace opened to sync SDK.');
            return;
        }
        const workspacePath = workspaceFolders[0].uri.fsPath;
        const outPath = path.join(workspacePath, 'vella.ts');
        
        cp.exec('curl -s http://localhost:8080/api/sdk/react', (error, stdout, stderr) => {
            if (error) {
                vscode.window.showErrorMessage(`Vella SDK Sync failed: ${error.message}`);
                return;
            }
            fs.writeFileSync(outPath, stdout);
            vscode.window.showInformationMessage('Vella: Synchronized React SDK successfully to vella.ts!');
        });
    });

    let generateWalletDisposable = vscode.commands.registerCommand('vella.generateWallet', () => {
        const id = crypto.randomBytes(32).toString('hex');
        const walletAddress = '0x' + id;
        vscode.window.showInformationMessage(`Vella: New Web3 Wallet Generated: ${walletAddress}`);
    });

    let openSchemaBuilderDisposable = vscode.commands.registerCommand('vella.openSchemaBuilder', () => {
        const panel = vscode.window.createWebviewPanel(
            'vellaSchemaBuilder',
            'Vella Schema Builder',
            vscode.ViewColumn.One,
            { enableScripts: true }
        );

        panel.webview.html = getWebviewContent();
    });

    let scaffoldReactDisposable = vscode.commands.registerCommand('vella.scaffoldReact', async () => {
        const content = `import React from 'react';
import { useVellaQuery } from '@vella/sdk';

export function VellaComponent() {
  const { data, loading, error } = useVellaQuery('your-query');

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      {data?.map((item: any) => (
        <div key={item.id}>{item.name}</div>
      ))}
    </div>
  );
}`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'typescriptreact' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldVueDisposable = vscode.commands.registerCommand('vella.scaffoldVue', async () => {
        const content = `<template>
  <div>
    <div v-if="loading">Loading...</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <div v-else>
      <div v-for="item in data" :key="item.id">
        {{ item.name }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { vellaClient } from '@vella/sdk';

const data = ref<any[]>([]);
const loading = ref(true);
const error = ref<any>(null);

onMounted(async () => {
  try {
    data.value = await vellaClient.query('your-query');
  } catch (err) {
    error.value = err;
  } finally {
    loading.value = false;
  }
});
</script>`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'vue' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldAngularDisposable = vscode.commands.registerCommand('vella.scaffoldAngular', async () => {
        const content = `import { Component, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { VellaService } from '@vella/sdk/angular';

@Component({
  selector: 'app-vella-component',
  standalone: true,
  imports: [CommonModule],
  template: \`
    <div *ngIf="loading()">Loading...</div>
    <div *ngIf="error()">Error: {{ error()?.message }}</div>
    <div *ngIf="!loading() && !error()">
      <div *ngFor="let item of data()">
        {{ item.name }}
      </div>
    </div>
  \`
})
export class VellaComponent implements OnInit {
  data = signal<any[]>([]);
  loading = signal<boolean>(true);
  error = signal<any>(null);

  constructor(private vella: VellaService) {}

  async ngOnInit() {
    try {
      const result = await this.vella.query('your-query');
      this.data.set(result);
    } catch (err) {
      this.error.set(err);
    } finally {
      this.loading.set(false);
    }
  }
}`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'typescript' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldErpSchemasDisposable = vscode.commands.registerCommand('vella.scaffoldErpSchemas', async () => {
        const content = `use vella::prelude::*;

#[derive(ModelSchema)]
pub struct Invoice {
    pub id: Id,
    pub amount: Field<Money>,
    pub status: Field<String>,
    pub created_at: Field<DateTime>,
}

#[derive(ModelSchema)]
pub struct Ledger {
    pub id: Id,
    pub account_id: Field<String>,
    pub debit: Field<Money>,
    pub credit: Field<Money>,
    pub timestamp: Field<DateTime>,
}

#[derive(ModelSchema)]
pub struct InventoryItem {
    pub id: Id,
    pub sku: Field<String>,
    pub quantity: Field<i32>,
    pub price: Field<Money>,
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldDoubleEntryLedgerDisposable = vscode.commands.registerCommand('vella.scaffoldDoubleEntryLedger', async () => {
        const content = `use vella::prelude::*;
use vella::db::Transaction;

pub async fn process_sale(
    tx: &mut Transaction,
    item_id: &str,
    qty: i32,
    price: Money
) -> Result<(), Error> {
    // Deduct inventory
    let mut item = InventoryItem::find(tx, item_id).await?;
    if item.quantity.get() < qty {
        return Err(Error::new("Insufficient inventory"));
    }
    item.quantity.set(item.quantity.get() - qty);
    item.save(tx).await?;

    let total_amount = price * qty;

    // Double-entry bookkeeping
    
    // Debit Cash (Increase Asset)
    Ledger::create(tx, Ledger {
        account_id: "Cash".to_string(),
        debit: total_amount,
        credit: Money::zero(),
    }).await?;

    // Credit Revenue (Increase Income)
    Ledger::create(tx, Ledger {
        account_id: "Revenue".to_string(),
        debit: Money::zero(),
        credit: total_amount,
    }).await?;

    Ok(())
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldLimitOrderBookDisposable = vscode.commands.registerCommand('vella.scaffoldLimitOrderBook', async () => {
        const content = `use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
}

pub struct LimitOrderBook {
    pub bids: VecDeque<Order>,
    pub asks: VecDeque<Order>,
}

impl LimitOrderBook {
    pub fn new() -> Self {
        Self {
            bids: VecDeque::new(),
            asks: VecDeque::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        // Price-Time Priority FIFO crossing logic placeholder
        match order.side {
            Side::Buy => self.bids.push_back(order),
            Side::Sell => self.asks.push_back(order),
        }
        self.match_orders();
    }

    fn match_orders(&mut self) {
        // TODO: Implement crossing logic
    }
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldTradingStrategyDisposable = vscode.commands.registerCommand('vella.scaffoldTradingStrategy', async () => {
        const content = `pub struct TickData {
    pub timestamp: i64,
    pub price: f64,
    pub volume: f64,
}

pub fn run_mean_reversion_strategy(ticks: &[TickData]) -> f64 {
    let mut pnl = 0.0;
    let mut position = 0.0;
    let mut moving_average = 0.0;
    let window_size = 10;
    let mut sum = 0.0;
    
    for (i, tick) in ticks.iter().enumerate() {
        sum += tick.price;
        
        if i >= window_size {
            sum -= ticks[i - window_size].price;
            moving_average = sum / window_size as f64;
            
            // Simple mean reversion logic
            if tick.price < moving_average {
                // Buy signal
                position += 1.0;
                pnl -= tick.price;
            } else if tick.price > moving_average && position > 0.0 {
                // Sell signal
                position -= 1.0;
                pnl += tick.price;
            }
        }
    }
    
    // Close position at the last price
    if position > 0.0 {
        pnl += position * ticks.last().unwrap().price;
    }
    
    pnl
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldSmartContractDeployerDisposable = vscode.commands.registerCommand('vella.scaffoldSmartContractDeployer', async () => {
        const content = `use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Replace with your local node URL (e.g., Hardhat or Anvil)
    let node_url = "http://localhost:8545";
    
    // Example bytecode of a compiled smart contract
    let bytecode = "0x608060405234801561001057600080fd5b506040516020806100f283398101806040528101908080519060200190929190505050806000819055505060a8806100536000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c80636d4ce63c14602d575b600080fd5b60336049565b6040518082815260200191505060405180910390f35b6000805490509056fea26469706673582212204c356942cbaefb702b36ff5c18e19c30f40cfd9237cdfb743787ddf3f7e0ef6b64736f6c634300081a0033";

    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_sendTransaction",
        "params": [{
            "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", // Example local account
            "data": bytecode
        }],
        "id": 1
    });

    println!("Sending deployment transaction...");
    let res = client.post(node_url)
        .json(&payload)
        .send()
        .await?;

    let response_body: serde_json::Value = res.json().await?;
    println!("Response: {:#?}", response_body);

    Ok(())
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldWalletGeneratorDisposable = vscode.commands.registerCommand('vella.scaffoldWalletGenerator', async () => {
        const content = `use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha3::{Digest, Keccak256};
use hex;

pub fn generate_web3_wallet() -> (String, String) {
    // Generate a new secure ECDSA secp256k1 private key
    let signing_key = SigningKey::random(&mut OsRng);
    let private_key_bytes = signing_key.to_bytes();
    let private_key_hex = hex::encode(private_key_bytes);

    // Derive the public key (verifying key)
    let verifying_key = VerifyingKey::from(&signing_key);
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();

    // Ethereum address is the last 20 bytes of the Keccak256 hash of the uncompressed public key (excluding the 0x04 prefix)
    let mut hasher = Keccak256::new();
    hasher.update(&public_key_bytes[1..]);
    let result = hasher.finalize();

    let address_bytes = &result[12..];
    let address_hex = format!("0x{}", hex::encode(address_bytes));

    (private_key_hex, address_hex)
}

fn main() {
    let (private_key, address) = generate_web3_wallet();
    println!("New Web3 Wallet Generated:");
    println!("Private Key: {}", private_key);
    println!("Address:     {}", address);
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(rocket) Vella Server';
    statusBarItem.show();

    context.subscriptions.push(
        syncSdkDisposable,
        generateWalletDisposable,
        openSchemaBuilderDisposable,
        scaffoldReactDisposable,
        scaffoldVueDisposable,
        scaffoldAngularDisposable,
        scaffoldErpSchemasDisposable,
        scaffoldDoubleEntryLedgerDisposable,
        scaffoldLimitOrderBookDisposable,
        scaffoldTradingStrategyDisposable,
        scaffoldSmartContractDeployerDisposable,
        scaffoldWalletGeneratorDisposable,
        statusBarItem
    );
}

function getWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vella Schema Builder</title>
    <style>
        body {
            background-color: #1e1e1e;
            color: #d4d4d4;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            margin: 0;
            padding: 20px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
        }
        h1 {
            font-size: 24px;
            margin: 0;
            color: #61afef;
        }
        button {
            background-color: #0e639c;
            color: white;
            border: none;
            padding: 10px 15px;
            font-size: 14px;
            border-radius: 4px;
            cursor: pointer;
            margin-left: 10px;
        }
        button:hover {
            background-color: #1177bb;
        }
        .canvas {
            background-color: #252526;
            border: 1px solid #3c3c3c;
            border-radius: 8px;
            min-height: 500px;
            position: relative;
            overflow: hidden;
        }
        .node {
            background-color: #2d2d30;
            border: 1px solid #555;
            border-radius: 6px;
            width: 250px;
            position: absolute;
            box-shadow: 0 4px 6px rgba(0,0,0,0.3);
            display: flex;
            flex-direction: column;
        }
        .node-header {
            background-color: #3f3f46;
            padding: 10px;
            border-top-left-radius: 5px;
            border-top-right-radius: 5px;
            font-weight: bold;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .node-content {
            padding: 10px;
        }
        .field {
            display: flex;
            justify-content: space-between;
            margin-bottom: 8px;
            font-size: 13px;
        }
        .field-name {
            color: #9cdcfe;
        }
        .field-type {
            color: #4ec9b0;
        }
        .connection {
            position: absolute;
            border-top: 2px dashed #61afef;
            width: 150px;
            top: 150px;
            left: 280px;
            transform: rotate(15deg);
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Vella Visual Schema Builder</h1>
        <div>
            <button>Add Model</button>
            <button>Save Schema</button>
        </div>
    </div>
    
    <div class="canvas">
        <div class="node" style="top: 50px; left: 50px;">
            <div class="node-header">
                <span>User</span>
                <span style="font-size: 12px; color: #aaa;">@model</span>
            </div>
            <div class="node-content">
                <div class="field">
                    <span class="field-name">id</span>
                    <span class="field-type">String @id</span>
                </div>
                <div class="field">
                    <span class="field-name">username</span>
                    <span class="field-type">String @unique</span>
                </div>
                <div class="field">
                    <span class="field-name">balance</span>
                    <span class="field-type">Float</span>
                </div>
                <div class="field" style="margin-top: 10px; text-align: center; color: #888; cursor: pointer;">
                    + Add Field
                </div>
            </div>
        </div>

        <div class="connection"></div>

        <div class="node" style="top: 180px; left: 400px;">
            <div class="node-header">
                <span>Invoice</span>
                <span style="font-size: 12px; color: #aaa;">@model</span>
            </div>
            <div class="node-content">
                <div class="field">
                    <span class="field-name">id</span>
                    <span class="field-type">String @id</span>
                </div>
                <div class="field">
                    <span class="field-name">amount</span>
                    <span class="field-type">Float</span>
                </div>
                <div class="field">
                    <span class="field-name">userId</span>
                    <span class="field-type">String</span>
                </div>
                <div class="field" style="margin-top: 10px; text-align: center; color: #888; cursor: pointer;">
                    + Add Field
                </div>
            </div>
        </div>
    </div>
</body>
</html>\`;
}

export function deactivate() {}
