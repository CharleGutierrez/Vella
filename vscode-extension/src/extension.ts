import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    let syncSdkDisposable = vscode.commands.registerCommand('vella.syncSdk', () => {
        vscode.window.showInformationMessage('Vella: Synchronizing React/Vue SDKs from backend...');
    });

    let generateWalletDisposable = vscode.commands.registerCommand('vella.generateWallet', () => {
        vscode.window.showInformationMessage('Vella: Generating new ECDSA Web3 Wallet...');
    });

    context.subscriptions.push(syncSdkDisposable, generateWalletDisposable);
}

export function deactivate() {}
