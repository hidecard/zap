const cp = require('child_process');
const path = require('path');
const vscode = require('vscode');

class ZapLspClient {
  constructor(command, cwd, onDiagnostics, onExit) {
    this.command = command;
    this.cwd = cwd;
    this.onDiagnostics = onDiagnostics;
    this.onExit = onExit;
    this.nextId = 1;
    this.pending = new Map();
    this.buffer = Buffer.alloc(0);
    this.started = false;
    this.process = cp.spawn(command, ['lsp'], { cwd, windowsHide: true, stdio: ['pipe', 'pipe', 'pipe'] });
    this.process.stdout.on('data', data => this.read(data));
    this.process.stderr.on('data', data => { this.lastStderr = `${this.lastStderr || ''}${data.toString()}`; });
    this.process.on('exit', code => {
      this.started = false;
      for (const pending of this.pending.values()) pending.reject(new Error(`Zap LSP exited with code ${code}`));
      this.pending.clear();
      if (this.onExit) this.onExit(code, this.lastStderr || '');
    });
    this.initialize().catch(() => {});
  }

  initialize() {
    return this.request('initialize', {
      processId: process.pid,
      rootUri: vscode.Uri.file(this.cwd).toString(),
      capabilities: {
        textDocument: {
          completion: { completionItem: { snippetSupport: false } },
          hover: {},
          definition: {},
          publishDiagnostics: {}
        },
        workspace: { symbol: {} }
      }
    }).then(result => {
      this.started = true;
      this.notify('initialized', {});
      return result;
    });
  }

  read(data) {
    this.buffer = Buffer.concat([this.buffer, data]);
    while (true) {
      const separator = this.buffer.indexOf('\r\n\r\n');
      if (separator < 0) return;
      const header = this.buffer.subarray(0, separator).toString('ascii');
      const match = /Content-Length:\s*(\d+)/i.exec(header);
      if (!match) { this.buffer = this.buffer.subarray(separator + 4); continue; }
      const length = Number(match[1]);
      const start = separator + 4;
      if (this.buffer.length < start + length) return;
      const body = this.buffer.subarray(start, start + length).toString('utf8');
      this.buffer = this.buffer.subarray(start + length);
      try { this.receive(JSON.parse(body)); } catch (_) { /* Ignore malformed server frames. */ }
    }
  }

  receive(message) {
    if (message.id !== undefined && this.pending.has(message.id)) {
      const pending = this.pending.get(message.id);
      this.pending.delete(message.id);
      if (message.error) pending.reject(new Error(message.error.message || 'Zap LSP request failed'));
      else pending.resolve(message.result);
      return;
    }
    if (message.method === 'textDocument/publishDiagnostics' && this.onDiagnostics) {
      this.onDiagnostics(message.params || {});
    }
  }

  request(method, params) {
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.send({ jsonrpc: '2.0', id, method, params });
    });
  }

  notify(method, params) {
    this.send({ jsonrpc: '2.0', method, params });
  }

  send(message) {
    if (!this.process || this.process.killed || !this.process.stdin.writable) return;
    const body = Buffer.from(JSON.stringify(message), 'utf8');
    this.process.stdin.write(`Content-Length: ${body.length}\r\n\r\n`);
    this.process.stdin.write(body);
  }

  open(document) {
    this.notify('textDocument/didOpen', {
      textDocument: { uri: document.uri.toString(), languageId: 'zap', version: document.version, text: document.getText() }
    });
  }

  change(document) {
    this.notify('textDocument/didChange', {
      textDocument: { uri: document.uri.toString(), version: document.version },
      contentChanges: [{ text: document.getText() }]
    });
  }

  close(document) {
    this.notify('textDocument/didClose', { textDocument: { uri: document.uri.toString() } });
  }

  stop() {
    if (this.process && !this.process.killed) {
      this.notify('shutdown', null);
      this.notify('exit', null);
      this.process.kill();
    }
  }
}

module.exports = { ZapLspClient };
