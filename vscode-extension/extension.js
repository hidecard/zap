const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');
const { ZapLspClient } = require('./lsp-client');

const diagnosticCollection = vscode.languages.createDiagnosticCollection('zap');
let diagnosticTimer;
let lspClient;

const KEYWORDS = [
  'and', 'as', 'async', 'await', 'break', 'catch', 'case', 'class', 'const',
  'continue', 'defer', 'elif', 'else', 'enum', 'for', 'fn', 'if', 'import',
  'in', 'let', 'match', 'module', 'mut', 'not', 'or', 'pass', 'private',
  'protected', 'pub', 'raise', 'return', 'self', 'static', 'struct', 'try',
  'var', 'while'
];
const TYPES = ['any', 'bool', 'future', 'list', 'map', 'none', 'number', 'option', 'result', 'set', 'text', 'unknown'];
const BUILTINS = [
  'say', 'print', 'len', 'type', 'range', 'enumerate', 'zip', 'map', 'filter', 'reduce',
  'now', 'sleep', 'has_env', 'env', 'path_join', 'basename', 'dirname', 'json_parse',
  'json_stringify', 'url_parse', 'url_encode', 'url_decode', 'http_get', 'http_request',
  'process_run'
];

function executable() {
  return vscode.workspace.getConfiguration('zap').get('executable', 'zap');
}

function workspaceRoot() {
  const folder = vscode.workspace.getWorkspaceFolder(vscode.window.activeTextEditor?.document.uri);
  return folder ? folder.uri.fsPath : vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

function runCli(args, cwd, callback) {
  cp.execFile(executable(), args, { cwd, windowsHide: true, maxBuffer: 4 * 1024 * 1024 }, callback);
}

function parseJsonDiagnostic(stdout, stderr, cwd) {
  let value;
  try {
    value = JSON.parse(stdout.trim());
  } catch (_) {
    return [{
      message: (stderr || stdout || 'Zap check failed').trim(),
      file: undefined,
      line: 1,
      column: 1,
      kind: 'Project'
    }];
  }
  if (!value || value.ok) return [];
  return [{
    message: value.message || value.error || 'Zap check failed',
    file: value.file ? path.resolve(cwd, value.file) : undefined,
    line: Number(value.line) || 1,
    column: Number(value.column) || 1,
    kind: value.kind || 'Error'
  }];
}

function refreshDiagnostics(document) {
  const config = vscode.workspace.getConfiguration('zap');
  if (!config.get('enableDiagnostics', true) || document.languageId !== 'zap') return;
  const root = workspaceRoot() || path.dirname(document.uri.fsPath);
  runCli(['check', '--json', root], root, (error, stdout, stderr) => {
    if (document.isClosed) return;
    const items = parseJsonDiagnostic(stdout, stderr, root)
      .filter(item => !item.file || path.resolve(item.file) === path.resolve(document.uri.fsPath));
    const diagnostics = items.map(item => {
      const line = Math.max(0, item.line - 1);
      const column = Math.max(0, item.column - 1);
      const range = new vscode.Range(line, column, line, column + 1);
      const severity = item.kind === 'Project' ? vscode.DiagnosticSeverity.Warning : vscode.DiagnosticSeverity.Error;
      const diagnostic = new vscode.Diagnostic(range, item.message, severity);
      diagnostic.source = 'zap';
      diagnostic.code = item.kind;
      return diagnostic;
    });
    diagnosticCollection.set(document.uri, diagnostics);
  });
}

function scheduleDiagnostics(document) {
  clearTimeout(diagnosticTimer);
  diagnosticTimer = setTimeout(() => refreshDiagnostics(document), vscode.workspace.getConfiguration('zap').get('diagnosticDelay', 350));
}

function runCurrentFile() {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'zap') {
    vscode.window.showWarningMessage('Open a .zp file before running Zap.');
    return;
  }
  const file = editor.document.uri.fsPath;
  const cwd = path.dirname(file);
  if (vscode.workspace.getConfiguration('zap').get('runInTerminal', true)) {
    const terminal = vscode.window.createTerminal({ name: 'Zap', cwd });
    terminal.show(true);
    terminal.sendText(`${quote(executable())} run ${quote(file)}`);
  } else {
    runCli(['run', file], cwd, (error, stdout, stderr) => {
      const channel = vscode.window.createOutputChannel('Zap');
      channel.clear();
      channel.append(stdout || stderr || 'Zap finished.');
      channel.show(true);
      if (error) vscode.window.showErrorMessage(`Zap exited with code ${error.code ?? 'unknown'}.`);
    });
  }
}

function quote(value) {
  if (process.platform === 'win32') return `"${String(value).replace(/"/g, '\\"')}"`;
  return `'${String(value).replace(/'/g, "'\\''")}'`;
}

