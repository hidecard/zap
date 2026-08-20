# Zap Language Support for VS Code

This extension adds first-class `.zp` editing support for the Zap programming language. It is intentionally dependency-light and uses the Zap CLI already installed on the developer machine for workspace validation and execution.

## Features

| Feature | Description |
|---|---|
| Language registration | Recognizes `.zp` files as Zap source files. |
| Syntax highlighting | Highlights declarations, keywords, types, literals, comments, operators, and standard-library calls. |
| Completion | Offers Zap keywords, types, and built-in functions. |
| Snippets | Includes functions, loops, conditionals, `try`/`catch`, imports, `main`, and `raise`. |
| Diagnostics | Runs `zap check --json` and maps stable Zap diagnostics into the Problems panel. |
| Run current file | Runs `zap run <file.zp>` in the integrated terminal or Output panel. |
| Workspace check | Runs `zap check --json <workspace>` on demand. |

## Installation from the repository

Open this folder in VS Code and choose **Extensions: Install from VSIX...** after creating a package with the repository script. For development, use **Developer: Install Extension from Location...** and select the `vscode-extension` directory.

The extension requires the Zap CLI to be available as `zap` on `PATH`, or configured explicitly through `zap.executable`.

## Commands

The Command Palette provides **Zap: Run Current File**, **Zap: Check Workspace**, and **Zap: Restart Diagnostics**. A play button and context-menu entry are also shown when a `.zp` editor is active.

## Settings

```json
{
  "zap.executable": "zap",
  "zap.enableDiagnostics": true,
  "zap.diagnosticDelay": 350,
  "zap.runInTerminal": true
}
```

Diagnostics are intentionally bounded by the native CLI and are refreshed after edits with a short debounce. The extension does not reimplement the Zap parser; it consumes the CLI's stable JSON diagnostic boundary, which keeps editor behavior aligned with command-line behavior.

## Development

Run `node scripts/test-extension.js` to validate the manifest, grammar, snippets, and extension JavaScript. Run `node scripts/package-extension.js` to create a repository-local `.vsix`-compatible zip archive under `dist/`.
