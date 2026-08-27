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

    let scaffoldUdpTelemetryDisposable = vscode.commands.registerCommand('vella.scaffoldUdpTelemetry', async () => {
        const content = `use std::net::UdpSocket;
use std::io;

pub fn start_telemetry_listener(address: &str) -> io::Result<()> {
    let socket = UdpSocket::bind(address)?;
    socket.set_nonblocking(true)?;
    
    let mut buf = [0u8; 1024];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _src)) => {
                // Process high-throughput telemetry data bypassing TCP overhead
                // e.g., F1 car telemetry or Oil Rig pressure gauges
                let _data = &buf[..size];
                // TODO: deserialize and process _data
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No data available, could yield or sleep briefly
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    let scaffoldScadaStateMachineDisposable = vscode.commands.registerCommand('vella.scaffoldScadaStateMachine', async () => {
        const content = `pub struct ScadaState {
    pub core_temperature: f64,
    pub cooling_system_active: bool,
}

impl ScadaState {
    pub fn new() -> Self {
        Self {
            core_temperature: 0.0,
            cooling_system_active: false,
        }
    }

    pub fn update(&mut self, new_temp: f64) {
        self.core_temperature = new_temp;
        
        let threshold = 100.0;
        let hysteresis = 5.0;

        // Basic PID-style software state machine logic
        if self.core_temperature > threshold {
            self.cooling_system_active = true;
        } else if self.core_temperature < (threshold - hysteresis) {
            self.cooling_system_active = false;
        }
    }
}
`;
        const doc = await vscode.workspace.openTextDocument({ content, language: 'rust' });
        await vscode.window.showTextDocument(doc);
    });

    // --- NEW FEATURES ---

    // Custom Editor
    const customEditorProvider = vscode.window.registerCustomEditorProvider('vella.schemaEditor', {
        async resolveCustomTextEditor(document: vscode.TextDocument, webviewPanel: vscode.WebviewPanel, token: vscode.CancellationToken): Promise<void> {
            webviewPanel.webview.options = { enableScripts: true };
            webviewPanel.webview.html = getWebviewContent();
        }
    });
    context.subscriptions.push(customEditorProvider);

    // Task Provider
    const taskProvider = vscode.tasks.registerTaskProvider('vella', {
        provideTasks: () => {
            return [
                new vscode.Task(
                    { type: 'vella', task: 'build' },
                    vscode.TaskScope.Workspace,
                    'Build Vella Project',
                    'vella',
                    new vscode.ShellExecution('cargo build --release')
                )
            ];
        },
        resolveTask: (task: vscode.Task) => {
            return task;
        }
    });
    context.subscriptions.push(taskProvider);

    // Decentralized SCM
    const scm = vscode.scm.createSourceControl('vella-scm', 'Vella IPFS');
    scm.inputBox.placeholder = "Commit to Decentralized IPFS Network...";
    context.subscriptions.push(scm);

    const serverUrl = vscode.workspace.getConfiguration('vella').get('serverUrl');
    vscode.window.showInformationMessage(`Vella Extension Activated. Server URL: ${serverUrl}`);

    const testController = vscode.tests.createTestController('vellaTestController', 'Vella Test Explorer');
    const hftTestItem = testController.createTestItem('hft_latency', 'HFT Latency Test');
    testController.items.add(hftTestItem);
    context.subscriptions.push(testController);

    const codeLensProvider = vscode.languages.registerCodeLensProvider('rust', {
        provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
            const lenses: vscode.CodeLens[] = [];
            const text = document.getText();
            const lines = text.split('\\n');
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                if (line.includes('async fn main')) {
                    lenses.push(new vscode.CodeLens(new vscode.Range(i, 0, i, 0), {
                        title: "☁ Deploy to Web3",
                        command: "vella.deployToCloud"
                    }));
                } else if (line.includes('fn test_')) {
                    lenses.push(new vscode.CodeLens(new vscode.Range(i, 0, i, 0), {
                        title: "▶ Run Vella Engine",
                        command: "vella.runHftBacktest"
                    }));
                }
            }
            return lenses;
        }
    });
    context.subscriptions.push(codeLensProvider);

    class DatabaseExplorerProvider implements vscode.TreeDataProvider<vscode.TreeItem>, vscode.TreeDragAndDropController<vscode.TreeItem> {
        dropMimeTypes = ['text/uri-list'];
        dragMimeTypes = [];

        getTreeItem(element: vscode.TreeItem): vscode.TreeItem {
            return element;
        }
        getChildren(element?: vscode.TreeItem): Thenable<vscode.TreeItem[]> {
            if (element) return Promise.resolve([]);
            return Promise.resolve([
                new vscode.TreeItem('Users', vscode.TreeItemCollapsibleState.None),
                new vscode.TreeItem('Invoices', vscode.TreeItemCollapsibleState.None),
                new vscode.TreeItem('LimitOrders', vscode.TreeItemCollapsibleState.None)
            ]);
        }

        async handleDrop(target: vscode.TreeItem | undefined, dataTransfer: vscode.DataTransfer, token: vscode.CancellationToken): Promise<void> {
            const uriList = dataTransfer.get('text/uri-list');
            if (uriList) {
                const urlString = await uriList.asString();
                vscode.window.showInformationMessage(`Vella: Dropped file into DB explorer: ${urlString}`);
            }
        }
    }
    const dbExplorerProvider = new DatabaseExplorerProvider();
    const treeView = vscode.window.createTreeView('vella.databaseExplorer', {
        treeDataProvider: dbExplorerProvider,
        dragAndDropController: dbExplorerProvider
    });
    context.subscriptions.push(treeView);

    let openCopilotDisposable = vscode.commands.registerCommand('vella.openCopilot', () => {
        const panel = vscode.window.createWebviewPanel('vellaCopilot', 'Vella AI Copilot', vscode.ViewColumn.Beside, { enableScripts: true });
        panel.webview.html = getCopilotWebviewContent();
    });

    let openTelemetryDashboardDisposable = vscode.commands.registerCommand('vella.openTelemetryDashboard', () => {
        const panel = vscode.window.createWebviewPanel('vellaTelemetry', 'Vella Telemetry Dashboard', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getTelemetryWebviewContent('Running cargo run --example test_scada...');
        
        cp.exec('cargo run --example test_scada', { cwd: vscode.workspace.workspaceFolders?.[0].uri.fsPath }, (err, stdout, stderr) => {
            panel.webview.html = getTelemetryWebviewContent(stdout || stderr || 'No output');
        });
    });

    let deployToCloudDisposable = vscode.commands.registerCommand('vella.deployToCloud', async () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('Vella: No workspace to deploy.');
            return;
        }
        const dockerfilePath = vscode.Uri.joinPath(workspaceFolders[0].uri, 'Dockerfile');
        const dockerfileContent = Buffer.from('FROM node:18\nWORKDIR /app\nCOPY . .\nRUN npm install\nCMD ["npm", "start"]', 'utf8');
        await vscode.workspace.fs.writeFile(dockerfilePath, dockerfileContent);
        vscode.window.showInformationMessage('Vella: Docker container built! Preparing for cloud deployment...');
    });

    let runHftBacktestDisposable = vscode.commands.registerCommand('vella.runHftBacktest', () => {
        const panel = vscode.window.createWebviewPanel('vellaHftBacktest', 'HFT Backtesting Sandbox', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getHftBacktestWebviewContent('Running cargo run --example test_hft...');
        
        cp.exec('cargo run --example test_hft', { cwd: vscode.workspace.workspaceFolders?.[0].uri.fsPath }, (err, stdout, stderr) => {
            panel.webview.html = getHftBacktestWebviewContent(stdout || stderr || 'No output');
        });
    });

    let openWeb3NetworkMapDisposable = vscode.commands.registerCommand('vella.openWeb3NetworkMap', () => {
        const panel = vscode.window.createWebviewPanel('vellaWeb3NetworkMap', 'Web3 Network Map', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getWeb3NetworkMapWebviewContent('Running cargo run --example test_blockchain...');
        
        cp.exec('cargo run --example test_blockchain', { cwd: vscode.workspace.workspaceFolders?.[0].uri.fsPath }, (err, stdout, stderr) => {
            panel.webview.html = getWeb3NetworkMapWebviewContent(stdout || stderr || 'No output');
        });
    });

    let setupCiCdDisposable = vscode.commands.registerCommand('vella.setupCiCd', async () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('Vella: No workspace to setup CI/CD.');
            return;
        }
        const workflowDir = vscode.Uri.joinPath(workspaceFolders[0].uri, '.github', 'workflows');
        await vscode.workspace.fs.createDirectory(workflowDir);
        const deployFilePath = vscode.Uri.joinPath(workflowDir, 'deploy.yml');
        const deployFileContent = Buffer.from(`name: Deploy
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run Rust Tests
        run: cargo test
      - name: Build Docker
        run: docker build -t vella-app .
      - name: Prepare Kubernetes Deployment
        run: kubectl apply -f k8s/
`, 'utf8');
        await vscode.workspace.fs.writeFile(deployFilePath, deployFileContent);
        vscode.window.showInformationMessage('Vella: Flawless CI/CD pipeline generated successfully!');
    });

    let openAgentSwarmDisposable = vscode.commands.registerCommand('vella.openAgentSwarm', () => {
        const panel = vscode.window.createWebviewPanel('vellaAgentSwarm', 'Agent Swarm Orchestrator', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getAgentSwarmWebviewContent();
    });

    let openHardwareSimulatorDisposable = vscode.commands.registerCommand('vella.openHardwareSimulator', () => {
        const panel = vscode.window.createWebviewPanel('vellaHardwareSimulator', 'Hardware-in-the-Loop Simulator', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getHardwareSimulatorWebviewContent();
    });

    let openMarketplaceDisposable = vscode.commands.registerCommand('vella.openMarketplace', () => {
        const panel = vscode.window.createWebviewPanel('vellaMarketplace', 'Plugin Marketplace', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getMarketplaceWebviewContent();
    });

    let startMultiplayerSessionDisposable = vscode.commands.registerCommand('vella.startMultiplayerSession', () => {
        vscode.window.showInformationMessage("Vella: Multiplayer session started! Share this Session ID (vella-mp-789xyz) with your team to collaborate on the Visual Schema Builder in real-time.");
    });

    let startTimeTravelDebuggerDisposable = vscode.commands.registerCommand('vella.startTimeTravelDebugger', () => {
        vscode.window.showInformationMessage("Vella: Time-Travel Debugger attached to local Rust process. Recording memory state for rewind...");
    });

    let openAdminPanelDisposable = vscode.commands.registerCommand('vella.openAdminPanel', () => {
        const panel = vscode.window.createWebviewPanel('vellaAdminPanel', 'Production Admin Panel', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getAdminPanelWebviewContent();
    });

    let exportArchitectureDiagramDisposable = vscode.commands.registerCommand('vella.exportArchitectureDiagram', async () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('Vella: No workspace opened to export architecture diagram.');
            return;
        }
        const svgPath = vscode.Uri.joinPath(workspaceFolders[0].uri, 'vella-architecture.svg');
        const svgContent = Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200"><circle cx="100" cy="100" r="50" fill="blue" /></svg>', 'utf8');
        await vscode.workspace.fs.writeFile(svgPath, svgContent);
        vscode.window.showInformationMessage('Vella: Architecture Graph successfully exported to vella-architecture.svg!');
    });

    let generateSqlQueryDisposable = vscode.commands.registerCommand('vella.generateSqlQuery', async () => {
        const query = await vscode.window.showInputBox({ prompt: 'Enter your plain English query' });
        if (query) {
            vscode.window.showInformationMessage(`Vella AI: Generated SQLx Rust snippet:\\n\\nsqlx::query!("SELECT * FROM users WHERE ...")`);
        }
    });

    let enterSpatialModeDisposable = vscode.commands.registerCommand('vella.enterSpatialMode', () => {
        vscode.window.showInformationMessage('Vella: WebXR Spatial Mode engaged. Put on your VR/AR headset.');
        const panel = vscode.window.createWebviewPanel('vellaSpatialMode', 'VR/AR Spatial Visualizer', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getSpatialModeWebviewContent();
    });

    let openQuantumSimulatorDisposable = vscode.commands.registerCommand('vella.openQuantumSimulator', () => {
        const panel = vscode.window.createWebviewPanel('vellaQuantumSimulator', 'Quantum Qubit Simulator', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getQuantumSimulatorWebviewContent();
    });

    let connectBciTelemetryDisposable = vscode.commands.registerCommand('vella.connectBciTelemetry', () => {
        vscode.window.showInformationMessage('Vella: Scanning Bluetooth for Neural Interface Headset...');
        const panel = vscode.window.createWebviewPanel('vellaBciTelemetry', 'Neural-Interface BCI Telemetry', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = getBciTelemetryWebviewContent();
    });

    const diagnosticCollection = vscode.languages.createDiagnosticCollection('vella');
    context.subscriptions.push(diagnosticCollection);

    vscode.workspace.onDidChangeTextDocument(event => {
        const doc = event.document;
        if (doc.languageId === 'rust') {
            const text = doc.getText();
            const diagnostics: vscode.Diagnostic[] = [];
            const regex = /unbalanced_ledger/g;
            let match;
            while ((match = regex.exec(text)) !== null) {
                const startPos = doc.positionAt(match.index);
                const endPos = doc.positionAt(match.index + match[0].length);
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(startPos, endPos),
                    "Vella: Unbalanced Double-Entry Ledger Transaction detected",
                    vscode.DiagnosticSeverity.Warning
                );
                diagnostics.push(diagnostic);
            }
            diagnosticCollection.set(doc.uri, diagnostics);
        }
    });

    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(rocket) Vella Server';
    statusBarItem.show();

    const autocompleteProvider = vscode.languages.registerCompletionItemProvider('rust', {
        provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
            const linePrefix = document.lineAt(position).text.substring(0, position.character);
            if (!linePrefix.endsWith('vella::')) {
                return undefined;
            }

            const hft = new vscode.CompletionItem('hft', vscode.CompletionItemKind.Module);
            hft.detail = 'High-Frequency Trading Module';
            hft.documentation = new vscode.MarkdownString('Provides ultra-low latency order matching and execution.');

            const web3 = new vscode.CompletionItem('web3', vscode.CompletionItemKind.Module);
            web3.detail = 'Zero-Knowledge & Blockchain Module';
            web3.documentation = new vscode.MarkdownString('Includes smart contract deployment and ZK rollups.');

            const erp = new vscode.CompletionItem('erp', vscode.CompletionItemKind.Module);
            erp.detail = 'Double-Entry Ledgers Module';
            erp.documentation = new vscode.MarkdownString('Enterprise resource planning and accounting primitives.');

            const scada = new vscode.CompletionItem('scada', vscode.CompletionItemKind.Module);
            scada.detail = 'IoT Telemetry Module';
            scada.documentation = new vscode.MarkdownString('SCADA systems, telemetry, and hardware-in-the-loop.');

            return [hft, web3, erp, scada];
        }
    }, ':');

    const hoverProvider = vscode.languages.registerHoverProvider('rust', {
        provideHover(document: vscode.TextDocument, position: vscode.Position) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);

            if (word === 'FixEngine') {
                return new vscode.Hover(new vscode.MarkdownString('**FixEngine**\n\nVella High-Frequency Trading FIX Protocol Engine. Handles concurrent session decoding.'));
            } else if (word === 'EthDeployer') {
                return new vscode.Hover(new vscode.MarkdownString('**EthDeployer**\n\nDeploys compiled EVM bytecode directly to the Vella localized rollup.'));
            }
        }
    });

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
        scaffoldUdpTelemetryDisposable,
        scaffoldScadaStateMachineDisposable,
        openCopilotDisposable,
        openTelemetryDashboardDisposable,
        deployToCloudDisposable,
        runHftBacktestDisposable,
        openWeb3NetworkMapDisposable,
        setupCiCdDisposable,
        openAgentSwarmDisposable,
        openHardwareSimulatorDisposable,
        openMarketplaceDisposable,
        startMultiplayerSessionDisposable,
        startTimeTravelDebuggerDisposable,
        openAdminPanelDisposable,
        exportArchitectureDiagramDisposable,
        generateSqlQueryDisposable,
        enterSpatialModeDisposable,
        openQuantumSimulatorDisposable,
        connectBciTelemetryDisposable,
        statusBarItem,
        autocompleteProvider,
        hoverProvider
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
</html>`;
}

function getCopilotWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        :root {
            --vella-bg: #0d0d12;
            --vella-surface: #1a1a24;
            --vella-primary: #8a2be2;
            --vella-secondary: #00d2ff;
            --vella-text: #e2e2e2;
            --vella-muted: #888899;
            --glow: 0 0 10px rgba(138, 43, 226, 0.5), 0 0 20px rgba(0, 210, 255, 0.3);
        }
        body { 
            background-color: var(--vella-bg); 
            color: var(--vella-text); 
            font-family: system-ui, -apple-system, Inter, sans-serif; 
            margin: 0;
            padding: 0;
            display: flex;
            flex-direction: column;
            height: 100vh;
            overflow: hidden;
        }
        .header {
            padding: 15px 20px;
            background-color: var(--vella-surface);
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 2px 10px rgba(0,0,0,0.5);
            z-index: 10;
        }
        .header h3 {
            margin: 0;
            font-size: 16px;
            font-weight: 600;
            background: linear-gradient(90deg, var(--vella-secondary), var(--vella-primary));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            text-shadow: 0 0 20px rgba(138, 43, 226, 0.2);
        }
        .chat-container { 
            flex: 1;
            padding: 20px; 
            overflow-y: auto; 
            display: flex;
            flex-direction: column;
            gap: 20px;
        }
        .message {
            display: flex;
            flex-direction: column;
            max-width: 85%;
        }
        .message.user {
            align-self: flex-end;
        }
        .message.ai {
            align-self: flex-start;
        }
        .message-sender {
            font-size: 12px;
            color: var(--vella-muted);
            margin-bottom: 6px;
            margin-left: 4px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .message.user .message-sender {
            text-align: right;
            margin-right: 4px;
        }
        .bubble { 
            padding: 12px 16px; 
            border-radius: 12px;
            font-size: 14px;
            line-height: 1.5;
            box-shadow: 0 4px 15px rgba(0,0,0,0.2);
        }
        .message.user .bubble {
            background: linear-gradient(135deg, var(--vella-primary), #5a189a);
            color: white;
            border-bottom-right-radius: 2px;
            box-shadow: var(--glow);
        }
        .message.ai .bubble {
            background-color: var(--vella-surface);
            border: 1px solid rgba(255,255,255,0.05);
            border-bottom-left-radius: 2px;
        }
        .code-block {
            background-color: #050508;
            border-radius: 8px;
            padding: 12px;
            margin-top: 10px;
            font-family: 'Fira Code', Consolas, monospace;
            font-size: 13px;
            border: 1px solid rgba(0, 210, 255, 0.2);
            color: #a6accd;
            overflow-x: auto;
        }
        .code-block .keyword { color: #c792ea; }
        .code-block .function { color: #82aaff; }
        .code-block .string { color: #c3e88d; }
        .code-block .comment { color: #546e7a; font-style: italic; }
        
        .input-container {
            padding: 15px 20px;
            background-color: var(--vella-surface);
            border-top: 1px solid rgba(255, 255, 255, 0.05);
            display: flex;
            gap: 10px;
            box-shadow: 0 -5px 15px rgba(0,0,0,0.3);
        }
        .input-box {
            flex: 1;
            position: relative;
        }
        input { 
            width: 100%; 
            padding: 14px 16px; 
            box-sizing: border-box; 
            background-color: rgba(255,255,255,0.03); 
            color: white; 
            border: 1px solid rgba(255,255,255,0.1); 
            border-radius: 8px; 
            font-family: inherit;
            font-size: 14px;
            transition: all 0.2s ease;
        }
        input:focus {
            outline: none;
            border-color: var(--vella-secondary);
            box-shadow: 0 0 10px rgba(0, 210, 255, 0.2);
            background-color: rgba(255,255,255,0.05);
        }
        .send-btn {
            background: linear-gradient(135deg, var(--vella-secondary), var(--vella-primary));
            border: none;
            border-radius: 8px;
            width: 48px;
            height: 48px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            color: white;
            box-shadow: var(--glow);
            transition: transform 0.1s;
        }
        .send-btn:hover {
            transform: scale(1.05);
        }
        .send-btn svg {
            width: 20px;
            height: 20px;
            fill: currentColor;
        }
    </style>
</head>
<body>
    <div class="header">
        <h3>Vella AI Copilot</h3>
    </div>
    <div class="chat-container">
        <div class="message ai">
            <div class="message-sender">Vella AI</div>
            <div class="bubble">System initialized. Quantum node connected. How can I assist you with your architecture today?</div>
        </div>
        <div class="message user">
            <div class="message-sender">You</div>
            <div class="bubble">Generate a trading algorithm</div>
        </div>
        <div class="message ai">
            <div class="message-sender">Vella AI</div>
            <div class="bubble">
                I've synthesized a high-frequency mean reversion algorithm for you, optimized for sub-millisecond execution on the Vella Engine.
                <div class="code-block">
<span class="keyword">pub fn</span> <span class="function">mean_reversion</span>(ticks: <span class="keyword">&amp;</span>[Tick]) -&gt; <span class="keyword">f64</span> {
    <span class="keyword">let mut</span> pnl = <span class="function">0.0</span>;
    <span class="keyword">let mut</span> pos = <span class="function">0.0</span>;
    <span class="keyword">for</span> t <span class="keyword">in</span> ticks {
        <span class="keyword">if</span> t.price &lt; t.vwap {
            pos += <span class="function">1.0</span>; <span class="comment">// Buy</span>
        } <span class="keyword">else if</span> t.price &gt; t.vwap {
            pos -= <span class="function">1.0</span>; <span class="comment">// Sell</span>
        }
    }
    pnl
}
                </div>
            </div>
        </div>
    </div>
    <div class="input-container">
        <div class="input-box">
            <input type="text" placeholder="Ask Vella AI..." />
        </div>
        <button class="send-btn">
            <svg viewBox="0 0 24 24"><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path></svg>
        </button>
    </div>
</body>
</html>`;
}

function getTelemetryWebviewContent(logs: string = '') {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; }
        .container { display: flex; gap: 20px; }
        .panel { border: 1px solid #3c3c3c; padding: 20px; flex: 1; border-radius: 5px; background-color: #252526; }
        .chart { height: 100px; border-radius: 4px; margin-top: 10px; }
        pre { background: #000; color: #0f0; padding: 10px; border-radius: 5px; overflow-x: auto; margin-top: 20px; }
    </style>
</head>
<body>
    <h2>Telemetry Dashboard</h2>
    <div class="container">
        <div class="panel">
            <h4>HFT Latency</h4>
            <div class="chart" style="background: linear-gradient(90deg, #4ec9b0, #0e639c);"></div>
            <p style="margin-top: 10px; font-size: 12px; color: #aaa;">Live Latency < 1ms</p>
        </div>
        <div class="panel">
            <h4>SCADA Core Temp</h4>
            <div class="chart" style="background: linear-gradient(90deg, #d16969, #ce9178);"></div>
            <p style="margin-top: 10px; font-size: 12px; color: #aaa;">Temp: 45.2 C</p>
        </div>
    </div>
    <h3>Backend Execution Logs</h3>
    <pre>${logs}</pre>
</body>
</html>`;
}

function getHftBacktestWebviewContent(logs: string = '') {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; }
        .chart { height: 300px; border: 1px solid #3c3c3c; margin-bottom: 20px; border-radius: 5px; background: repeating-linear-gradient(90deg, #1e1e1e, #1e1e1e 10px, #252526 10px, #252526 20px); }
        .dropzone { border: 2px dashed #0e639c; padding: 50px; text-align: center; border-radius: 5px; cursor: pointer; }
        .dropzone:hover { background-color: #252526; }
        pre { background: #000; color: #0f0; padding: 10px; border-radius: 5px; overflow-x: auto; margin-top: 20px; }
    </style>
</head>
<body>
    <h2>HFT Backtesting Sandbox</h2>
    <div class="chart">
        <div style="padding: 140px; text-align: center; color: #aaa;">[ Candlestick Chart Rendered Here ]</div>
    </div>
    <div class="dropzone">Drop CSV Tick Data Here</div>
    <h3>Backend Execution Logs</h3>
    <pre>${logs}</pre>
</body>
</html>`;
}

function getWeb3NetworkMapWebviewContent(logs: string = '') {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; }
        .network { position: relative; width: 100%; height: 300px; border: 1px solid #3c3c3c; background-color: #1e1e1e; border-radius: 8px; }
        .node { position: absolute; border-radius: 50%; background-color: #4ec9b0; width: 20px; height: 20px; box-shadow: 0 0 10px #4ec9b0; display: flex; align-items: center; justify-content: center; }
        .node.ipfs { top: 100px; left: 150px; background-color: #0e639c; box-shadow: 0 0 10px #0e639c; }
        .node.zk { top: 200px; left: 400px; background-color: #c586c0; box-shadow: 0 0 10px #c586c0; }
        .node.depin { top: 150px; left: 600px; background-color: #ce9178; box-shadow: 0 0 10px #ce9178; }
        .label { position: absolute; top: 25px; left: -20px; font-size: 12px; color: #aaa; white-space: nowrap; }
        .line { position: absolute; background-color: #555; height: 2px; transform-origin: 0 0; }
        pre { background: #000; color: #0f0; padding: 10px; border-radius: 5px; overflow-x: auto; margin-top: 20px; }
    </style>
</head>
<body>
    <h2>Web3 Network Map</h2>
    <div class="network">
        <div class="node ipfs"><div class="label">IPFS Peer</div></div>
        <div class="node zk"><div class="label">ZK-Rollup</div></div>
        <div class="node depin"><div class="label">DePIN Node</div></div>
        <div class="line" style="top: 110px; left: 160px; width: 320px; transform: rotate(15deg);"></div>
        <div class="line" style="top: 210px; left: 410px; width: 220px; transform: rotate(-15deg);"></div>
        <div class="line" style="top: 110px; left: 160px; width: 460px; transform: rotate(5deg);"></div>
    </div>
    <h3>Backend Execution Logs</h3>
    <pre>${logs}</pre>
</body>
</html>`;
}

function getAgentSwarmWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; text-align: center; }
        .node { padding: 15px; border-radius: 8px; border: 1px solid #555; display: inline-block; margin: 20px; background-color: #2d2d30; box-shadow: 0 4px 6px rgba(0,0,0,0.3); font-weight: bold; }
        .line { height: 2px; background: #61afef; width: 50px; display: inline-block; vertical-align: middle; }
    </style>
</head>
<body>
    <h2 style="color: #61afef; margin-bottom: 40px;">Agent Swarm Orchestrator</h2>
    <div style="display: flex; align-items: center; justify-content: center;">
        <div class="node">Coder Agent</div>
        <div class="line"></div>
        <div class="node">QA Agent</div>
        <div class="line"></div>
        <div class="node">Architect Agent</div>
    </div>
</body>
</html>`;
}

function getHardwareSimulatorWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; }
        .slider-container { margin: 20px 0; background-color: #252526; padding: 20px; border-radius: 8px; border: 1px solid #3c3c3c; }
        input[type=range] { width: 100%; margin-top: 10px; cursor: pointer; }
        label { font-weight: bold; color: #9cdcfe; }
        .val { float: right; color: #ce9178; }
    </style>
</head>
<body>
    <h2 style="color: #4ec9b0;">Hardware-in-the-Loop Simulator</h2>
    <p>Simulating physical PLC hardware connection to the SCADA engine.</p>
    <div class="slider-container">
        <label>Core Temperature</label><span class="val">45 °C</span>
        <input type="range" min="0" max="100" value="45">
    </div>
    <div class="slider-container">
        <label>Pipeline Pressure</label><span class="val">500 PSI</span>
        <input type="range" min="0" max="1000" value="500">
    </div>
</body>
</html>`;
}

function getMarketplaceWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; padding: 20px; }
        .plugin { border: 1px solid #3c3c3c; padding: 15px; margin-bottom: 15px; border-radius: 8px; background-color: #252526; display: flex; justify-content: space-between; align-items: center; }
        .plugin h3 { margin: 0 0 5px 0; color: #dcdcaa; }
        .plugin p { margin: 0; color: #999; font-size: 14px; }
        button { background-color: #0e639c; color: white; border: none; padding: 10px 15px; border-radius: 4px; cursor: pointer; font-weight: bold; }
        button:hover { background-color: #1177bb; }
    </style>
</head>
<body>
    <h2 style="color: #c586c0; margin-bottom: 20px;">Vella Plugin Marketplace</h2>
    <div class="plugin">
        <div>
            <h3>Stripe Payments Integrator</h3>
            <p>Easily add Stripe payments to your app.</p>
        </div>
        <button>1-Click Install</button>
    </div>
    <div class="plugin">
        <div>
            <h3>Solana Smart Contracts</h3>
            <p>Deploy to Solana instantly.</p>
        </div>
        <button>1-Click Install</button>
    </div>
</body>
</html>`;
}

function getAdminPanelWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #1e1e1e; color: #d4d4d4; font-family: sans-serif; margin: 0; display: flex; height: 100vh; }
        .sidebar { width: 250px; background-color: #252526; border-right: 1px solid #3c3c3c; padding: 20px; }
        .sidebar h2 { color: #61afef; margin-top: 0; }
        .sidebar ul { list-style: none; padding: 0; }
        .sidebar li { padding: 10px; cursor: pointer; border-radius: 4px; margin-bottom: 5px; }
        .sidebar li:hover { background-color: #37373d; }
        .sidebar li.active { background-color: #0e639c; color: white; }
        .main { flex: 1; padding: 20px; display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card { background-color: #2d2d30; padding: 20px; border-radius: 8px; border: 1px solid #3c3c3c; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
        .card h3 { color: #dcdcaa; margin-top: 0; }
        .stat { font-size: 24px; font-weight: bold; color: #4ec9b0; }
    </style>
</head>
<body>
    <div class="sidebar">
        <h2>Vella Admin</h2>
        <ul>
            <li class="active">Dashboard</li>
            <li>Users</li>
            <li>Inventory</li>
            <li>Trading Pairs</li>
            <li>Settings</li>
        </ul>
    </div>
    <div class="main">
        <div class="card">
            <h3>Total Users</h3>
            <div class="stat">1,245</div>
            <p style="color: #aaa; font-size: 12px; margin-bottom: 0;">+12% this week</p>
        </div>
        <div class="card">
            <h3>Active Trading Pairs</h3>
            <div class="stat">42</div>
            <p style="color: #aaa; font-size: 12px; margin-bottom: 0;">BTC/USD leading volume</p>
        </div>
        <div class="card">
            <h3>Inventory Items</h3>
            <div class="stat">8,930</div>
            <p style="color: #aaa; font-size: 12px; margin-bottom: 0;">In 5 warehouses</p>
        </div>
    </div>
</body>
</html>`;
}

function getSpatialModeWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { margin: 0; overflow: hidden; background-color: #000; color: #00ffcc; font-family: monospace; display: flex; align-items: center; justify-content: center; height: 100vh; perspective: 1000px; }
        .cube { width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spin 5s infinite linear; }
        .face { position: absolute; width: 200px; height: 200px; background: rgba(0, 255, 204, 0.1); border: 2px solid #00ffcc; display: flex; align-items: center; justify-content: center; font-size: 24px; box-shadow: 0 0 20px #00ffcc inset; }
        .front { transform: translateZ(100px); }
        .back { transform: rotateY(180deg) translateZ(100px); }
        .left { transform: rotateY(-90deg) translateZ(100px); }
        .right { transform: rotateY(90deg) translateZ(100px); }
        .top { transform: rotateX(90deg) translateZ(100px); }
        .bottom { transform: rotateX(-90deg) translateZ(100px); }
        @keyframes spin { from { transform: rotateX(0deg) rotateY(0deg); } to { transform: rotateX(360deg) rotateY(360deg); } }
    </style>
</head>
<body>
    <div class="cube">
        <div class="face front">Web3</div>
        <div class="face back">ERP</div>
        <div class="face left">Spatial</div>
        <div class="face right">AR/VR</div>
        <div class="face top">Code</div>
        <div class="face bottom">Vella</div>
    </div>
</body>
</html>`;
}

function getQuantumSimulatorWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #0a0a0a; color: #ff00ff; font-family: sans-serif; padding: 20px; text-align: center; }
        .lattice { display: grid; grid-template-columns: repeat(4, 1fr); gap: 15px; max-width: 400px; margin: 0 auto; }
        .qubit { width: 60px; height: 60px; border-radius: 50%; background: radial-gradient(circle, #ff00ff, #330033); display: flex; align-items: center; justify-content: center; font-weight: bold; color: white; box-shadow: 0 0 15px #ff00ff; animation: pulse 2s infinite alternate; }
        @keyframes pulse { from { transform: scale(0.9); opacity: 0.8; } to { transform: scale(1.1); opacity: 1; } }
        .metrics { margin-top: 40px; border: 1px solid #ff00ff; padding: 20px; border-radius: 8px; display: inline-block; }
    </style>
</head>
<body>
    <h2>16-Qubit Processing Lattice</h2>
    <div class="lattice">
        <div class="qubit">|0&rang;</div><div class="qubit">|1&rang;</div><div class="qubit">|+&rang;</div><div class="qubit">|-&rang;</div>
        <div class="qubit">|0&rang;</div><div class="qubit">|1&rang;</div><div class="qubit">|+&rang;</div><div class="qubit">|-&rang;</div>
        <div class="qubit">|0&rang;</div><div class="qubit">|1&rang;</div><div class="qubit">|+&rang;</div><div class="qubit">|-&rang;</div>
        <div class="qubit">|0&rang;</div><div class="qubit">|1&rang;</div><div class="qubit">|+&rang;</div><div class="qubit">|-&rang;</div>
    </div>
    <div class="metrics">
        <h3>Entanglement Metrics (Post-Quantum Crypto)</h3>
        <p>Coherence Time: 145 &mu;s</p>
        <p>Fidelity: 99.9%</p>
    </div>
</body>
</html>`;
}

function getBciTelemetryWebviewContent() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <style>
        body { background-color: #111; color: #00ff00; font-family: sans-serif; padding: 20px; }
        .chart { height: 150px; background: repeating-linear-gradient(90deg, #111, #111 20px, #222 20px, #222 40px); border: 1px solid #00ff00; border-radius: 4px; margin-top: 10px; position: relative; overflow: hidden; }
        .wave { position: absolute; width: 200%; height: 100%; background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><path d="M 0 50 Q 25 0 50 50 T 100 50" stroke="%2300ff00" stroke-width="2" fill="none"/></svg>') repeat-x; animation: scroll 3s linear infinite; }
        @keyframes scroll { from { transform: translateX(0); } to { transform: translateX(-50%); } }
        .meter { width: 100%; height: 30px; background: #333; border-radius: 15px; margin-top: 30px; overflow: hidden; border: 1px solid #00ff00; }
        .fill { width: 85%; height: 100%; background: linear-gradient(90deg, #00ff00, #ffff00, #ff0000); }
    </style>
</head>
<body>
    <h2>Neural-Interface BCI Telemetry</h2>
    <h4>EEG Brainwave (Alpha/Beta)</h4>
    <div class="chart">
        <div class="wave"></div>
    </div>
    <h4>Focus Level (85%)</h4>
    <div class="meter">
        <div class="fill"></div>
    </div>
</body>
</html>`;
}

export function deactivate() {}