function checkWorkspace() {
  const root = workspaceRoot();
  if (!root) {
    vscode.window.showInformationMessage('Open a Zap workspace before checking it.');
    return;
  }
  runCli(['check', '--json', root], root, (error, stdout, stderr) => {
    const items = parseJsonDiagnostic(stdout, stderr, root);
    if (!items.length) {
      diagnosticCollection.clear();
      vscode.window.showInformationMessage('Zap check passed.');
      return;
    }
    const editor = vscode.window.activeTextEditor;
    if (editor) refreshDiagnostics(editor.document);
    vscode.window.showErrorMessage(`Zap check found ${items.length} issue${items.length === 1 ? '' : 's'}.`);
  });
}

function lspDiagnostics(params) {
  if (!params.uri) return;
  const uri = vscode.Uri.parse(params.uri);
  const diagnostics = (params.diagnostics || []).map(item => {
    const start = item.range?.start || { line: 0, character: 0 };
    const end = item.range?.end || start;
    const diagnostic = new vscode.Diagnostic(
      new vscode.Range(start.line, start.character, end.line, end.character),
      item.message || 'Zap diagnostic',
      item.severity === 1 ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
    );
    diagnostic.source = item.source || 'zap-lsp';
    return diagnostic;
  });
  diagnosticCollection.set(uri, diagnostics);
}

function startLsp(context) {
  if (!vscode.workspace.getConfiguration('zap').get('enableLsp', true)) return;
  const root = workspaceRoot();
  if (!root) return;
  try {
    lspClient = new ZapLspClient(executable(), root, lspDiagnostics, (code, stderr) => {
      if (code && stderr) vscode.window.setStatusBarMessage(`Zap LSP stopped: ${stderr.trim().slice(0, 120)}`, 5000);
    });
    context.subscriptions.push({ dispose: () => lspClient && lspClient.stop() });
  } catch (error) {
    vscode.window.showWarningMessage(`Zap LSP could not start: ${error.message}`);
  }
}

