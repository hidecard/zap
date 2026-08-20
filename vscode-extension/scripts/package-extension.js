const fs = require('fs');
const path = require('path');
const cp = require('child_process');

const root = path.resolve(__dirname, '..');
const packageJson = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
const dist = path.join(root, 'dist');
fs.mkdirSync(dist, { recursive: true });
const archive = path.join(dist, `${packageJson.name}-${packageJson.version}.vsix`);
if (fs.existsSync(archive)) fs.unlinkSync(archive);
const files = [
  'package.json',
  'language-configuration.json',
  'extension.js',
  'README.md',
  'README_MM.md',
  'syntaxes',
  'snippets'
];
cp.execFileSync('zip', ['-q', '-r', archive, ...files], { cwd: root, stdio: 'inherit' });
console.log(`Created ${archive}`);
