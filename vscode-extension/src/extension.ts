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

    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(rocket) Vella Server';
    statusBarItem.show();

    context.subscriptions.push(syncSdkDisposable, generateWalletDisposable, statusBarItem);
}

export function deactivate() {}
