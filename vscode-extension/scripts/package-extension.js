const fs = require('fs');
const path = require('path');
const cp = require('child_process');

const root = path.resolve(__dirname, '..');
const packageJson = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
const dist = path.join(root, 'dist');
fs.mkdirSync(dist, { recursive: true });
for (const entry of fs.readdirSync(dist)) {
  if (entry.startsWith(`${packageJson.name}-`) && entry.endsWith('.vsix')) fs.unlinkSync(path.join(dist, entry));
}
const archive = path.join(dist, `${packageJson.name}-${packageJson.version}.vsix`);
const files = [
  'package.json',
  'language-configuration.json',
  'extension.js',
  'lsp-client.js',
  'README.md',
  'README_MM.md',
  'syntaxes',
  'snippets'
];
cp.execFileSync('zip', ['-q', '-r', archive, ...files], { cwd: root, stdio: 'inherit' });
console.log(`Created ${archive}`);
