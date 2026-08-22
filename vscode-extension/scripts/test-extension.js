const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');
const repositoryRoot = path.resolve(root, '..');
const required = [
  'package.json',
  'language-configuration.json',
  'extension.js',
  'lsp-client.js',
  'README.md',
  'README_MM.md',
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
const manifest = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
const grammar = JSON.parse(fs.readFileSync(path.join(root, 'syntaxes/zap.tmLanguage.json'), 'utf8'));
const cargo = fs.readFileSync(path.join(repositoryRoot, 'native', 'Cargo.toml'), 'utf8');
const versionMatch = cargo.match(/^version\s*=\s*"([^"]+)"\s*$/m);
if (!versionMatch || manifest.version !== versionMatch[1]) {
  throw new Error('canonical extension version does not match native Cargo version');
}
if (manifest.name !== 'zap-language-support' || manifest.publisher !== 'ArkarYan' || manifest.main !== 'extension.js') {
  throw new Error('canonical extension package metadata is incomplete');
}
const catalog = fs.readFileSync(path.join(repositoryRoot, 'native', 'src', 'stdlib_catalog.rs'), 'utf8');
const builtins = [...catalog.matchAll(/stable_builtin!\("([^"]+)"/g)].map((match) => match[1]);
const grammarText = JSON.stringify(grammar);
for (const builtin of builtins) {
  if (!new RegExp(`(?<![A-Za-z0-9_])${builtin}(?![A-Za-z0-9_])`).test(grammarText)) {
    throw new Error(`catalog builtin is missing from canonical grammar: ${builtin}`);
  }
}
const extensionSource = fs.readFileSync(path.join(root, 'extension.js'), 'utf8');
const lspSource = fs.readFileSync(path.join(root, 'lsp-client.js'), 'utf8');
if (!lspSource.includes('Content-Length') || !lspSource.includes("request(method")) {
  throw new Error('LSP client framing or request transport is missing');
}
if (!lspSource.includes('contentChanges')) {
  throw new Error('LSP client does not send standard contentChanges');
}
for (const requirement of [
  'textDocument/definition',
  'textDocument/hover',
  'textDocument/rename',
  'textDocument/signatureHelp',
  'textDocument/formatting'
]) {
  if (!extensionSource.includes(requirement)) throw new Error(`extension is missing ${requirement}`);
}
if (!extensionSource.includes('registerSignatureHelpProvider') ||
    !extensionSource.includes('registerDocumentFormattingEditProvider')) {
  throw new Error('extension LSP providers are not registered');
}
if (!extensionSource.includes('zap.runFile') || !extensionSource.includes('zap.checkWorkspace')) {
  throw new Error('extension commands are not registered');
}
console.log(`Zap VS Code extension validation passed for v${manifest.version}; ${builtins.length} catalog builtins covered.`);
