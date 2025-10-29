import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const serverPath = path.join(context.extensionPath, '..', 'rust-server', 'target', 'release', 'korean-language-server');
  
  const serverOptions: ServerOptions = {
    run: { command: serverPath },
    debug: { command: serverPath }
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'korean' }]
  };

  client = new LanguageClient(
    'koreanLanguageServer',
    '한국어 Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
