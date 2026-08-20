# Zap Language Support for VS Code

This extension adds first-class `.zp` editing support for the Zap programming language. It is intentionally dependency-light and uses the Zap CLI already installed on the developer machine for workspace validation and execution.

## Features

| Feature | Description |
|---|---|
| Language registration | Recognizes `.zp` files as Zap source files. |
| Syntax highlighting | Highlights declarations, keywords, types, literals, comments, operators, and standard-library calls. |
| Completion | Uses the Zap LSP for workspace-aware symbols and falls back to built-in keywords, types, and functions. |
| Signature help | Shows function parameter labels and the active argument while typing a call. |
| Formatting | Formats a Zap document through the LSP using normalized newlines, four-space indentation, and trimmed trailing whitespace. |
| Hover | Shows LSP documentation for functions, classes, modules, imports, and bindings. |
| Go to definition | Resolves top-level Zap declarations through the LSP definition provider. |
| Workspace symbols | Makes Zap declarations searchable through VS Code symbol search. |
| Snippets | Includes functions, loops, conditionals, `try`/`catch`, imports, `main`, and `raise`. |
| Diagnostics | Receives `textDocument/publishDiagnostics` from the Zap LSP and retains CLI JSON checking as a fallback. |
| Run current file | Runs `zap run <file.zp>` in the integrated terminal or Output panel. |
| Workspace check | Runs `zap check --json <workspace>` on demand. |

## Installation from the repository

Open this folder in VS Code and choose **Extensions: Install from VSIX...** after creating a package with the repository script. For development, use **Developer: Install Extension from Location...** and select the `vscode-extension` directory.

The extension requires the Zap CLI to be available as `zap` on `PATH`, or configured explicitly through `zap.executable`. The extension launches `zap lsp` as a stdio JSON-RPC server and synchronizes open and changed `.zp` documents with it.

## Commands

The Command Palette provides **Zap: Run Current File**, **Zap: Check Workspace**, and **Zap: Restart Diagnostics**. A play button and context-menu entry are also shown when a `.zp` editor is active.

## Settings

```json
{
  "zap.executable": "zap",
  "zap.enableLsp": true,
  "zap.enableDiagnostics": true,
  "zap.diagnosticDelay": 350,
  "zap.runInTerminal": true
}
```

The LSP client uses standard `Content-Length` JSON-RPC framing and supports initialize, document open/change/close, completion, signature help, hover, definition, document formatting, workspace symbols, and publish-diagnostics notifications. Signature help is triggered after `(` and `,`; document formatting is available through **Format Document** and **Format Selection** where supported by VS Code. Diagnostics are intentionally bounded by the native LSP and CLI and are refreshed after edits with a short debounce. The extension does not reimplement the Zap parser; it consumes the CLI's stable JSON diagnostic boundary, which keeps editor behavior aligned with command-line behavior.

## Development

Run `node scripts/test-extension.js` to validate the manifest, grammar, snippets, and extension JavaScript. Run `node scripts/package-extension.js` to create a repository-local `.vsix`-compatible zip archive under `dist/`.
