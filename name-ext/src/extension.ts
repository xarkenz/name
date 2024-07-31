// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
'use strict';
import * as vscode from 'vscode';
import * as Net from 'net';
import { activateNameDebug } from './activateNameDebug';
import * as path from 'path';
import * as window from 'window'; 

const termName = "NAME Emulator";

const runMode: 'external' | 'server' | 'namedPipeServer' | 'inline' = 'server';

// TODO: Allow this code to run on Windows, Linux, and macOS.
// The current issue is that the paths are made with linux in mind.
// There exist libraries which would resolve this. There are also known techniques specific to vscode. 
// Should not take much looking.
export function activate(context: vscode.ExtensionContext) {
	// figure out what operating system the user is using
	var OSName = 'Unknown';
	if (window.navigator.userAgent.indexOf('Win') != -1) OSName = 'Windows';
	if (window.navigator.userAgent.indexOf('Mac') != -1) OSName = 'MacOSName';
	if (window.navigator.userAgent.indexOf('X11') != -1) OSName = 'UNIX';
	if (window.navigator.userAgent.indexOf('Linux') != -1) OSName = 'Linux';
	console.log(OSName);
	console.log(navigator.userAgent);

	context.subscriptions.push(
		vscode.commands.registerCommand("extension.vsname.startEmu", () => {
			// User configuration
			var configuration = vscode.workspace.getConfiguration('name-ext');
			if (!configuration) {
				vscode.window.showErrorMessage("Failed to find NAME configurations");
				return;
			}
			
			const namePath = configuration.get('namePath', '');
			if (namePath.length < 1) {
				vscode.window.showErrorMessage(`Failed to find a path for NAME, please set the path in VSCode's User Settings under name-ext`);
				return;
			}

			const nameASPath = path.join(namePath, 'name-as');
			const nameDefaultCfgPath = path.join(nameASPath, 'configs' + path.sep + 'default.toml');
			const nameEMUPath = path.join(namePath, 'name-emu');
			const nameEXTPath = path.join(namePath, 'name-ext');
			console.log(nameEXTPath);

			var editor = vscode.window.activeTextEditor;			
			if (editor) {
				// Get currently-open file path
				var currentlyOpenTabFilePath = editor.document.fileName;
				var currentlyOpenTabFileName = path.basename(currentlyOpenTabFilePath);
				if (!vscode.workspace.workspaceFolders) {
					vscode.window.showInformationMessage("Open a folder/workspace first");
					return;
				}
				else {
					var currentlyOpenDirectory = vscode.workspace.workspaceFolders[0].uri.fsPath;
				}

				const terminalOptions = { name: termName, closeOnExit: true };
				var terminal = vscode.window.terminals.find(terminal => terminal.name === termName);
				terminal = terminal ? terminal : vscode.window.createTerminal(terminalOptions);
				terminal.show();

				// TODO: Create a bin/ dir which contains the compiled binaries for each OS
				// TODO: check if bash is on MacOS by default :3
				if (OSName === "Linux" || OSName === "UNIX" || OSName === "MacOS")  {
					// Build and run assembler
					terminal.sendText(`cd ${nameASPath}`);
					terminal.sendText(`cargo build --release`);
					terminal.sendText(`cargo run ${nameDefaultCfgPath} ${currentlyOpenTabFilePath} ${currentlyOpenDirectory}/${currentlyOpenTabFileName}.o --lineinfo`);
					
					// Build and run emulator
					terminal.sendText(`cd ${nameEMUPath}`);
					terminal.sendText('cargo build --release');
					terminal.sendText(`cargo run ${currentlyOpenTabFilePath} ${currentlyOpenDirectory}/${currentlyOpenTabFileName}.o ${currentlyOpenDirectory}/${currentlyOpenTabFileName}.o.li`);
				} else if (OSName === "Windows") { // this code 100% doesn't work. will fix when node.js and rust and etc. decide to not be doo doo fart face
					// FIXME: windows SUCKS (i'm the problem)
					// Build and run assembler
					terminal.sendText(`cd %nameASPath%`);
					terminal.sendText(`cargo build --release`);
					terminal.sendText(`cargo run %nameDefaultCfgPath% %currentlyOpenTabFilePath% %currentlyOpenDirectory%\\%currentlyOpenTabFileName%.o --lineinfo`);
					
					// Build and run emulator
					terminal.sendText(`cd %nameEMUPath%`);
					terminal.sendText('cargo build --release');
					terminal.sendText(`cargo run %currentlyOpenTabFilePath% %currentlyOpenDirectory%\\%currentlyOpenTabFileName%.o %currentlyOpenDirectory%\\%currentlyOpenTabFileName%.o.li`);
				}
			}
		})
	);
	context.subscriptions.push(
		vscode.commands.registerCommand("extension.vsname.startAndDebug", () => {
			// User configuration
			var configuration = vscode.workspace.getConfiguration('name-ext');
			if (!configuration) {
				vscode.window.showErrorMessage("Failed to find NAME configurations");
				return;
			}
			
			const namePath = configuration.get('namePath', '');
			if (namePath.length < 1) {
				vscode.window.showErrorMessage(`Failed to find a path for NAME, please set the path in VSCode's User Settings under name-ext`);
				return;
			}

			const nameASPath = path.join(namePath, 'name-as');
			const nameDefaultCfgPath = path.join(nameASPath, 'configs' + path.sep + 'default.toml');
			const nameEMUPath = path.join(namePath, 'name-emu');
			const nameEXTPath = path.join(namePath, 'name-ext');
			console.log(nameEXTPath);

			var editor = vscode.window.activeTextEditor;			
			if (editor) {
				// Get currently-open file path
				var currentlyOpenTabFilePath = editor.document.fileName;
				var currentlyOpenTabFileName = path.basename(currentlyOpenTabFilePath);
				if (!vscode.workspace.workspaceFolders) {
					vscode.window.showInformationMessage("Open a folder/workspace first");
					return;
				}
				else {
					var currentlyOpenDirectory = vscode.workspace.workspaceFolders[0].uri.fsPath;
				}

				const terminalOptions = { name: termName, closeOnExit: true };
				var terminal = vscode.window.terminals.find(terminal => terminal.name === termName);
				terminal = terminal ? terminal : vscode.window.createTerminal(terminalOptions);
				terminal.show();

				// TODO: Create a bin/ dir which contains the compiled binaries for each OS
				if (OSName === 'Linux' || OSName === 'UNIX' || OSName === 'MacOS'){
					// Build and run assembler
					
					terminal.sendText(`cd ${nameASPath}`);
					terminal.sendText(`cargo build --release`);
					terminal.sendText(`cargo run ${nameDefaultCfgPath} ${currentlyOpenTabFilePath} ${path.join(currentlyOpenDirectory, currentlyOpenTabFileName)}.o --lineinfo`);
					
					// Build and run emulator
					terminal.sendText(`cd ${nameEMUPath}`);
					terminal.sendText('cargo build --release');
					terminal.sendText(`cargo run ${currentlyOpenTabFilePath} ${path.join(currentlyOpenDirectory, currentlyOpenTabFileName)}.o ${path.join(currentlyOpenDirectory, currentlyOpenTabFileName)}.o.li --debug`);
				} else if (OSName === 'Windows') { // hello curious student. this is your sign to switch to linux =)
					// Build and run assembler

					terminal.sendText(`cd %nameASPath%`);
					terminal.sendText(`cargo build --release`);
					terminal.sendText(`cargo run %nameDefaultCfgPath% %currentlyOpenTabFilePath% %{path.join(currentlyOpenDirectory, currentlyOpenTabFileName)}%.o --lineinfo`);

					// Build and run emulator
					terminal.sendText(`cd %nameEMUPath%`);
					terminal.sendText('cargo build --release');
					terminal.sendText(`cargo run %currentlyOpenTabFilePath% %path.join(currentlyOpenDirectory, currentlyOpenTabFileName)}.o %path.join(currentlyOpenDirectory, currentlyOpenTabFileName)%.o.li --debug`);
				}
				setTimeout(() => {
					vscode.commands.executeCommand('workbench.action.debug.start');
				}, 6000);
			
			}
		})
	);

	// debug adapters can be run in different ways by using a vscode.DebugAdapterDescriptorFactory:
	switch (runMode) {
		case 'server':
			// run the debug adapter as a server inside the extension and communicate via a socket
			activateNameDebug(context, new NameDebugAdapterServerDescriptorFactory());
			break;

		case 'external': default:
			// run the debug adapter as a separate process
			//activateNameDebug(context, new DebugAdapterExecutableFactory());
			break;

		case 'inline':
			// run the debug adapter inside the extension and directly talk to it
			activateNameDebug(context);
			break;
	}

}

// This method is called when your extension is deactivated
export function deactivate() {}

class NameDebugAdapterServerDescriptorFactory implements vscode.DebugAdapterDescriptorFactory {

	private server?: Net.Server;

	createDebugAdapterDescriptor(session: vscode.DebugSession, executable: vscode.DebugAdapterExecutable | undefined): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {

		// make VS Code connect to debug server
		return new vscode.DebugAdapterServer(63321);
	}

	dispose() {
		if (this.server) {
			this.server.close();
		}
	}
}
