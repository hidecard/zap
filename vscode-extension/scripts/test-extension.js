const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');
const required = [
  'package.json',
  'language-configuration.json',
  'extension.js',
  'lsp-client.js',
  'syntaxes/zap.tmLanguage.json',
  'snippets/zap.json'
];
for (const relative of required) {
  const file = path.join(root, relative);
  if (!fs.existsSync(file)) throw new Error(`missing extension file: ${relative}`);
}
for (const relative of ['package.json', 'language-configuration.json', 'syntaxes/zap.tmLanguage.json', 'snippets/zap.json']) {
  JSON.parse(fs.readFileSync(path.join(root, relative), 'utf8'));
}
const extensionSource = fs.readFileSync(path.join(root, 'extension.js'), 'utf8');
const lspSource = fs.readFileSync(path.join(root, 'lsp-client.js'), 'utf8');
const grammar = JSON.parse(fs.readFileSync(path.join(root, 'syntaxes/zap.tmLanguage.json'), 'utf8'));
const builtinPattern = grammar.repository.builtins.patterns[0].match;
for (const builtin of ['spawn', 'task_join', 'task_is_ready']) {
  if (!builtinPattern.includes(builtin)) throw new Error(`async builtin is missing from syntax grammar: ${builtin}`);
}
if (!lspSource.includes('Content-Length') || !lspSource.includes("request(method")) {
  throw new Error('LSP client framing or request transport is missing');
}
if (!extensionSource.includes('textDocument/definition') || !extensionSource.includes('textDocument/hover')) {
  throw new Error('LSP definition or hover provider is missing');
}
if (!extensionSource.includes('textDocument/signatureHelp') || !extensionSource.includes('registerSignatureHelpProvider')) {
  throw new Error('LSP signature-help provider is missing');
}
if (!extensionSource.includes('textDocument/formatting') || !extensionSource.includes('registerDocumentFormattingEditProvider')) {
  throw new Error('LSP formatting provider is missing');
}
if (!extensionSource.includes('zap.runFile') || !extensionSource.includes('zap.checkWorkspace')) {
  throw new Error('extension commands are not registered');
}
console.log('Zap VS Code extension validation passed.');