function activate(context) {
  context.subscriptions.push(diagnosticCollection);
  context.subscriptions.push(vscode.commands.registerCommand('zap.runFile', runCurrentFile));
  context.subscriptions.push(vscode.commands.registerCommand('zap.checkWorkspace', checkWorkspace));
  context.subscriptions.push(vscode.commands.registerCommand('zap.restartDiagnostics', () => {
    diagnosticCollection.clear();
    const editor = vscode.window.activeTextEditor;
    if (editor) refreshDiagnostics(editor.document);
  }));
  context.subscriptions.push(vscode.languages.registerCompletionItemProvider('zap', {
    provideCompletionItems(document, position) {
      if (lspClient && lspClient.started) {
        return lspClient.request('textDocument/completion', {
          textDocument: { uri: document.uri.toString() },
          position: { line: position.line, character: position.character }
        }).then(result => (result?.items || []).map(value => {
          const completion = item(value.label, vscode.CompletionItemKind.Text);
          completion.detail = value.detail || 'Zap LSP';
          return completion;
        })).catch(() => localCompletions());
      }
      return localCompletions();
    }
  }, '.', ':'));
  context.subscriptions.push(vscode.languages.registerSignatureHelpProvider('zap', {
    provideSignatureHelp(document, position) {
      if (!lspClient || !lspClient.started) return undefined;
      return lspClient.request('textDocument/signatureHelp', {
        textDocument: { uri: document.uri.toString() },
        position: { line: position.line, character: position.character }
      }).then(result => {
        if (!result || !Array.isArray(result.signatures)) return undefined;
        const help = new vscode.SignatureHelp();
        help.activeSignature = result.activeSignature || 0;
        help.activeParameter = result.activeParameter || 0;
        help.signatures = result.signatures.map(signature => {
          const item = new vscode.SignatureInformation(signature.label || '');
          item.documentation = signature.documentation || '';
          item.parameters = (signature.parameters || []).map(parameter =>
            new vscode.ParameterInformation(parameter.label || String(parameter))
          );
          return item;
        });
        return help;
      }).catch(() => undefined);
    }
  }, '(', ','));
  context.subscriptions.push(vscode.languages.registerDocumentFormattingEditProvider('zap', {
    provideDocumentFormattingEdits(document) {
      if (!lspClient || !lspClient.started) return [];
      return lspClient.request('textDocument/formatting', {
        textDocument: { uri: document.uri.toString() },
        options: { tabSize: 4, insertSpaces: true }
      }).then(edits => (edits || []).map(edit => new vscode.TextEdit(
        new vscode.Range(
          edit.range.start.line,
          edit.range.start.character,
          edit.range.end.line,
          edit.range.end.character
        ),
        edit.newText || ''
      ))).catch(() => []);
    }
  }));
  context.subscriptions.push(vscode.languages.registerHoverProvider('zap', {
    provideHover(document, position) {
      if (!lspClient || !lspClient.started) return undefined;
      return lspClient.request('textDocument/hover', {
        textDocument: { uri: document.uri.toString() },
        position: { line: position.line, character: position.character }
      }).then(result => {
        if (!result || !result.contents) return undefined;
        const value = typeof result.contents === 'string' ? result.contents : result.contents.value;
        return value ? new vscode.Hover(new vscode.MarkdownString(value)) : undefined;
      }).catch(() => undefined);
    }
  }));
  context.subscriptions.push(vscode.languages.registerDefinitionProvider('zap', {
    provideDefinition(document, position) {
      if (!lspClient || !lspClient.started) return undefined;
      return lspClient.request('textDocument/definition', {
        textDocument: { uri: document.uri.toString() },
        position: { line: position.line, character: position.character }
      }).then(result => {
        if (!result) return undefined;
        const locations = Array.isArray(result) ? result : [result];
        return locations.map(location => new vscode.Location(
          vscode.Uri.parse(location.uri),
          new vscode.Range(
            location.range.start.line,
            location.range.start.character,
            location.range.end.line,
            location.range.end.character
          )
        ));
      }).catch(() => undefined);
    }
  }));
  context.subscriptions.push(vscode.languages.registerRenameProvider('zap', {
    provideRenameEdits(document, position, newName) {
      if (!lspClient || !lspClient.started) return undefined;
      return lspClient.request('textDocument/rename', {
        textDocument: { uri: document.uri.toString() },
        position: { line: position.line, character: position.character },
        newName
      }).then(result => {
        if (!result || !result.changes) return undefined;
        const workspaceEdit = new vscode.WorkspaceEdit();
        for (const [uri, edits] of Object.entries(result.changes)) {
          workspaceEdit.set(vscode.Uri.parse(uri), (edits || []).map(edit => new vscode.TextEdit(
            new vscode.Range(
              edit.range.start.line,
              edit.range.start.character,
              edit.range.end.line,
              edit.range.end.character
            ),
            edit.newText || ''
          )));
        }
        return workspaceEdit;
      }).catch(() => undefined);
    }
  }));
  context.subscriptions.push(vscode.languages.registerWorkspaceSymbolProvider('zap', {
    provideWorkspaceSymbols(query) {
      if (!lspClient || !lspClient.started) return [];
      return lspClient.request('workspace/symbol', { query }).then(result => (result || []).map(symbol => new vscode.SymbolInformation(
        symbol.name,
        symbol.kind || vscode.SymbolKind.Namespace,
        new vscode.Range(
          symbol.location.range.start.line,
          symbol.location.range.start.character,
          symbol.location.range.end.line,
          symbol.location.range.end.character
        ),
        vscode.Uri.parse(symbol.location.uri),
        symbol.containerName
      ))).catch(() => []);
    }
  }));
  context.subscriptions.push(vscode.workspace.onDidChangeTextDocument(event => {
    if (event.document.languageId === 'zap') {
      if (lspClient) lspClient.change(event.document);
      scheduleDiagnostics(event.document);
    }
  }));
  context.subscriptions.push(vscode.workspace.onDidOpenTextDocument(document => {
    if (document.languageId === 'zap') {
      if (lspClient) lspClient.open(document);
      refreshDiagnostics(document);
    }
  }));
  context.subscriptions.push(vscode.workspace.onDidCloseTextDocument(document => {
    if (lspClient && document.languageId === 'zap') lspClient.close(document);
    diagnosticCollection.delete(document.uri);
  }));
  startLsp(context);
  for (const document of vscode.workspace.textDocuments) {
    if (document.languageId === 'zap' && lspClient) lspClient.open(document);
  }
  if (vscode.window.activeTextEditor?.document.languageId === 'zap') refreshDiagnostics(vscode.window.activeTextEditor.document);
}

function localCompletions() {
  const items = [];
  for (const word of KEYWORDS) items.push(item(word, vscode.CompletionItemKind.Keyword));
  for (const word of TYPES) items.push(item(word, vscode.CompletionItemKind.TypeParameter));
  for (const word of BUILTINS) items.push(item(word, vscode.CompletionItemKind.Function));
  return items;
}

function item(label, kind) {
  const completion = new vscode.CompletionItem(label, kind);
  completion.detail = 'Zap language';
  return completion;
}

function deactivate() {
  clearTimeout(diagnosticTimer);
  diagnosticCollection.dispose();
}

module.exports = { activate, deactivate, parseJsonDiagnostic, lspDiagnostics };
