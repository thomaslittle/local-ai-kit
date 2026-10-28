#!/usr/bin/env node
/**
 * Build the per-platform npm packages for one target.
 *
 *   node scripts/npm-platform-packages.mjs \
 *     --binary target/release/local-ai-kit \
 *     --target x86_64-unknown-linux-gnu \
 *     --out npm-out
 *
 * Each package contains exactly one binary and a manifest with `os`/`cpu` and a
 * `bin` entry. The SDK resolves `@local-ai-kit/cli-<platform>/package.json` and
 * the binary next to it (esbuild/Biome shape), so no postinstall script or
 * download is involved. The release workflow adds these names to the SDK's
 * `optionalDependencies` before publishing; the workspace copy stays free of
 * platform-specific dependencies so `npm ci` is identical everywhere.
 */
import {copyFileSync, mkdirSync, readFileSync, statSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';

const TARGETS = {
  'x86_64-unknown-linux-gnu': {name: 'linux-x64', os: 'linux', cpu: 'x64'},
  'aarch64-unknown-linux-gnu': {name: 'linux-arm64', os: 'linux', cpu: 'arm64'},
  'x86_64-apple-darwin': {name: 'darwin-x64', os: 'darwin', cpu: 'x64'},
  'aarch64-apple-darwin': {name: 'darwin-arm64', os: 'darwin', cpu: 'arm64'},
  'x86_64-pc-windows-msvc': {name: 'win32-x64', os: 'win32', cpu: 'x64'},
};

function argument(name) {
  const index = process.argv.indexOf(`--${name}`);
  if (index === -1 || !process.argv[index + 1]) {
    console.error(`Missing --${name}`);
    process.exit(2);
  }
  return process.argv[index + 1];
}

const target = argument('target');
const binary = argument('binary');
const out = argument('out');
const spec = TARGETS[target];
if (!spec) {
  console.error(`Unsupported target: ${target}`);
  process.exit(2);
}
if (!statSync(binary).isFile()) {
  console.error(`Binary not found: ${binary}`);
  process.exit(2);
}
const version = JSON.parse(
  readFileSync(new URL('../packages/sdk-typescript/package.json', import.meta.url), 'utf8'),
).version;
const directory = join(out, `cli-${spec.name}`);
mkdirSync(join(directory, 'bin'), {recursive: true});
const executable = spec.os === 'win32' ? 'local-ai-kit.exe' : 'local-ai-kit';
copyFileSync(binary, join(directory, 'bin', executable));
const manifest = {
  name: `@local-ai-kit/cli-${spec.name}`,
  version,
  description: `Local AI Kit helper for ${spec.os}/${spec.cpu}`,
  os: [spec.os],
  cpu: [spec.cpu],
  bin: { 'local-ai-kit': `bin/${executable}` },
  files: ['bin', 'README.md'],
  license: 'MIT',
  repository: 'https://github.com/thomaslittle/local-ai-kit',
};
writeFileSync(join(directory, 'package.json'), `${JSON.stringify(manifest, null, 2)}\n`);
writeFileSync(
  join(directory, 'README.md'),
  `# @local-ai-kit/cli-${spec.name}\n\nThe Local AI Kit helper for ${spec.os}/${spec.cpu}. ` +
    `Installed automatically as an optional dependency of \`@local-ai-kit/sdk\`; ` +
    `it contains one binary and runs no install scripts.\n`,
);
console.log(join(directory, 'package.json'));
