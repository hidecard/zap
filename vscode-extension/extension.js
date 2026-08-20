const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

const diagnosticCollection = vscode.languages.createDiagnosticCollection('zap');
let diagnosticTimer;

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
    provideCompletionItems() {
      const items = [];
      for (const word of KEYWORDS) items.push(item(word, vscode.CompletionItemKind.Keyword));
      for (const word of TYPES) items.push(item(word, vscode.CompletionItemKind.TypeParameter));
      for (const word of BUILTINS) items.push(item(word, vscode.CompletionItemKind.Function));
      return items;
    }
  }, '.', ':'));
  context.subscriptions.push(vscode.workspace.onDidChangeTextDocument(event => {
    if (event.document.languageId === 'zap') scheduleDiagnostics(event.document);
  }));
  context.subscriptions.push(vscode.workspace.onDidOpenTextDocument(document => {
    if (document.languageId === 'zap') refreshDiagnostics(document);
  }));
  context.subscriptions.push(vscode.workspace.onDidCloseTextDocument(document => diagnosticCollection.delete(document.uri)));
  if (vscode.window.activeTextEditor?.document.languageId === 'zap') refreshDiagnostics(vscode.window.activeTextEditor.document);
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

module.exports = { activate, deactivate, parseJsonDiagnostic };
